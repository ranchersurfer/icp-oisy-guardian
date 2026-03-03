'use client'

import { useState, useCallback } from 'react'
import {
  DndContext,
  DragEndEvent,
  DragOverEvent,
  PointerSensor,
  useSensor,
  useSensors,
  closestCorners,
} from '@dnd-kit/core'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { Task, TaskStatus } from '@/lib/tasks'
import KanbanColumn from './KanbanColumn'
import NewTaskModal from './NewTaskModal'
import { Button } from '@/components/ui/button'
import { Plus } from 'lucide-react'

const COLUMNS: TaskStatus[] = ['backlog', 'in_progress', 'in_review', 'done']

async function fetchTasks(): Promise<Task[]> {
  const res = await fetch('/api/tasks')
  if (!res.ok) throw new Error('Failed to fetch tasks')
  return res.json()
}

async function patchTask(id: string, patch: Partial<Task>): Promise<Task> {
  const res = await fetch(`/api/tasks/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(patch),
  })
  if (!res.ok) throw new Error('Failed to update task')
  return res.json()
}

export default function KanbanBoard() {
  const queryClient = useQueryClient()
  const [modalOpen, setModalOpen] = useState(false)

  const { data: tasks = [], isLoading } = useQuery({
    queryKey: ['tasks'],
    queryFn: fetchTasks,
    refetchInterval: 15000,
  })

  const mutation = useMutation({
    mutationFn: ({ id, patch }: { id: string; patch: Partial<Task> }) => patchTask(id, patch),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['tasks'] }),
  })

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 5 } })
  )

  function getTasksByStatus(status: TaskStatus): Task[] {
    return tasks.filter(t => t.status === status)
  }

  function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event
    if (!over) return

    const taskId = active.id as string
    const overId = over.id as string

    // If dropped over a column id
    if (COLUMNS.includes(overId as TaskStatus)) {
      const task = tasks.find(t => t.id === taskId)
      if (task && task.status !== overId) {
        mutation.mutate({ id: taskId, patch: { status: overId as TaskStatus } })
      }
      return
    }

    // If dropped over another task, find its column
    const overTask = tasks.find(t => t.id === overId)
    if (overTask && overTask.status) {
      const task = tasks.find(t => t.id === taskId)
      if (task && task.status !== overTask.status) {
        mutation.mutate({ id: taskId, patch: { status: overTask.status } })
      }
    }
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64 text-white/40 text-sm">
        Loading tasks...
      </div>
    )
  }

  return (
    <div className="flex flex-col h-full">
      {/* Board header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-white font-semibold text-lg">Task Board</h1>
          <p className="text-white/40 text-xs mt-0.5">{tasks.length} tasks across all columns</p>
        </div>
        <Button
          onClick={() => setModalOpen(true)}
          className="bg-indigo-600 hover:bg-indigo-500 text-white h-8 text-xs px-3 gap-1.5"
        >
          <Plus size={14} />
          New Task
        </Button>
      </div>

      {/* Kanban columns */}
      <DndContext sensors={sensors} collisionDetection={closestCorners} onDragEnd={handleDragEnd}>
        <div className="flex gap-5 overflow-x-auto pb-4">
          {COLUMNS.map(status => (
            <KanbanColumn key={status} status={status} tasks={getTasksByStatus(status)} />
          ))}
        </div>
      </DndContext>

      <NewTaskModal
        open={modalOpen}
        onClose={() => setModalOpen(false)}
        onCreated={() => queryClient.invalidateQueries({ queryKey: ['tasks'] })}
      />
    </div>
  )
}

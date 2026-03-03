'use client'

import { useDroppable } from '@dnd-kit/core'
import { SortableContext, verticalListSortingStrategy } from '@dnd-kit/sortable'
import { Task, TaskStatus } from '@/lib/tasks'
import TaskCard from './TaskCard'

const COLUMN_META: Record<TaskStatus, { label: string; color: string }> = {
  backlog: { label: 'Backlog', color: 'text-slate-400' },
  in_progress: { label: 'In Progress', color: 'text-yellow-400' },
  in_review: { label: 'In Review', color: 'text-blue-400' },
  done: { label: 'Done', color: 'text-emerald-400' },
}

interface KanbanColumnProps {
  status: TaskStatus
  tasks: Task[]
}

export default function KanbanColumn({ status, tasks }: KanbanColumnProps) {
  const { setNodeRef, isOver } = useDroppable({ id: status })
  const meta = COLUMN_META[status]

  return (
    <div className="flex flex-col w-64 shrink-0">
      {/* Column header */}
      <div className="flex items-center gap-2 mb-3 px-1">
        <span className={`text-xs font-semibold uppercase tracking-wide ${meta.color}`}>
          {meta.label}
        </span>
        <span className="bg-white/10 text-white/50 text-xs rounded-full px-1.5 py-0.5 font-medium">
          {tasks.length}
        </span>
      </div>

      {/* Drop zone */}
      <div
        ref={setNodeRef}
        className={`flex flex-col gap-2 min-h-[200px] rounded-lg p-2 transition-colors ${
          isOver ? 'bg-indigo-500/10' : 'bg-white/[0.02]'
        }`}
      >
        <SortableContext items={tasks.map(t => t.id)} strategy={verticalListSortingStrategy}>
          {tasks.map(task => (
            <TaskCard key={task.id} task={task} />
          ))}
        </SortableContext>
        {tasks.length === 0 && (
          <div className="flex items-center justify-center h-24 text-white/20 text-xs">
            Drop here
          </div>
        )}
      </div>
    </div>
  )
}

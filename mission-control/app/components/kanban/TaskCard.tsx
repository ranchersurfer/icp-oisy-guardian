'use client'

import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { Task } from '@/lib/tasks'

const PRIORITY_STYLES = {
  high: 'bg-red-500/20 text-red-400 border-red-500/30',
  med: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  low: 'bg-slate-500/20 text-slate-400 border-slate-500/30',
}

const ASSIGNEE_COLORS: Record<string, string> = {
  'guardian-dev': 'bg-purple-500',
  'mission-control': 'bg-indigo-500',
  'creator': 'bg-pink-500',
  'prospector': 'bg-orange-500',
  'human': 'bg-emerald-500',
}

function getAssigneeColor(assignee?: string): string {
  if (!assignee) return 'bg-slate-500'
  return ASSIGNEE_COLORS[assignee] || 'bg-blue-500'
}

function getInitial(assignee?: string): string {
  if (!assignee) return '?'
  return assignee.charAt(0).toUpperCase()
}

function timeAgo(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime()
  const mins = Math.floor(diff / 60000)
  if (mins < 60) return `${mins}m ago`
  const hours = Math.floor(mins / 60)
  if (hours < 24) return `${hours}h ago`
  const days = Math.floor(hours / 24)
  return `${days}d ago`
}

interface TaskCardProps {
  task: Task
}

export default function TaskCard({ task }: TaskCardProps) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id: task.id,
  })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  }

  return (
    <div
      ref={setNodeRef}
      style={style}
      {...attributes}
      {...listeners}
      className="bg-[#1e1e35] border border-white/10 rounded-lg p-3 cursor-grab active:cursor-grabbing group hover:border-indigo-500/40 transition-colors"
    >
      <p className="text-white/90 text-sm font-medium leading-snug mb-2">{task.title}</p>

      {task.description && (
        <p className="text-white/40 text-xs mb-3 line-clamp-2">{task.description}</p>
      )}

      <div className="flex items-center justify-between">
        <div className="flex items-center gap-1.5">
          {/* Assignee circle */}
          <div
            className={`w-5 h-5 rounded-full flex items-center justify-center text-[10px] font-bold text-white ${getAssigneeColor(task.assignee)}`}
            title={task.assignee}
          >
            {getInitial(task.assignee)}
          </div>

          {/* Priority badge */}
          {task.priority && (
            <span
              className={`text-[10px] px-1.5 py-0.5 rounded border font-medium ${PRIORITY_STYLES[task.priority]}`}
            >
              {task.priority}
            </span>
          )}
        </div>

        {/* Updated at */}
        <span className="text-white/30 text-[10px]">{timeAgo(task.updated_at)}</span>
      </div>
    </div>
  )
}

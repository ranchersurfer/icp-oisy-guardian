'use client'

import { useQuery } from '@tanstack/react-query'
import Link from 'next/link'
import { ArrowLeft, MessageSquarePlus, CheckSquare, FileText } from 'lucide-react'
import type { Project } from './ProjectsGrid'

interface Task {
  id: string
  title: string
  description?: string
  status: 'backlog' | 'doing' | 'review' | 'done'
  assignee?: string
  priority?: string
  project_id?: string
  updated_at?: string
}

const STATUS_CONFIG: Record<string, { label: string; bg: string; text: string }> = {
  not_started: { label: 'Not Started', bg: 'bg-slate-500/20', text: 'text-slate-400' },
  in_progress: { label: 'In Progress', bg: 'bg-blue-500/20', text: 'text-blue-300' },
  blocked: { label: 'Blocked', bg: 'bg-red-500/20', text: 'text-red-400' },
  done: { label: 'Done', bg: 'bg-emerald-500/20', text: 'text-emerald-400' },
}

const TASK_STATUS_CONFIG: Record<string, { label: string; dot: string }> = {
  backlog: { label: 'Backlog', dot: 'bg-slate-500' },
  doing: { label: 'In Progress', dot: 'bg-blue-400' },
  review: { label: 'In Review', dot: 'bg-yellow-400' },
  done: { label: 'Done', dot: 'bg-emerald-400' },
}

const PRIORITY_CONFIG: Record<string, { label: string; text: string }> = {
  high: { label: 'High', text: 'text-red-400' },
  med: { label: 'Med', text: 'text-yellow-400' },
  low: { label: 'Low', text: 'text-slate-400' },
}

const AGENT_LABELS: Record<string, string> = {
  'dream-cycle': 'Dream Cycle',
  'mission-control': 'Mission Control',
  'guardian-dev': 'Guardian-Dev',
  prospector: 'Prospector',
  creator: 'Creator',
  main: 'Mission Control',
}

function ProgressBar({ value }: { value: number }) {
  return (
    <div className="h-2 bg-white/10 rounded-full overflow-hidden">
      <div
        className={`h-full rounded-full transition-all ${
          value === 100 ? 'bg-emerald-500' : value > 0 ? 'bg-indigo-500' : 'bg-slate-600'
        }`}
        style={{ width: `${Math.min(100, Math.max(0, value))}%` }}
      />
    </div>
  )
}

export default function ProjectDetail({ projectId }: { projectId: string }) {
  const { data: projects = [], isLoading: loadingProjects } = useQuery<Project[]>({
    queryKey: ['projects'],
    queryFn: () => fetch('/api/projects').then(r => r.json()),
  })

  const { data: allTasks = [] } = useQuery<Task[]>({
    queryKey: ['tasks'],
    queryFn: () => fetch('/api/tasks').then(r => r.json()),
  })

  const project = projects.find(p => p.id === projectId)
  const linkedTasks = allTasks.filter(t => t.project_id === projectId)

  if (loadingProjects) {
    return <div className="h-64 flex items-center justify-center text-white/30 text-sm">Loading…</div>
  }

  if (!project) {
    return (
      <div className="space-y-4">
        <Link href="/projects" className="inline-flex items-center gap-2 text-sm text-white/50 hover:text-white/80 transition-colors">
          <ArrowLeft size={14} /> Back to Projects
        </Link>
        <div className="text-white/30 text-sm">Project not found.</div>
      </div>
    )
  }

  const status = STATUS_CONFIG[project.status] ?? STATUS_CONFIG.not_started
  const ownerLabel = AGENT_LABELS[project.owner_agent_id] ?? project.owner_agent_id

  return (
    <div className="space-y-6 max-w-4xl">
      {/* Back */}
      <Link href="/projects" className="inline-flex items-center gap-2 text-sm text-white/50 hover:text-white/80 transition-colors">
        <ArrowLeft size={14} /> Back to Projects
      </Link>

      {/* Header */}
      <div className="bg-[#1a1a2e] border border-white/10 rounded-xl p-6">
        <div className="flex items-start justify-between gap-4 mb-4">
          <div>
            <h1 className="text-2xl font-bold text-white mb-1">{project.name}</h1>
            <p className="text-sm text-white/50">Owner: {ownerLabel}</p>
          </div>
          <span className={`text-sm px-3 py-1 rounded-full font-medium shrink-0 ${status.bg} ${status.text}`}>
            {status.label}
          </span>
        </div>

        {/* Progress */}
        <div className="mb-4">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm text-white/50">Progress</span>
            <span className="text-sm font-semibold text-white">{project.progress}%</span>
          </div>
          <ProgressBar value={project.progress} />
        </div>

        {/* Meta */}
        {(project.created_at || project.roadmap_id) && (
          <div className="flex flex-wrap gap-4 text-xs text-white/35 pt-4 border-t border-white/5">
            {project.roadmap_id && <span>Roadmap: {project.roadmap_id}</span>}
            {project.created_at && (
              <span>Started: {new Date(project.created_at).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}</span>
            )}
          </div>
        )}
      </div>

      {/* Overview */}
      <div className="bg-[#1a1a2e] border border-white/10 rounded-xl p-6">
        <h2 className="text-sm font-semibold text-white/70 uppercase tracking-wide mb-3">Overview & Goal</h2>
        <p className="text-white/70 leading-relaxed">{project.description}</p>

        {project.related_docs.length > 0 && (
          <div className="mt-4 pt-4 border-t border-white/5">
            <h3 className="text-xs text-white/40 mb-2 flex items-center gap-1.5">
              <FileText size={12} /> Related Docs
            </h3>
            <div className="flex flex-wrap gap-2">
              {project.related_docs.map(doc => (
                <span key={doc} className="text-xs text-indigo-300/70 bg-indigo-500/10 px-2 py-0.5 rounded font-mono">
                  {doc}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Linked Tasks */}
      <div className="bg-[#1a1a2e] border border-white/10 rounded-xl p-6">
        <h2 className="text-sm font-semibold text-white/70 uppercase tracking-wide mb-3 flex items-center gap-2">
          <CheckSquare size={14} />
          Linked Tasks
          {linkedTasks.length > 0 && (
            <span className="text-xs bg-white/10 px-1.5 py-0.5 rounded font-normal text-white/50">{linkedTasks.length}</span>
          )}
        </h2>
        {linkedTasks.length === 0 ? (
          <p className="text-white/30 text-sm">No tasks linked to this project yet.</p>
        ) : (
          <div className="space-y-2">
            {linkedTasks.map(task => {
              const ts = TASK_STATUS_CONFIG[task.status] ?? { label: task.status, dot: 'bg-slate-500' }
              const pri = task.priority ? PRIORITY_CONFIG[task.priority] : null
              return (
                <div key={task.id} className="flex items-center gap-3 py-2 border-b border-white/5 last:border-0">
                  <span className={`w-2 h-2 rounded-full shrink-0 ${ts.dot}`} />
                  <div className="flex-1 min-w-0">
                    <span className="text-sm text-white/80">{task.title}</span>
                    {task.description && (
                      <p className="text-xs text-white/35 truncate">{task.description}</p>
                    )}
                  </div>
                  <div className="flex items-center gap-2 shrink-0">
                    {pri && <span className={`text-xs ${pri.text}`}>{pri.label}</span>}
                    <span className="text-xs text-white/40">{ts.label}</span>
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </div>

      {/* Ask Mission Control */}
      <div className="bg-[#1a1a2e] border border-white/10 rounded-xl p-6">
        <h2 className="text-sm font-semibold text-white/70 uppercase tracking-wide mb-3">Actions</h2>
        <button
          className="inline-flex items-center gap-2 px-4 py-2.5 bg-indigo-600/30 hover:bg-indigo-600/50 border border-indigo-500/40 hover:border-indigo-400/60 text-indigo-200 text-sm font-medium rounded-lg transition-all"
          onClick={() => {}}
        >
          <MessageSquarePlus size={15} />
          Ask Mission Control
        </button>
        <p className="text-xs text-white/30 mt-2">
          Ask Mission Control: "What's 1 task we can do right now for {project.name}?"
        </p>
      </div>
    </div>
  )
}

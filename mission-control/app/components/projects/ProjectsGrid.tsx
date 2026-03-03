'use client'

import { useQuery } from '@tanstack/react-query'
import Link from 'next/link'
import { ExternalLink } from 'lucide-react'

export interface Project {
  id: string
  name: string
  description: string
  status: 'not_started' | 'in_progress' | 'blocked' | 'done'
  progress: number
  owner_agent_id: string
  created_at: string | null
  updated_at: string | null
  roadmap_id: string | null
  related_tasks: string[]
  related_docs: string[]
  related_memories: string[]
}

const STATUS_CONFIG: Record<string, { label: string; bg: string; text: string }> = {
  not_started: { label: 'Not Started', bg: 'bg-slate-500/20', text: 'text-slate-400' },
  in_progress: { label: 'In Progress', bg: 'bg-blue-500/20', text: 'text-blue-300' },
  blocked: { label: 'Blocked', bg: 'bg-red-500/20', text: 'text-red-400' },
  done: { label: 'Done', bg: 'bg-emerald-500/20', text: 'text-emerald-400' },
}

const AGENT_LABELS: Record<string, string> = {
  'dream-cycle': 'Dream Cycle',
  'mission-control': 'Mission Control',
  'guardian-dev': 'Guardian-Dev',
  prospector: 'Prospector',
  creator: 'Creator',
  main: 'Mission Control',
}

function agentLabel(id: string) {
  return AGENT_LABELS[id] ?? id
}

function ProgressBar({ value }: { value: number }) {
  return (
    <div className="h-1.5 bg-white/10 rounded-full overflow-hidden">
      <div
        className={`h-full rounded-full transition-all ${
          value === 100 ? 'bg-emerald-500' : value > 0 ? 'bg-indigo-500' : 'bg-slate-600'
        }`}
        style={{ width: `${Math.min(100, Math.max(0, value))}%` }}
      />
    </div>
  )
}

export default function ProjectsGrid() {
  const { data: projects = [], isLoading } = useQuery<Project[]>({
    queryKey: ['projects'],
    queryFn: () => fetch('/api/projects').then(r => r.json()),
    refetchInterval: 15000,
  })

  if (isLoading) {
    return (
      <div className="h-64 flex items-center justify-center text-white/30 text-sm">Loading projects…</div>
    )
  }

  const statusOrder: Record<string, number> = { in_progress: 0, not_started: 1, blocked: 2, done: 3 }
  const sorted = [...projects].sort((a, b) => (statusOrder[a.status] ?? 9) - (statusOrder[b.status] ?? 9))

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-semibold text-white">Projects</h1>
          <p className="text-sm text-white/40 mt-0.5">{projects.length} projects tracked</p>
        </div>
      </div>

      {/* Status summary */}
      <div className="flex gap-4 flex-wrap">
        {Object.entries(STATUS_CONFIG).map(([status, cfg]) => {
          const count = projects.filter(p => p.status === status).length
          return (
            <div key={status} className="flex items-center gap-2">
              <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${cfg.bg} ${cfg.text}`}>
                {cfg.label}
              </span>
              <span className="text-sm text-white/50">{count}</span>
            </div>
          )
        })}
      </div>

      {/* Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
        {sorted.map(project => {
          const status = STATUS_CONFIG[project.status] ?? STATUS_CONFIG.not_started
          return (
            <Link
              key={project.id}
              href={`/projects/${project.id}`}
              className="group block bg-[#1a1a2e] border border-white/10 rounded-xl p-5 hover:border-indigo-500/40 hover:bg-[#1f1f38] transition-all"
            >
              <div className="flex items-start justify-between mb-3">
                <h3 className="font-semibold text-white group-hover:text-indigo-200 transition-colors leading-tight">
                  {project.name}
                </h3>
                <ExternalLink size={14} className="text-white/20 group-hover:text-white/40 shrink-0 mt-0.5 ml-2" />
              </div>

              <p className="text-sm text-white/50 leading-relaxed mb-4 line-clamp-2">
                {project.description}
              </p>

              {/* Progress */}
              <div className="mb-3">
                <div className="flex items-center justify-between mb-1.5">
                  <span className="text-xs text-white/40">Progress</span>
                  <span className="text-xs font-medium text-white/60">{project.progress}%</span>
                </div>
                <ProgressBar value={project.progress} />
              </div>

              {/* Footer */}
              <div className="flex items-center justify-between pt-3 border-t border-white/5">
                <span className="text-xs text-white/40">{agentLabel(project.owner_agent_id)}</span>
                <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${status.bg} ${status.text}`}>
                  {status.label}
                </span>
              </div>
            </Link>
          )
        })}
      </div>

      {projects.length === 0 && (
        <div className="text-center py-16 text-white/30 text-sm">No projects found</div>
      )}
    </div>
  )
}

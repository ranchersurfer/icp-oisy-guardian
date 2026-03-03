'use client'

import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Cpu, ChevronRight, X, CheckCircle, AlertCircle, Clock, Zap } from 'lucide-react'

interface AgentModel {
  primary: string
  fallbacks?: string[]
}

interface AgentTools {
  allowed: string[]
  elevated?: boolean
}

interface CronJob {
  job_id: string
  name: string
  schedule: string
  schedule_description?: string
  enabled: boolean
}

interface Agent {
  id: string
  name: string
  role: string
  model: AgentModel
  tools: AgentTools
  current_task?: string
  task_status?: string
  cron_jobs?: CronJob[]
  status?: 'idle' | 'working' | 'error'
  metrics?: Record<string, string>
}

interface AgentsState {
  generated_at: string
  metadata: Record<string, string>
  agents: Agent[]
}

async function fetchAgents(): Promise<AgentsState> {
  const res = await fetch('/api/agents')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

const STATUS_STYLES = {
  idle: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
  working: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  error: 'bg-red-500/20 text-red-400 border-red-500/30',
}

const STATUS_ICONS = {
  idle: <CheckCircle size={12} />,
  working: <Clock size={12} />,
  error: <AlertCircle size={12} />,
}

function AgentCard({
  agent,
  isRoot,
  onClick,
  selected,
}: {
  agent: Agent
  isRoot?: boolean
  onClick: () => void
  selected: boolean
}) {
  const status = agent.status || 'idle'
  return (
    <button
      onClick={onClick}
      className={`text-left w-full rounded-xl border p-4 transition-all ${
        selected
          ? 'border-indigo-500/40 bg-indigo-500/10 shadow-lg shadow-indigo-500/5'
          : isRoot
          ? 'border-white/20 bg-white/[0.05] hover:border-white/30'
          : 'border-white/10 bg-white/[0.03] hover:border-white/20'
      }`}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex items-center gap-2">
          <div className={`p-1.5 rounded-lg ${isRoot ? 'bg-indigo-500/20' : 'bg-white/5'}`}>
            <Cpu size={14} className={isRoot ? 'text-indigo-400' : 'text-white/50'} />
          </div>
          <div>
            <div className="text-white font-medium text-sm">{agent.name}</div>
            <div className="text-white/40 text-xs mt-0.5">{agent.model.primary}</div>
          </div>
        </div>
        <div className="flex items-center gap-1 shrink-0">
          {status === 'working' && (
            <span className="relative flex h-2 w-2 mr-1">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
              <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500" />
            </span>
          )}
          <span className={`flex items-center gap-1 text-xs px-2 py-0.5 rounded-full border ${STATUS_STYLES[status]}`}>
            {STATUS_ICONS[status]}
            {status}
          </span>
          <ChevronRight size={12} className="text-white/20" />
        </div>
      </div>
      <p className="text-white/40 text-xs mt-2 leading-relaxed line-clamp-2">{agent.role}</p>
      {agent.current_task && (
        <div className="mt-2 text-white/50 text-xs bg-white/5 rounded px-2 py-1 truncate">
          🔧 {agent.current_task}
        </div>
      )}
      <div className="flex items-center gap-2 mt-2 text-white/25 text-xs">
        <Zap size={10} />
        <span>{agent.tools.allowed.length} tools</span>
        {agent.tools.elevated && <span className="text-amber-400/60">• elevated</span>}
      </div>
    </button>
  )
}

function AgentDetail({ agent, onClose }: { agent: Agent; onClose: () => void }) {
  const status = agent.status || 'idle'
  return (
    <div className="w-80 shrink-0 border border-white/10 rounded-xl bg-[#0e0e1a] flex flex-col overflow-hidden animate-in slide-in-from-right-4 duration-200">
      <div className="flex items-center justify-between px-4 py-3 border-b border-white/10">
        <div className="flex items-center gap-2">
          <Cpu size={14} className="text-indigo-400" />
          <span className="text-white font-medium text-sm">{agent.name}</span>
        </div>
        <button onClick={onClose} className="text-white/30 hover:text-white transition-colors">
          <X size={14} />
        </button>
      </div>
      <div className="flex-1 overflow-auto p-4 space-y-4">
        {/* Status */}
        <div>
          <div className="text-white/30 text-xs font-semibold uppercase mb-2">Status</div>
          <span className={`flex items-center gap-1.5 text-xs px-2 py-1 rounded-full border w-fit ${STATUS_STYLES[status]}`}>
            {STATUS_ICONS[status]} {status}
          </span>
        </div>

        {/* Role */}
        <div>
          <div className="text-white/30 text-xs font-semibold uppercase mb-2">Role</div>
          <p className="text-white/60 text-xs leading-relaxed">{agent.role}</p>
        </div>

        {/* Model */}
        <div>
          <div className="text-white/30 text-xs font-semibold uppercase mb-2">Model</div>
          <div className="text-white/70 text-xs font-mono bg-white/5 rounded px-2 py-1">{agent.model.primary}</div>
          {agent.model.fallbacks && agent.model.fallbacks.length > 0 && (
            <div className="mt-1 text-white/30 text-xs">Fallbacks: {agent.model.fallbacks.join(', ')}</div>
          )}
        </div>

        {/* Current task */}
        {agent.current_task && (
          <div>
            <div className="text-white/30 text-xs font-semibold uppercase mb-2">Current Task</div>
            <div className="text-white/60 text-xs bg-yellow-500/5 border border-yellow-500/20 rounded px-2 py-2 leading-relaxed">
              {agent.current_task}
            </div>
          </div>
        )}

        {/* Tools */}
        <div>
          <div className="text-white/30 text-xs font-semibold uppercase mb-2">Tools ({agent.tools.allowed.length})</div>
          <div className="flex flex-wrap gap-1">
            {agent.tools.allowed.map(t => (
              <span key={t} className="text-xs px-1.5 py-0.5 bg-white/5 border border-white/10 rounded text-white/50">
                {t}
              </span>
            ))}
          </div>
          {agent.tools.elevated && (
            <div className="mt-1.5 text-amber-400/60 text-xs flex items-center gap-1">
              <AlertCircle size={10} /> Elevated permissions
            </div>
          )}
        </div>

        {/* Cron jobs */}
        {agent.cron_jobs && agent.cron_jobs.length > 0 && (
          <div>
            <div className="text-white/30 text-xs font-semibold uppercase mb-2">Cron Jobs</div>
            <div className="space-y-1.5">
              {agent.cron_jobs.map(job => (
                <div key={job.job_id} className="bg-white/5 rounded p-2 text-xs">
                  <div className="text-white/70 font-medium">{job.name}</div>
                  <div className="text-white/30 font-mono mt-0.5">{job.schedule}</div>
                  {job.schedule_description && (
                    <div className="text-white/40 mt-0.5">{job.schedule_description}</div>
                  )}
                  <div className={`mt-1 text-xs ${job.enabled ? 'text-emerald-400' : 'text-white/30'}`}>
                    {job.enabled ? '● Active' : '○ Paused'}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Metrics */}
        {agent.metrics && Object.keys(agent.metrics).length > 0 && (
          <div>
            <div className="text-white/30 text-xs font-semibold uppercase mb-2">Metrics</div>
            <div className="space-y-1">
              {Object.entries(agent.metrics).map(([k, v]) => (
                <div key={k} className="flex justify-between text-xs">
                  <span className="text-white/40">{k.replace(/_/g, ' ')}</span>
                  <span className="text-white/70">{v}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default function TeamView() {
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null)

  const { data: state, isLoading } = useQuery({
    queryKey: ['agents'],
    queryFn: fetchAgents,
    refetchInterval: 15000,
  })

  const agents = state?.agents || []
  const root = agents.find(a => a.id === 'main' || a.id === 'mission-control')
  const children = agents.filter(a => a.id !== root?.id)

  return (
    <div className="flex flex-col gap-6 h-full">
      {/* Mission banner */}
      <div className="rounded-xl border border-indigo-500/20 bg-gradient-to-r from-indigo-500/10 via-purple-500/5 to-transparent px-6 py-4">
        <div className="text-xs text-indigo-400/70 uppercase font-semibold tracking-wider mb-1">Mission Statement</div>
        <p className="text-white/80 text-sm font-medium">
          Building the future of AI-powered automation
        </p>
        {state?.generated_at && (
          <p className="text-white/30 text-xs mt-1">
            State updated: {new Date(state.generated_at).toLocaleString()}
          </p>
        )}
      </div>

      <div className="flex gap-4 flex-1 min-h-0 overflow-hidden">
        {/* Org tree */}
        <div className="flex-1 overflow-auto">
          {isLoading ? (
            <div className="text-white/30 text-sm">Loading agents…</div>
          ) : (
            <div className="flex flex-col gap-4">
              {/* Root node */}
              {root && (
                <div>
                  <div className="text-white/30 text-xs uppercase font-semibold mb-2">Mission Control</div>
                  <AgentCard
                    agent={root}
                    isRoot
                    onClick={() => setSelectedAgent(root)}
                    selected={selectedAgent?.id === root.id}
                  />
                </div>
              )}

              {/* Sub-agents */}
              {children.length > 0 && (
                <div>
                  <div className="text-white/30 text-xs uppercase font-semibold mb-2">
                    Sub-agents ({children.length})
                  </div>
                  <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 xl:grid-cols-3">
                    {children.map(agent => (
                      <AgentCard
                        key={agent.id}
                        agent={agent}
                        onClick={() => setSelectedAgent(agent)}
                        selected={selectedAgent?.id === agent.id}
                      />
                    ))}
                  </div>
                </div>
              )}

              {agents.length === 0 && (
                <div className="text-white/20 text-sm">No agents found in agents_state.json</div>
              )}
            </div>
          )}
        </div>

        {/* Detail panel */}
        {selectedAgent && (
          <AgentDetail agent={selectedAgent} onClose={() => setSelectedAgent(null)} />
        )}
      </div>
    </div>
  )
}

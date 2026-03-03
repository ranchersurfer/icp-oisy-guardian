'use client'

import { useQuery } from '@tanstack/react-query'
import { BarChart2, Cpu } from 'lucide-react'
import { useEffect, useState } from 'react'

interface Agent {
  id: string
  status?: 'idle' | 'working' | 'error'
}

interface AgentsState {
  agents: Agent[]
}

async function fetchAgentsForTopBar(): Promise<AgentsState> {
  const res = await fetch('/api/agents')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

interface TopBarProps {
  agentName?: string
  onMetricsToggle?: () => void
  metricsOpen?: boolean
}

const STATUS_STYLES = {
  idle: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
  working: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  error: 'bg-red-500/20 text-red-400 border-red-500/30',
}

function useSecondsAgo(refetchedAt: number | undefined) {
  const [secs, setSecs] = useState(0)
  useEffect(() => {
    if (!refetchedAt) return
    const tick = () => setSecs(Math.floor((Date.now() - refetchedAt) / 1000))
    tick()
    const id = setInterval(tick, 5000)
    return () => clearInterval(id)
  }, [refetchedAt])
  return secs
}

export default function TopBar({
  agentName = 'Mission Control',
  onMetricsToggle,
  metricsOpen,
}: TopBarProps) {
  const query = useQuery({
    queryKey: ['agents'],
    queryFn: fetchAgentsForTopBar,
    refetchInterval: 15000,
  })

  const agents = query.data?.agents ?? []
  const anyWorking = agents.some(a => a.status === 'working')
  const hasError = agents.some(a => a.status === 'error')
  const overallStatus = hasError ? 'error' : anyWorking ? 'working' : 'idle'

  const secsAgo = useSecondsAgo(query.dataUpdatedAt)

  const lastUpdatedLabel = query.dataUpdatedAt
    ? secsAgo < 10
      ? 'just now'
      : secsAgo < 60
      ? `${secsAgo}s ago`
      : `${Math.floor(secsAgo / 60)}m ago`
    : null

  return (
    <header className="h-12 border-b border-white/10 bg-[#12121f] flex items-center px-4 gap-3 shrink-0">
      {/* Agent pill */}
      <div className="flex items-center gap-1.5 bg-white/5 border border-white/10 rounded-full px-3 py-1">
        <Cpu size={12} className="text-indigo-400" />
        <span className="text-white/80 text-xs font-medium">{agentName}</span>
      </div>

      {/* Status badge — pulses when working */}
      <span
        className={`text-xs px-2 py-0.5 rounded-full border font-medium ${STATUS_STYLES[overallStatus]} ${
          anyWorking ? 'animate-pulse' : ''
        }`}
      >
        {overallStatus.charAt(0).toUpperCase() + overallStatus.slice(1)}
      </span>

      <div className="flex-1" />

      {/* Last updated */}
      {lastUpdatedLabel && (
        <span className="text-white/20 text-xs">
          updated {lastUpdatedLabel}
        </span>
      )}

      <span className="text-white/25 text-xs">
        {new Date().toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
      </span>

      {/* Metrics toggle */}
      {onMetricsToggle && (
        <button
          onClick={onMetricsToggle}
          className={`flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs transition-colors border ${
            metricsOpen
              ? 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30'
              : 'text-white/40 border-white/10 hover:text-white/70 hover:border-white/20'
          }`}
        >
          <BarChart2 size={12} />
          Metrics
        </button>
      )}
    </header>
  )
}

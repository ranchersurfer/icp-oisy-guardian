'use client'

import { Badge } from '@/components/ui/badge'
import { Cpu, BarChart2 } from 'lucide-react'

interface TopBarProps {
  agentName?: string
  status?: 'idle' | 'working' | 'error'
  onMetricsToggle?: () => void
  metricsOpen?: boolean
}

const STATUS_STYLES = {
  idle: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
  working: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  error: 'bg-red-500/20 text-red-400 border-red-500/30',
}

export default function TopBar({
  agentName = 'Mission Control',
  status = 'idle',
  onMetricsToggle,
  metricsOpen,
}: TopBarProps) {
  return (
    <header className="h-12 border-b border-white/10 bg-[#12121f] flex items-center px-4 gap-3 shrink-0">
      {/* Agent pill */}
      <div className="flex items-center gap-1.5 bg-white/5 border border-white/10 rounded-full px-3 py-1">
        <Cpu size={12} className="text-indigo-400" />
        <span className="text-white/80 text-xs font-medium">{agentName}</span>
      </div>

      {/* Status badge */}
      <span
        className={`text-xs px-2 py-0.5 rounded-full border font-medium ${STATUS_STYLES[status]}`}
      >
        {status.charAt(0).toUpperCase() + status.slice(1)}
      </span>

      <div className="flex-1" />

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

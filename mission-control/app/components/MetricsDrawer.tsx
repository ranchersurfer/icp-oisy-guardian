'use client'

import { useQuery } from '@tanstack/react-query'
import { X, DollarSign, Brain, Activity, Shield } from 'lucide-react'

interface Metrics {
  monthlyCost: string
  tier2FileCount: number
  memoryLastUpdated: string
  activeCronJobs: number
  securityScore: string
}

async function fetchMetrics(): Promise<Metrics> {
  const res = await fetch('/api/agents?metrics=1')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

interface MetricsDrawerProps {
  open: boolean
  onClose: () => void
}

function StatCard({ icon, label, value, sub }: { icon: React.ReactNode; label: string; value: string; sub?: string }) {
  return (
    <div className="bg-white/[0.03] border border-white/10 rounded-lg p-3">
      <div className="flex items-center gap-2 mb-2">
        <div className="text-white/30">{icon}</div>
        <span className="text-white/40 text-xs uppercase font-semibold">{label}</span>
      </div>
      <div className="text-white text-lg font-semibold">{value}</div>
      {sub && <div className="text-white/30 text-xs mt-0.5 truncate">{sub}</div>}
    </div>
  )
}

export default function MetricsDrawer({ open, onClose }: MetricsDrawerProps) {
  const { data: metrics, isLoading } = useQuery({
    queryKey: ['metrics'],
    queryFn: fetchMetrics,
    refetchInterval: 60000,
    enabled: open,
  })

  if (!open) return null

  return (
    <div className="w-64 shrink-0 border-l border-white/10 bg-[#0e0e1a] flex flex-col overflow-hidden">
      <div className="flex items-center justify-between px-4 py-3 border-b border-white/10">
        <span className="text-white/70 text-sm font-medium">Metrics</span>
        <button onClick={onClose} className="text-white/30 hover:text-white transition-colors">
          <X size={14} />
        </button>
      </div>
      <div className="flex-1 overflow-auto p-3 space-y-3">
        {isLoading ? (
          <div className="text-white/30 text-xs">Loading…</div>
        ) : metrics ? (
          <>
            <StatCard
              icon={<DollarSign size={14} />}
              label="Monthly Cost"
              value={metrics.monthlyCost}
            />
            <StatCard
              icon={<Brain size={14} />}
              label="Memory Files"
              value={String(metrics.tier2FileCount)}
              sub={metrics.memoryLastUpdated
                ? `Updated ${new Date(metrics.memoryLastUpdated).toLocaleDateString()}`
                : 'No updates yet'}
            />
            <StatCard
              icon={<Activity size={14} />}
              label="Active Cron Jobs"
              value={String(metrics.activeCronJobs)}
            />
            <StatCard
              icon={<Shield size={14} />}
              label="Security Score"
              value={metrics.securityScore}
            />
          </>
        ) : null}
      </div>
    </div>
  )
}

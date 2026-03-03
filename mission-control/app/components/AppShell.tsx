'use client'

import { useState } from 'react'
import Sidebar from '@/components/Sidebar'
import TopBar from '@/components/TopBar'
import MetricsDrawer from '@/components/MetricsDrawer'

export default function AppShell({ children }: { children: React.ReactNode }) {
  const [metricsOpen, setMetricsOpen] = useState(false)

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar />
      <div className="flex flex-col flex-1 min-w-0">
        <TopBar
          onMetricsToggle={() => setMetricsOpen(o => !o)}
          metricsOpen={metricsOpen}
        />
        <div className="flex flex-1 min-h-0 overflow-hidden">
          <main className="flex-1 overflow-auto p-6">{children}</main>
          <MetricsDrawer open={metricsOpen} onClose={() => setMetricsOpen(false)} />
        </div>
      </div>
    </div>
  )
}

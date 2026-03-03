import { NextResponse } from 'next/server'
import fs from 'fs'
import { workspacePath } from '@/lib/workspace'

export async function GET() {
  try {
    const schedulePath = workspacePath('schedule.json')
    let items: unknown[] = []

    if (fs.existsSync(schedulePath)) {
      const raw = fs.readFileSync(schedulePath, 'utf8')
      items = JSON.parse(raw)
    }

    // Also pull cron jobs from agents_state.json
    const agentsPath = workspacePath('agents_state.json')
    if (fs.existsSync(agentsPath)) {
      const agentsRaw = fs.readFileSync(agentsPath, 'utf8')
      const agentsData = JSON.parse(agentsRaw)
      const scheduleIds = new Set((items as { id: string }[]).map((i) => i.id))

      for (const agent of agentsData.agents ?? []) {
        for (const job of agent.cron_jobs ?? []) {
          const syntheticId = `agent-${agent.id}-${job.job_id ?? job.schedule}`
          if (!scheduleIds.has(syntheticId)) {
            items.push({
              id: syntheticId,
              title: job.name ?? job.description ?? `${agent.name} cron`,
              description: job.description ?? '',
              schedule: job.schedule,
              schedule_description: job.schedule_description ?? job.description ?? '',
              agent_id: agent.id,
              type: 'cron',
              next_run_at: job.next_run ?? null,
              last_run_at: job.last_run ?? null,
              source_file: job.script ?? null,
              status: job.enabled === false ? 'planned' : 'active',
            })
          }
        }
      }
    }

    return NextResponse.json(items)
  } catch (e) {
    return NextResponse.json({ error: String(e) }, { status: 500 })
  }
}

import fs from 'fs'
import { workspacePath } from './workspace'

export interface AgentModel {
  primary: string
  fallbacks?: string[]
}

export interface AgentTools {
  allowed: string[]
  elevated?: boolean
  comment?: string
}

export interface CronJob {
  job_id: string
  name: string
  schedule: string
  schedule_description?: string
  enabled: boolean
  script?: string
  created_at: string
  next_run?: string
}

export interface Agent {
  id: string
  name: string
  role: string
  workspace?: string
  model: AgentModel
  execution_mode?: string
  tools: AgentTools
  current_task?: string
  task_status?: string
  cron_jobs?: CronJob[]
  metrics?: Record<string, string>
  parent_id?: string
  status?: 'idle' | 'working' | 'error'
}

export interface AgentsState {
  generated_at: string
  metadata: Record<string, string>
  agents: Agent[]
}

export async function readAgentsState(): Promise<AgentsState> {
  const p = workspacePath('agents_state.json')
  if (!fs.existsSync(p)) {
    return {
      generated_at: new Date().toISOString(),
      metadata: {},
      agents: [
        {
          id: 'main',
          name: 'Mission Control',
          role: 'Primary orchestrator',
          model: { primary: 'claude-sonnet-4-6' },
          tools: { allowed: ['read', 'write', 'exec', 'browser', 'web_search'] },
          status: 'idle',
        }
      ]
    }
  }
  const raw = fs.readFileSync(p, 'utf8')
  const data: AgentsState = JSON.parse(raw)
  // normalize status
  data.agents = data.agents.map(a => ({
    ...a,
    status: (a.task_status === 'in_progress' || a.task_status === 'automated')
      ? 'working'
      : a.task_status === 'error'
      ? 'error'
      : 'idle',
  }))
  return data
}

export async function readMetrics() {
  const memDir = workspacePath('memory')
  let tier2Count = 0
  let lastUpdated = ''
  if (fs.existsSync(memDir)) {
    const files = fs.readdirSync(memDir).filter(f => f.endsWith('.md'))
    tier2Count = files.length
    const mtimes = files.map(f => {
      try { return fs.statSync(`${memDir}/${f}`).mtime.toISOString() } catch { return '' }
    }).filter(Boolean).sort().reverse()
    lastUpdated = mtimes[0] || ''
  }

  // count active cron jobs from agents_state
  let cronCount = 0
  try {
    const state = await readAgentsState()
    state.agents.forEach(a => {
      cronCount += (a.cron_jobs || []).filter(c => c.enabled).length
    })
  } catch {}

  // security score
  let securityScore = '8/10'
  const secPath = workspacePath('memory', 'security.md')
  if (fs.existsSync(secPath)) {
    const content = fs.readFileSync(secPath, 'utf8')
    const m = content.match(/score[:\s]+(\d+\/\d+|\d+)/i)
    if (m) securityScore = m[1]
  }

  return {
    monthlyCost: '$20/mo (est.)',
    tier2FileCount: tier2Count,
    memoryLastUpdated: lastUpdated,
    activeCronJobs: cronCount,
    securityScore,
  }
}

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

export interface AgentStatusEntry {
  status: 'idle' | 'working' | 'error'
  current_task: string | null
  last_updated: string | null
}

export function readAgentStatusFile(): Record<string, AgentStatusEntry> {
  const p = workspacePath('agent-status.json')
  if (!fs.existsSync(p)) {
    const defaults: Record<string, AgentStatusEntry> = {
      'guardian-dev': { status: 'working', current_task: 'Phase 1c: ICRC Index Integration', last_updated: new Date().toISOString() },
      'dream-cycle': { status: 'idle', current_task: null, last_updated: null },
      'prospector': { status: 'idle', current_task: null, last_updated: null },
      'creator': { status: 'idle', current_task: null, last_updated: null },
      'mission-control': { status: 'working', current_task: 'Orchestrating agents', last_updated: new Date().toISOString() },
    }
    fs.writeFileSync(p, JSON.stringify(defaults, null, 2), 'utf8')
    return defaults
  }
  try {
    return JSON.parse(fs.readFileSync(p, 'utf8'))
  } catch {
    return {}
  }
}

export async function readAgentsState(): Promise<AgentsState> {
  const p = workspacePath('agents_state.json')
  const agentStatus = readAgentStatusFile()

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
          status: agentStatus['mission-control']?.status ?? 'idle',
          current_task: agentStatus['mission-control']?.current_task ?? undefined,
        }
      ]
    }
  }

  const raw = fs.readFileSync(p, 'utf8')
  const data: AgentsState = JSON.parse(raw)

  // normalize status + merge agent-status.json overrides
  data.agents = data.agents.map(a => {
    const override = agentStatus[a.id]
    const baseStatus: 'idle' | 'working' | 'error' = (a.task_status === 'in_progress' || a.task_status === 'automated')
      ? 'working'
      : a.task_status === 'error'
      ? 'error'
      : 'idle'

    return {
      ...a,
      status: override?.status ?? baseStatus,
      current_task: override?.current_task ?? a.current_task,
    }
  })

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

  // count entries in schedule.json
  let cronCount = 0
  const schedulePath = workspacePath('schedule.json')
  if (fs.existsSync(schedulePath)) {
    try {
      const scheduleData = JSON.parse(fs.readFileSync(schedulePath, 'utf8'))
      cronCount = Array.isArray(scheduleData) ? scheduleData.length : 0
    } catch {}
  }

  // security score — parse from memory/security.md
  let securityScore = '8/10'
  const secPath = workspacePath('memory', 'security.md')
  if (fs.existsSync(secPath)) {
    const content = fs.readFileSync(secPath, 'utf8')
    const m = content.match(/(\d+\/10)/i)
    if (m) securityScore = m[1]
  }

  // cost estimate — parse from memory/cost-optimization.md
  let monthlyCost = '$18-27/mo'
  const costPath = workspacePath('memory', 'cost-optimization.md')
  if (fs.existsSync(costPath)) {
    const content = fs.readFileSync(costPath, 'utf8')
    const m = content.match(/\$(\d+)[-–](\d+)\/mo|\$(\d+)\/mo/)
    if (m) {
      if (m[1] && m[2]) monthlyCost = `$${m[1]}-${m[2]}/mo`
      else if (m[3]) monthlyCost = `$${m[3]}/mo`
    }
  }

  return {
    monthlyCost,
    tier2FileCount: tier2Count,
    memoryLastUpdated: lastUpdated,
    activeCronJobs: cronCount,
    securityScore,
  }
}

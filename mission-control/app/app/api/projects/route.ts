import { NextResponse } from 'next/server'
import fs from 'fs'
import { workspacePath } from '@/lib/workspace'

export async function GET() {
  try {
    const projectsPath = workspacePath('projects.json')
    let projects: unknown[] = []

    if (fs.existsSync(projectsPath)) {
      const raw = fs.readFileSync(projectsPath, 'utf8')
      projects = JSON.parse(raw)
    } else {
      // Fallback: derive from agents_state.json roadmap
      const agentsPath = workspacePath('agents_state.json')
      if (fs.existsSync(agentsPath)) {
        const agentsRaw = fs.readFileSync(agentsPath, 'utf8')
        const agentsData = JSON.parse(agentsRaw)
        projects = (agentsData.roadmap ?? []).map((r: {
          id: string
          title: string
          description: string
          status: string
          owner_agent: string
          started_at?: string
          completed_at?: string
        }) => ({
          id: r.id,
          name: r.title,
          description: r.description,
          status: r.status === 'done' ? 'done' : r.status === 'planning' || r.status === 'planned' ? 'not_started' : 'in_progress',
          progress: r.status === 'done' ? 100 : 0,
          owner_agent_id: r.owner_agent,
          created_at: r.started_at ?? null,
          updated_at: r.completed_at ?? r.started_at ?? null,
          roadmap_id: r.id,
          related_tasks: [],
          related_docs: [],
          related_memories: [],
        }))
      }
    }

    return NextResponse.json(projects)
  } catch (e) {
    return NextResponse.json({ error: String(e) }, { status: 500 })
  }
}

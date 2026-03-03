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
    }

    return NextResponse.json(projects)
  } catch (e) {
    return NextResponse.json({ error: String(e) }, { status: 500 })
  }
}

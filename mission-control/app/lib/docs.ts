import fs from 'fs'
import path from 'path'
import { workspacePath, WORKSPACE_ROOT } from './workspace'

export interface DocEntry {
  id: string
  title: string
  path: string
  relativePath: string
  folder: string
  type: 'spec' | 'plan' | 'strategy' | 'log' | 'misc'
  size: number
  updated_at: string
}

function detectType(filename: string, content?: string): DocEntry['type'] {
  const lower = filename.toLowerCase()
  if (/spec|oisy/.test(lower)) return 'spec'
  if (/plan|roadmap|build/.test(lower)) return 'plan'
  if (/strategy|business|prioritiz/.test(lower)) return 'strategy'
  if (/log|dev_log|memory|brief/.test(lower)) return 'log'
  return 'misc'
}

const SCAN_DIRS = [
  '',
  'guardian-dev',
  'prospector',
  'creator',
  'mission-control',
]

export async function scanDocs(): Promise<DocEntry[]> {
  const docs: DocEntry[] = []
  for (const dir of SCAN_DIRS) {
    const absDir = dir ? workspacePath(dir) : WORKSPACE_ROOT
    if (!fs.existsSync(absDir)) continue
    let files: string[]
    try {
      files = fs.readdirSync(absDir).filter(f => f.endsWith('.md'))
    } catch {
      continue
    }
    for (const file of files) {
      const absPath = path.join(absDir, file)
      let stat: fs.Stats
      try { stat = fs.statSync(absPath) } catch { continue }
      const relPath = path.relative(WORKSPACE_ROOT, absPath)
      docs.push({
        id: relPath.replace(/\//g, '_').replace(/\.md$/, ''),
        title: file.replace('.md', '').replace(/[-_]/g, ' '),
        path: absPath,
        relativePath: relPath,
        folder: dir || 'workspace',
        type: detectType(file),
        size: stat.size,
        updated_at: stat.mtime.toISOString(),
      })
    }
  }
  return docs.sort((a, b) => b.updated_at.localeCompare(a.updated_at))
}

export async function readDocContent(relPath: string): Promise<string> {
  const absPath = path.join(WORKSPACE_ROOT, relPath)
  if (!fs.existsSync(absPath)) return '*File not found.*'
  return fs.readFileSync(absPath, 'utf8')
}

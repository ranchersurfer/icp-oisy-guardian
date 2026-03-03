import fs from 'fs'
import path from 'path'
import { workspacePath } from './workspace'

export interface MemoryCard {
  id: string
  date: string
  section: string
  content: string
  summary: string
  tags: string[]
  lineIndex: number
}

export interface MemoryDay {
  date: string
  filename: string
  cardCount: number
  wordCount: number
  sizeKb: number
}

export interface MemoryFull {
  date: string
  content: string
  wordCount: number
  sizeKb: number
}

function extractTags(text: string): string[] {
  const tags = new Set<string>()
  const patterns = [
    /project:[a-z0-9-]+/gi,
    /topic:[a-z0-9-]+/gi,
    /agent:[a-z0-9-]+/gi,
  ]
  for (const p of patterns) {
    const matches = text.match(p) || []
    matches.forEach(m => tags.add(m.toLowerCase()))
  }
  // keyword-based tags
  if (/guardian/i.test(text)) tags.add('project:guardian')
  if (/security|token|exposed|breach/i.test(text)) tags.add('topic:security')
  if (/dream.?cycle/i.test(text)) tags.add('agent:dream-cycle')
  if (/prospector/i.test(text)) tags.add('agent:prospector')
  if (/discord/i.test(text)) tags.add('topic:discord')
  if (/github|git/i.test(text)) tags.add('topic:github')
  return Array.from(tags)
}

export async function readMemoryDays(): Promise<MemoryDay[]> {
  const dir = workspacePath('memory')
  if (!fs.existsSync(dir)) return []
  const files = fs.readdirSync(dir)
    .filter(f => f.endsWith('.md') && /^\d{4}-\d{2}-\d{2}/.test(f))
    .sort()
    .reverse()
  return files.map(f => {
    const filePath = path.join(dir, f)
    const cards = parseMemoryFile(filePath)
    const content = fs.existsSync(filePath) ? fs.readFileSync(filePath, 'utf8') : ''
    const sizeKb = Math.round((fs.existsSync(filePath) ? fs.statSync(filePath).size : 0) / 1024 * 10) / 10
    const wordCount = content.split(/\s+/).filter(Boolean).length
    return { date: f.replace('.md', ''), filename: f, cardCount: cards.length, wordCount, sizeKb }
  })
}

export function parseMemoryFile(filePath: string): MemoryCard[] {
  if (!fs.existsSync(filePath)) return []
  const content = fs.readFileSync(filePath, 'utf8')
  const lines = content.split('\n')
  const cards: MemoryCard[] = []
  let currentSection = 'General'
  let cardIndex = 0

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim()
    if (!line) continue
    if (line.startsWith('## ')) {
      currentSection = line.replace(/^## /, '')
      continue
    }
    if (line.startsWith('# ')) continue // skip title
    if (line.startsWith('#')) continue

    const summary = line.replace(/^\*+/, '').replace(/\*+$/, '').slice(0, 120)
    if (summary.length < 3) continue

    const dateMatch = path.basename(filePath).match(/(\d{4}-\d{2}-\d{2})/)
    const date = dateMatch ? dateMatch[1] : 'unknown'

    cards.push({
      id: `${date}-${cardIndex++}`,
      date,
      section: currentSection,
      content: line,
      summary,
      tags: extractTags(line + ' ' + currentSection),
      lineIndex: i,
    })
  }
  return cards
}

export async function readMemoryCards(date: string): Promise<MemoryCard[]> {
  const dir = workspacePath('memory')
  // find file matching date
  const files = fs.existsSync(dir) ? fs.readdirSync(dir) : []
  const file = files.find(f => f.startsWith(date))
  if (!file) return []
  return parseMemoryFile(path.join(dir, file))
}

export async function readLongTermMemory(): Promise<string> {
  const p = workspacePath('MEMORY.md')
  if (!fs.existsSync(p)) return '*No long-term memory file found.*'
  return fs.readFileSync(p, 'utf8')
}

export async function readFullMemory(date: string): Promise<MemoryFull | null> {
  const dir = workspacePath('memory')
  const files = fs.existsSync(dir) ? fs.readdirSync(dir) : []
  const file = files.find(f => f.startsWith(date))
  if (!file) return null
  const filePath = path.join(dir, file)
  const content = fs.readFileSync(filePath, 'utf8')
  const stat = fs.statSync(filePath)
  const sizeKb = Math.round(stat.size / 1024 * 10) / 10
  const wordCount = content.split(/\s+/).filter(Boolean).length
  return { date, content, wordCount, sizeKb }
}

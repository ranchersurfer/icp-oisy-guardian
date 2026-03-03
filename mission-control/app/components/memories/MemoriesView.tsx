'use client'

import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { BookOpen, Calendar } from 'lucide-react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

interface MemoryDay {
  date: string
  filename: string
  cardCount: number
  wordCount: number
  sizeKb: number
}

interface MemoryFull {
  date: string
  content: string
  wordCount: number
  sizeKb: number
}

const DAY_NAMES = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday']

function getDayName(dateStr: string): string {
  const [y, m, d] = dateStr.split('-').map(Number)
  const dt = new Date(y, m - 1, d)
  return DAY_NAMES[dt.getDay()]
}

async function fetchDays(): Promise<MemoryDay[]> {
  const res = await fetch('/api/memories')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

async function fetchLongTermContent(): Promise<MemoryFull> {
  const res = await fetch('/api/memories?longterm=1')
  if (!res.ok) throw new Error('Failed')
  const data = await res.json()
  const content: string = data.content || ''
  const wordCount = content.split(/\s+/).filter(Boolean).length
  const sizeKb = Math.round(new Blob([content]).size / 1024 * 10) / 10
  return { date: 'MEMORY.md', content, wordCount, sizeKb }
}

async function fetchFullMemory(date: string): Promise<MemoryFull> {
  const res = await fetch(`/api/memories/${date}`)
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

// Custom markdown components for journal-style rendering
const markdownComponents = {
  h1: ({ children }: { children?: React.ReactNode }) => (
    <h1 className="text-xl font-bold text-white mb-4 mt-6">{children}</h1>
  ),
  h2: ({ children }: { children?: React.ReactNode }) => (
    <h2 className="flex items-center gap-2 text-base font-bold text-white mt-6 mb-3 pl-3 border-l-2 border-indigo-400">
      <span className="text-indigo-400">◆</span>
      {children}
    </h2>
  ),
  h3: ({ children }: { children?: React.ReactNode }) => (
    <h3 className="text-sm font-semibold text-white/80 mt-4 mb-2">{children}</h3>
  ),
  p: ({ children }: { children?: React.ReactNode }) => (
    <p className="text-white/70 text-sm leading-relaxed mb-3">{children}</p>
  ),
  strong: ({ children }: { children?: React.ReactNode }) => (
    <strong className="text-white font-semibold">{children}</strong>
  ),
  em: ({ children }: { children?: React.ReactNode }) => (
    <em className="text-white/60 italic">{children}</em>
  ),
  li: ({ children }: { children?: React.ReactNode }) => (
    <li className="text-white/70 text-sm leading-relaxed mb-1 flex gap-2">
      <span className="text-indigo-400 mt-0.5 shrink-0">•</span>
      <span>{children}</span>
    </li>
  ),
  ul: ({ children }: { children?: React.ReactNode }) => (
    <ul className="mb-3 space-y-0.5 list-none pl-0">{children}</ul>
  ),
  ol: ({ children }: { children?: React.ReactNode }) => (
    <ol className="mb-3 space-y-1 list-decimal pl-5 text-white/70 text-sm">{children}</ol>
  ),
  code: ({ children }: { children?: React.ReactNode }) => (
    <code className="bg-white/10 text-indigo-300 px-1.5 py-0.5 rounded text-xs font-mono">{children}</code>
  ),
  pre: ({ children }: { children?: React.ReactNode }) => (
    <pre className="bg-white/5 border border-white/10 rounded-lg p-4 overflow-auto mb-3 text-xs font-mono text-white/70">{children}</pre>
  ),
  hr: () => <hr className="border-white/10 my-4" />,
  blockquote: ({ children }: { children?: React.ReactNode }) => (
    <blockquote className="border-l-2 border-indigo-500/50 pl-4 italic text-white/50 my-3">{children}</blockquote>
  ),
}

function JournalContent({ data }: { data: MemoryFull }) {
  const dayLabel = data.date === 'MEMORY.md'
    ? 'Long-Term Memory'
    : `${data.date} — ${getDayName(data.date)}`

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="shrink-0 mb-4 pb-4 border-b border-white/10">
        <div className="text-white font-semibold text-base">{dayLabel}</div>
        <div className="text-white/40 text-xs mt-1">
          {data.sizeKb} KB • {data.wordCount.toLocaleString()} words
        </div>
      </div>
      {/* Scrollable content */}
      <div className="flex-1 overflow-auto pr-1">
        <ReactMarkdown
          remarkPlugins={[remarkGfm]}
          components={markdownComponents as Record<string, React.ElementType>}
        >
          {data.content}
        </ReactMarkdown>
      </div>
    </div>
  )
}

export default function MemoriesView() {
  const [selectedDate, setSelectedDate] = useState<string | null>(null)
  const [showLongTerm, setShowLongTerm] = useState(false)

  const { data: days = [] } = useQuery({
    queryKey: ['memory-days'],
    queryFn: fetchDays,
    refetchInterval: 15000,
  })

  const effectiveDate = selectedDate || days[0]?.date || null

  const { data: fullMemory, isLoading: loadingFull } = useQuery({
    queryKey: ['memory-full', effectiveDate],
    queryFn: () => fetchFullMemory(effectiveDate!),
    enabled: !!effectiveDate && !showLongTerm,
    refetchInterval: 15000,
  })

  const { data: ltMemory, isLoading: loadingLt } = useQuery({
    queryKey: ['memory-longterm'],
    queryFn: fetchLongTermContent,
    enabled: showLongTerm,
    refetchInterval: 15000,
  })

  const activeData = showLongTerm ? ltMemory : fullMemory
  const isLoading = showLongTerm ? loadingLt : loadingFull

  return (
    <div className="flex gap-4 h-full min-h-0">
      {/* Left sidebar */}
      <div className="w-52 shrink-0 flex flex-col gap-1 overflow-auto">
        <div className="text-white/30 text-xs uppercase font-semibold mb-2 px-2 shrink-0">Memory</div>

        {/* Long-term memory special entry */}
        <button
          onClick={() => { setShowLongTerm(true); setSelectedDate(null) }}
          className={`text-left px-3 py-2.5 rounded-lg text-sm transition-colors flex items-center gap-2 ${
            showLongTerm
              ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
              : 'text-white/50 hover:bg-white/5 hover:text-white/80'
          }`}
        >
          <BookOpen size={13} className="shrink-0" />
          <div>
            <div className="font-medium">Long-Term Memory</div>
            <div className="text-xs opacity-60">MEMORY.md</div>
          </div>
        </button>

        <div className="flex items-center gap-2 mt-2 mb-1 px-2">
          <Calendar size={11} className="text-white/30" />
          <div className="text-white/30 text-xs uppercase font-semibold">Daily Logs</div>
        </div>

        {days.map(d => (
          <button
            key={d.date}
            onClick={() => { setSelectedDate(d.date); setShowLongTerm(false) }}
            className={`text-left px-3 py-2.5 rounded-lg text-sm transition-colors ${
              !showLongTerm && (selectedDate || days[0]?.date) === d.date
                ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                : 'text-white/50 hover:bg-white/5 hover:text-white/80'
            }`}
          >
            <div className="font-medium">{d.date}</div>
            <div className="text-xs opacity-60">{d.sizeKb} KB • {d.wordCount} words</div>
          </button>
        ))}
        {days.length === 0 && (
          <div className="text-white/20 text-xs px-2">No memory files found</div>
        )}
      </div>

      {/* Right panel */}
      <div className="flex-1 min-w-0 bg-white/[0.02] border border-white/10 rounded-lg p-5 overflow-hidden flex flex-col">
        {isLoading ? (
          <div className="text-white/30 text-sm">Loading…</div>
        ) : activeData ? (
          <JournalContent data={activeData} />
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-white/20 text-sm gap-2">
            <BookOpen size={32} className="opacity-30" />
            <span>Select a day to read</span>
          </div>
        )}
      </div>
    </div>
  )
}

'use client'

import { useState, useMemo } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Search, Calendar, Tag, BookOpen, Clock } from 'lucide-react'
import ReactMarkdown from 'react-markdown'

interface MemoryDay {
  date: string
  filename: string
  cardCount: number
}

interface MemoryCard {
  id: string
  date: string
  section: string
  content: string
  summary: string
  tags: string[]
  lineIndex: number
}

async function fetchDays(): Promise<MemoryDay[]> {
  const res = await fetch('/api/memories')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

async function fetchCards(date: string): Promise<MemoryCard[]> {
  const res = await fetch(`/api/memories?date=${date}`)
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

async function fetchLongTerm(): Promise<{ content: string }> {
  const res = await fetch('/api/memories?longterm=1')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

const TAG_COLORS: Record<string, string> = {
  'project:guardian': 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30',
  'topic:security': 'bg-red-500/20 text-red-300 border-red-500/30',
  'topic:discord': 'bg-purple-500/20 text-purple-300 border-purple-500/30',
  'topic:github': 'bg-gray-500/20 text-gray-300 border-gray-500/30',
  'agent:dream-cycle': 'bg-blue-500/20 text-blue-300 border-blue-500/30',
  'agent:prospector': 'bg-amber-500/20 text-amber-300 border-amber-500/30',
}

function tagColor(tag: string) {
  return TAG_COLORS[tag] || 'bg-white/10 text-white/50 border-white/20'
}

export default function MemoriesView() {
  const [activeTab, setActiveTab] = useState<'daily' | 'longterm'>('daily')
  const [selectedDate, setSelectedDate] = useState<string | null>(null)
  const [search, setSearch] = useState('')
  const [activeFilter, setActiveFilter] = useState('all')

  const { data: days = [] } = useQuery({
    queryKey: ['memory-days'],
    queryFn: fetchDays,
    refetchInterval: 15000,
  })

  const { data: cards = [] } = useQuery({
    queryKey: ['memory-cards', selectedDate],
    queryFn: () => fetchCards(selectedDate!),
    refetchInterval: 15000,
    enabled: !!selectedDate,
  })

  const { data: ltData } = useQuery({
    queryKey: ['memory-longterm'],
    queryFn: fetchLongTerm,
    refetchInterval: 15000,
    enabled: activeTab === 'longterm',
  })

  // auto-select first day
  const currentDate = selectedDate || days[0]?.date || null

  const allTags = useMemo(() => {
    const t = new Set<string>()
    cards.forEach(c => c.tags.forEach(tag => t.add(tag)))
    return Array.from(t).slice(0, 8)
  }, [cards])

  const filteredCards = useMemo(() => {
    let result = cards
    if (activeFilter !== 'all') result = result.filter(c => c.tags.includes(activeFilter))
    if (search.trim()) {
      const q = search.toLowerCase()
      result = result.filter(c =>
        c.summary.toLowerCase().includes(q) ||
        c.section.toLowerCase().includes(q) ||
        c.tags.some(t => t.includes(q))
      )
    }
    return result
  }, [cards, search, activeFilter])

  // group by section
  const grouped = useMemo(() => {
    const map = new Map<string, MemoryCard[]>()
    filteredCards.forEach(c => {
      if (!map.has(c.section)) map.set(c.section, [])
      map.get(c.section)!.push(c)
    })
    return map
  }, [filteredCards])

  return (
    <div className="flex flex-col h-full gap-4">
      {/* Tabs */}
      <div className="flex items-center gap-1 border-b border-white/10 pb-3">
        <button
          onClick={() => setActiveTab('daily')}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
            activeTab === 'daily' ? 'bg-white/10 text-white' : 'text-white/40 hover:text-white/70'
          }`}
        >
          <Calendar size={14} /> Daily Logs
        </button>
        <button
          onClick={() => setActiveTab('longterm')}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
            activeTab === 'longterm' ? 'bg-white/10 text-white' : 'text-white/40 hover:text-white/70'
          }`}
        >
          <BookOpen size={14} /> Long-term Memory
        </button>
      </div>

      {activeTab === 'longterm' ? (
        <div className="flex-1 overflow-auto rounded-lg border border-white/10 bg-white/[0.02] p-6">
          {ltData ? (
            <div className="prose prose-invert prose-sm max-w-none">
              <ReactMarkdown>{ltData.content}</ReactMarkdown>
            </div>
          ) : (
            <div className="text-white/30 text-sm">Loading long-term memory…</div>
          )}
        </div>
      ) : (
        <div className="flex gap-4 flex-1 min-h-0">
          {/* Left: day list */}
          <div className="w-48 shrink-0 flex flex-col gap-1 overflow-auto">
            <div className="text-white/30 text-xs uppercase font-semibold mb-2 px-2">Days</div>
            {days.map(d => (
              <button
                key={d.date}
                onClick={() => setSelectedDate(d.date)}
                className={`text-left px-3 py-2 rounded-lg text-sm transition-colors ${
                  (selectedDate || days[0]?.date) === d.date
                    ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30'
                    : 'text-white/50 hover:bg-white/5 hover:text-white/80'
                }`}
              >
                <div className="font-medium">{d.date}</div>
                <div className="text-xs opacity-60">{d.cardCount} entries</div>
              </button>
            ))}
            {days.length === 0 && (
              <div className="text-white/20 text-xs px-2">No memory files found</div>
            )}
          </div>

          {/* Right: cards */}
          <div className="flex-1 flex flex-col gap-3 min-w-0 overflow-auto">
            {/* Search + filter bar */}
            <div className="flex items-center gap-2 flex-wrap">
              <div className="relative flex-1 min-w-48">
                <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-white/30" />
                <input
                  type="text"
                  value={search}
                  onChange={e => setSearch(e.target.value)}
                  placeholder="Search memories…"
                  className="w-full bg-white/5 border border-white/10 rounded-lg pl-8 pr-3 py-1.5 text-sm text-white placeholder-white/30 focus:outline-none focus:border-indigo-500/50"
                />
              </div>
              <div className="flex items-center gap-1 flex-wrap">
                <button
                  onClick={() => setActiveFilter('all')}
                  className={`px-2.5 py-1 rounded-md text-xs transition-colors ${
                    activeFilter === 'all' ? 'bg-white/15 text-white' : 'text-white/40 hover:text-white/70'
                  }`}
                >
                  All
                </button>
                {allTags.map(tag => (
                  <button
                    key={tag}
                    onClick={() => setActiveFilter(tag === activeFilter ? 'all' : tag)}
                    className={`px-2.5 py-1 rounded-md text-xs border transition-colors ${
                      activeFilter === tag
                        ? tagColor(tag) + ' opacity-100'
                        : 'border-white/10 text-white/40 hover:text-white/70'
                    }`}
                  >
                    {tag}
                  </button>
                ))}
              </div>
            </div>

            {/* Memory cards grouped by section */}
            {grouped.size === 0 ? (
              <div className="flex items-center justify-center h-32 text-white/20 text-sm">
                {currentDate ? 'No memories match filters' : 'Select a day'}
              </div>
            ) : (
              Array.from(grouped.entries()).map(([section, sCards]) => (
                <div key={section}>
                  <div className="flex items-center gap-2 mb-2">
                    <span className="text-white/60 text-xs font-semibold uppercase">{section}</span>
                    <div className="flex-1 h-px bg-white/10" />
                  </div>
                  <div className="flex flex-col gap-2">
                    {sCards.map(card => (
                      <div
                        key={card.id}
                        className="bg-white/[0.03] border border-white/10 rounded-lg p-3 hover:border-white/20 transition-colors"
                      >
                        <p className="text-white/80 text-sm leading-relaxed">{card.summary}</p>
                        {card.tags.length > 0 && (
                          <div className="flex flex-wrap gap-1 mt-2">
                            {card.tags.map(tag => (
                              <span
                                key={tag}
                                className={`text-xs px-1.5 py-0.5 rounded border ${tagColor(tag)}`}
                              >
                                {tag}
                              </span>
                            ))}
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  )
}

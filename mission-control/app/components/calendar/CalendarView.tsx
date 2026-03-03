'use client'

import { useQuery } from '@tanstack/react-query'
import { useState } from 'react'
import {
  ChevronLeft,
  ChevronRight,
  Calendar as CalendarIcon,
  List,
  CheckCircle2,
  Clock,
  AlertCircle,
} from 'lucide-react'

interface ScheduledItem {
  id: string
  title: string
  description: string
  schedule: string
  schedule_description: string
  agent_id: string
  type: 'cron' | 'one_off'
  next_run_at: string | null
  last_run_at: string | null
  source_file: string | null
  status: 'active' | 'planned' | 'error'
}

const AGENT_COLORS: Record<string, { bg: string; text: string; dot: string }> = {
  'dream-cycle': { bg: 'bg-blue-500/20', text: 'text-blue-300', dot: 'bg-blue-400' },
  'mission-control': { bg: 'bg-indigo-500/20', text: 'text-indigo-300', dot: 'bg-indigo-400' },
  'guardian-dev': { bg: 'bg-purple-500/20', text: 'text-purple-300', dot: 'bg-purple-400' },
  prospector: { bg: 'bg-orange-500/20', text: 'text-orange-300', dot: 'bg-orange-400' },
  creator: { bg: 'bg-yellow-500/20', text: 'text-yellow-300', dot: 'bg-yellow-400' },
}

const DEFAULT_COLOR = { bg: 'bg-slate-500/20', text: 'text-slate-300', dot: 'bg-slate-400' }

function agentColor(agentId: string) {
  return AGENT_COLORS[agentId] ?? DEFAULT_COLOR
}

const AGENT_LABELS: Record<string, string> = {
  'dream-cycle': 'Dream Cycle',
  'mission-control': 'Mission Control',
  'guardian-dev': 'Guardian-Dev',
  prospector: 'Prospector',
  creator: 'Creator',
  main: 'Mission Control',
}

function agentLabel(id: string) {
  return AGENT_LABELS[id] ?? id
}

function parseEventDate(item: ScheduledItem): Date | null {
  if (item.next_run_at) return new Date(item.next_run_at)
  // For ISO one_off schedules
  if (item.type === 'one_off' && item.schedule) {
    const d = new Date(item.schedule)
    if (!isNaN(d.getTime())) return d
  }
  return null
}

function formatTime(dateStr: string | null): string {
  if (!dateStr) return '—'
  const d = new Date(dateStr)
  if (isNaN(d.getTime())) return '—'
  return d.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: true })
}

function formatDateTime(dateStr: string | null): string {
  if (!dateStr) return '—'
  const d = new Date(dateStr)
  if (isNaN(d.getTime())) return '—'
  return d.toLocaleString('en-US', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })
}

const STATUS_ICONS: Record<string, React.ReactNode> = {
  active: <CheckCircle2 size={12} className="text-emerald-400" />,
  planned: <Clock size={12} className="text-yellow-400" />,
  error: <AlertCircle size={12} className="text-red-400" />,
}

const STATUS_LABELS: Record<string, string> = {
  active: 'Active',
  planned: 'Planned',
  error: 'Error',
}

// ── Month Calendar ──────────────────────────────────────────────────────────

function MonthCalendar({ items, year, month }: { items: ScheduledItem[]; year: number; month: number }) {
  const firstDay = new Date(year, month, 1).getDay()
  const daysInMonth = new Date(year, month + 1, 0).getDate()
  const today = new Date()

  // Map items to their dates in this month
  const eventsByDay: Record<number, ScheduledItem[]> = {}

  for (const item of items) {
    const d = parseEventDate(item)
    if (d && d.getFullYear() === year && d.getMonth() === month) {
      const day = d.getDate()
      if (!eventsByDay[day]) eventsByDay[day] = []
      eventsByDay[day].push(item)
    }
    // For cron jobs (active), also show them on their scheduled time this month
    if (item.type === 'cron' && item.status === 'active') {
      // Parse schedule to get day-of-month occurrences (simplified: show on day 1 if "* * *" pattern)
      // We'll show a recurring indicator on each appropriate day
      // For "0 1 * * *" → show on every day of month
      const parts = item.schedule.split(' ')
      if (parts.length === 5) {
        const [, , dayOfMonth] = parts
        if (dayOfMonth === '*') {
          // Daily cron – show on all days
          for (let day = 1; day <= daysInMonth; day++) {
            if (!eventsByDay[day]) eventsByDay[day] = []
            // Only add once
            if (!eventsByDay[day].find(e => e.id === item.id)) {
              eventsByDay[day].push(item)
            }
          }
        }
      }
    }
  }

  const cells: (number | null)[] = []
  for (let i = 0; i < firstDay; i++) cells.push(null)
  for (let d = 1; d <= daysInMonth; d++) cells.push(d)

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']

  return (
    <div>
      <div className="grid grid-cols-7 mb-1">
        {dayNames.map(d => (
          <div key={d} className="text-center text-xs text-white/30 py-1 font-medium">{d}</div>
        ))}
      </div>
      <div className="grid grid-cols-7 gap-px bg-white/5 rounded-lg overflow-hidden">
        {cells.map((day, idx) => {
          const isToday =
            day !== null &&
            today.getFullYear() === year &&
            today.getMonth() === month &&
            today.getDate() === day
          const events = day ? (eventsByDay[day] ?? []) : []
          // Deduplicate recurring events – show max 1 dot per agent
          const uniqueAgents = Array.from(new Set(events.map(e => e.agent_id)))
          return (
            <div
              key={idx}
              className={`bg-[#1a1a2e] min-h-[72px] p-1.5 ${day ? 'hover:bg-[#1f1f38]' : ''} transition-colors`}
            >
              {day && (
                <>
                  <span className={`text-xs font-medium mb-1 inline-flex items-center justify-center w-5 h-5 rounded-full ${
                    isToday ? 'bg-indigo-500 text-white' : 'text-white/50'
                  }`}>
                    {day}
                  </span>
                  <div className="flex flex-col gap-0.5 mt-0.5">
                    {uniqueAgents.slice(0, 3).map(agentId => {
                      const agentEvents = events.filter(e => e.agent_id === agentId)
                      const first = agentEvents[0]
                      const color = agentColor(agentId)
                      return (
                        <div
                          key={agentId}
                          title={`${first.title} · ${agentLabel(agentId)} · ${first.status}`}
                          className={`text-[10px] px-1 py-0.5 rounded truncate ${color.bg} ${color.text}`}
                        >
                          {first.title.length > 14 ? first.title.slice(0, 13) + '…' : first.title}
                        </div>
                      )
                    })}
                    {uniqueAgents.length > 3 && (
                      <span className="text-[10px] text-white/30">+{uniqueAgents.length - 3} more</span>
                    )}
                  </div>
                </>
              )}
            </div>
          )
        })}
      </div>
    </div>
  )
}

// ── Week Calendar ───────────────────────────────────────────────────────────

function WeekCalendar({ items, weekStart }: { items: ScheduledItem[]; weekStart: Date }) {
  const days: Date[] = []
  for (let i = 0; i < 7; i++) {
    const d = new Date(weekStart)
    d.setDate(weekStart.getDate() + i)
    days.push(d)
  }

  const today = new Date()

  return (
    <div className="grid grid-cols-7 gap-px bg-white/5 rounded-lg overflow-hidden">
      {days.map((day, idx) => {
        const isToday = day.toDateString() === today.toDateString()
        const dayEvents = items.filter(item => {
          const d = parseEventDate(item)
          if (d) return d.toDateString() === day.toDateString()
          // Daily crons
          if (item.type === 'cron' && item.status === 'active') {
            const parts = item.schedule.split(' ')
            if (parts.length === 5 && parts[2] === '*') return true
          }
          return false
        })
        return (
          <div key={idx} className="bg-[#1a1a2e] min-h-[160px] p-2 hover:bg-[#1f1f38] transition-colors">
            <div className={`text-xs font-medium mb-2 flex flex-col items-center ${isToday ? 'text-white' : 'text-white/40'}`}>
              <span>{['Sun','Mon','Tue','Wed','Thu','Fri','Sat'][day.getDay()]}</span>
              <span className={`mt-0.5 w-6 h-6 flex items-center justify-center rounded-full text-xs ${isToday ? 'bg-indigo-500 text-white' : ''}`}>
                {day.getDate()}
              </span>
            </div>
            <div className="flex flex-col gap-1">
              {dayEvents.map(item => {
                const color = agentColor(item.agent_id)
                return (
                  <div
                    key={item.id}
                    title={`${item.title} · ${item.schedule_description} · ${item.status}`}
                    className={`text-[10px] px-1.5 py-1 rounded ${color.bg} ${color.text} leading-tight`}
                  >
                    <div className="font-medium truncate">{item.title}</div>
                    <div className="opacity-60 truncate">{item.schedule_description}</div>
                  </div>
                )
              })}
            </div>
          </div>
        )
      })}
    </div>
  )
}

// ── Main Component ──────────────────────────────────────────────────────────

export default function CalendarView() {
  const [viewMode, setViewMode] = useState<'month' | 'week'>('month')
  const [currentDate, setCurrentDate] = useState(new Date(2026, 2, 1)) // March 2026

  const { data: items = [], isLoading } = useQuery<ScheduledItem[]>({
    queryKey: ['schedule'],
    queryFn: () => fetch('/api/schedule').then(r => r.json()),
    refetchInterval: 15000,
  })

  const year = currentDate.getFullYear()
  const month = currentDate.getMonth()

  const monthName = currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })
  const weekStart = new Date(currentDate)
  weekStart.setDate(currentDate.getDate() - currentDate.getDay())
  const weekEnd = new Date(weekStart)
  weekEnd.setDate(weekStart.getDate() + 6)
  const weekLabel = `${weekStart.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} – ${weekEnd.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}`

  function prev() {
    if (viewMode === 'month') {
      setCurrentDate(new Date(year, month - 1, 1))
    } else {
      const d = new Date(currentDate)
      d.setDate(d.getDate() - 7)
      setCurrentDate(d)
    }
  }
  function next() {
    if (viewMode === 'month') {
      setCurrentDate(new Date(year, month + 1, 1))
    } else {
      const d = new Date(currentDate)
      d.setDate(d.getDate() + 7)
      setCurrentDate(d)
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-semibold text-white">Calendar</h1>
          <p className="text-sm text-white/40 mt-0.5">Cron jobs & scheduled tasks</p>
        </div>

        <div className="flex items-center gap-3">
          {/* Agent legend */}
          <div className="hidden lg:flex items-center gap-3">
            {Object.entries(AGENT_COLORS).map(([id, c]) => (
              <div key={id} className="flex items-center gap-1.5">
                <span className={`w-2 h-2 rounded-full ${c.dot}`} />
                <span className="text-xs text-white/50">{agentLabel(id)}</span>
              </div>
            ))}
          </div>

          {/* View toggle */}
          <div className="flex items-center bg-white/5 rounded-md p-0.5">
            <button
              onClick={() => setViewMode('month')}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded text-xs transition-colors ${viewMode === 'month' ? 'bg-indigo-600/50 text-indigo-200' : 'text-white/50 hover:text-white/80'}`}
            >
              <CalendarIcon size={13} />
              Month
            </button>
            <button
              onClick={() => setViewMode('week')}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded text-xs transition-colors ${viewMode === 'week' ? 'bg-indigo-600/50 text-indigo-200' : 'text-white/50 hover:text-white/80'}`}
            >
              <List size={13} />
              Week
            </button>
          </div>
        </div>
      </div>

      {/* Calendar navigation */}
      <div className="flex items-center gap-3">
        <button onClick={prev} className="p-1.5 rounded-md hover:bg-white/10 text-white/60 hover:text-white transition-colors">
          <ChevronLeft size={16} />
        </button>
        <span className="text-sm font-medium text-white min-w-[200px]">
          {viewMode === 'month' ? monthName : weekLabel}
        </span>
        <button onClick={next} className="p-1.5 rounded-md hover:bg-white/10 text-white/60 hover:text-white transition-colors">
          <ChevronRight size={16} />
        </button>
      </div>

      {/* Calendar grid */}
      {isLoading ? (
        <div className="h-64 flex items-center justify-center text-white/30 text-sm">Loading schedule…</div>
      ) : viewMode === 'month' ? (
        <MonthCalendar items={items} year={year} month={month} />
      ) : (
        <WeekCalendar items={items} weekStart={weekStart} />
      )}

      {/* List view */}
      <div>
        <h2 className="text-sm font-semibold text-white/70 mb-3 uppercase tracking-wide">All Scheduled Items</h2>
        <div className="rounded-lg border border-white/10 overflow-hidden">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-white/10 bg-white/5">
                <th className="text-left px-4 py-2.5 text-white/40 font-medium text-xs">Title</th>
                <th className="text-left px-4 py-2.5 text-white/40 font-medium text-xs">Agent</th>
                <th className="text-left px-4 py-2.5 text-white/40 font-medium text-xs">Schedule</th>
                <th className="text-left px-4 py-2.5 text-white/40 font-medium text-xs">Next Run</th>
                <th className="text-left px-4 py-2.5 text-white/40 font-medium text-xs">Last Run</th>
                <th className="text-left px-4 py-2.5 text-white/40 font-medium text-xs">Status</th>
              </tr>
            </thead>
            <tbody>
              {items.map(item => {
                const color = agentColor(item.agent_id)
                return (
                  <tr key={item.id} className="border-b border-white/5 hover:bg-white/5 transition-colors">
                    <td className="px-4 py-3">
                      <div className="font-medium text-white/90">{item.title}</div>
                      <div className="text-xs text-white/35 mt-0.5 truncate max-w-xs">{item.description}</div>
                    </td>
                    <td className="px-4 py-3">
                      <span className={`inline-flex items-center gap-1.5 text-xs px-2 py-0.5 rounded-full ${color.bg} ${color.text}`}>
                        <span className={`w-1.5 h-1.5 rounded-full ${color.dot}`} />
                        {agentLabel(item.agent_id)}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-white/60 text-xs font-mono">{item.schedule_description || item.schedule}</td>
                    <td className="px-4 py-3 text-white/60 text-xs">{formatDateTime(item.next_run_at)}</td>
                    <td className="px-4 py-3 text-white/60 text-xs">{formatDateTime(item.last_run_at)}</td>
                    <td className="px-4 py-3">
                      <span className="inline-flex items-center gap-1 text-xs text-white/60">
                        {STATUS_ICONS[item.status]}
                        {STATUS_LABELS[item.status] ?? item.status}
                      </span>
                    </td>
                  </tr>
                )
              })}
              {items.length === 0 && (
                <tr>
                  <td colSpan={6} className="px-4 py-8 text-center text-white/30 text-sm">No scheduled items found</td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  )
}

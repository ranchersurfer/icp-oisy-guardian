'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import {
  LayoutGrid,
  Calendar,
  FolderOpen,
  Brain,
  FileText,
  Users,
} from 'lucide-react'

const NAV_ITEMS = [
  { href: '/tasks', icon: LayoutGrid, label: 'Task Board' },
  { href: '/calendar', icon: Calendar, label: 'Calendar' },
  { href: '/projects', icon: FolderOpen, label: 'Projects' },
  { href: '/memories', icon: Brain, label: 'Memories' },
  { href: '/docs', icon: FileText, label: 'Docs' },
  { href: '/team', icon: Users, label: 'Team' },
]

export default function Sidebar() {
  const pathname = usePathname()

  return (
    <aside className="flex flex-col w-56 min-h-screen bg-[#1a1a2e] border-r border-white/10 py-4 px-3 shrink-0">
      {/* App name */}
      <div className="mb-6 px-2">
        <span className="text-white font-semibold text-sm tracking-wide">Mission Control</span>
        <p className="text-white/40 text-xs mt-0.5">Agent Ops Dashboard</p>
      </div>

      {/* Nav */}
      <nav className="flex flex-col gap-0.5 flex-1">
        {NAV_ITEMS.map(({ href, icon: Icon, label }) => {
          const active = pathname.startsWith(href)
          return (
            <Link
              key={href}
              href={href}
              className={`flex items-center gap-2.5 px-2 py-1.5 rounded-md text-sm transition-colors ${
                active
                  ? 'bg-indigo-600/30 text-indigo-300 font-medium'
                  : 'text-white/50 hover:text-white/80 hover:bg-white/5'
              }`}
            >
              <Icon size={16} />
              <span>{label}</span>
            </Link>
          )
        })}
      </nav>

      {/* Footer */}
      <div className="pt-3 border-t border-white/10">
        <p className="text-white/25 text-xs px-2">v0.1.0 · Phase 1</p>
      </div>
    </aside>
  )
}

'use client'

import { useState, useMemo } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Search, FileText, X, ChevronRight } from 'lucide-react'
import ReactMarkdown from 'react-markdown'

interface DocEntry {
  id: string
  title: string
  path: string
  relativePath: string
  folder: string
  type: 'spec' | 'plan' | 'strategy' | 'log' | 'misc'
  size: number
  updated_at: string
}

const TYPE_COLORS: Record<string, string> = {
  spec: 'bg-indigo-500/20 text-indigo-300 border-indigo-500/30',
  plan: 'bg-blue-500/20 text-blue-300 border-blue-500/30',
  strategy: 'bg-amber-500/20 text-amber-300 border-amber-500/30',
  log: 'bg-green-500/20 text-green-300 border-green-500/30',
  misc: 'bg-white/10 text-white/50 border-white/20',
}

async function fetchDocs(): Promise<DocEntry[]> {
  const res = await fetch('/api/docs')
  if (!res.ok) throw new Error('Failed')
  return res.json()
}

async function fetchDocContent(relPath: string): Promise<string> {
  const res = await fetch(`/api/docs?path=${encodeURIComponent(relPath)}`)
  if (!res.ok) throw new Error('Failed')
  const data = await res.json()
  return data.content
}

function fmtSize(bytes: number) {
  if (bytes < 1024) return `${bytes}B`
  return `${(bytes / 1024).toFixed(1)}KB`
}

function fmtDate(iso: string) {
  return new Date(iso).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
}

export default function DocsView() {
  const [search, setSearch] = useState('')
  const [typeFilter, setTypeFilter] = useState<string>('all')
  const [folderFilter, setFolderFilter] = useState<string>('all')
  const [selectedDoc, setSelectedDoc] = useState<DocEntry | null>(null)
  const [docContent, setDocContent] = useState<string | null>(null)
  const [loadingContent, setLoadingContent] = useState(false)

  const { data: docs = [], isLoading } = useQuery({
    queryKey: ['docs'],
    queryFn: fetchDocs,
  })

  const folders = useMemo(() => Array.from(new Set(docs.map(d => d.folder))).sort(), [docs])
  const types = useMemo(() => Array.from(new Set(docs.map(d => d.type))).sort(), [docs])

  const filtered = useMemo(() => {
    let result = docs
    if (typeFilter !== 'all') result = result.filter(d => d.type === typeFilter)
    if (folderFilter !== 'all') result = result.filter(d => d.folder === folderFilter)
    if (search.trim()) {
      const q = search.toLowerCase()
      result = result.filter(d =>
        d.title.toLowerCase().includes(q) ||
        d.relativePath.toLowerCase().includes(q)
      )
    }
    return result
  }, [docs, search, typeFilter, folderFilter])

  async function openDoc(doc: DocEntry) {
    setSelectedDoc(doc)
    setDocContent(null)
    setLoadingContent(true)
    try {
      const content = await fetchDocContent(doc.relativePath)
      setDocContent(content)
    } catch {
      setDocContent('*Failed to load document.*')
    } finally {
      setLoadingContent(false)
    }
  }

  return (
    <div className="flex gap-4 h-full min-h-0">
      {/* Left: list */}
      <div className={`flex flex-col gap-3 ${selectedDoc ? 'w-96 shrink-0' : 'flex-1'} min-w-0 overflow-auto`}>
        {/* Filters */}
        <div className="flex items-center gap-2 flex-wrap">
          <div className="relative flex-1 min-w-48">
            <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-white/30" />
            <input
              type="text"
              value={search}
              onChange={e => setSearch(e.target.value)}
              placeholder="Search docs…"
              className="w-full bg-white/5 border border-white/10 rounded-lg pl-8 pr-3 py-1.5 text-sm text-white placeholder-white/30 focus:outline-none focus:border-indigo-500/50"
            />
          </div>
          <select
            value={typeFilter}
            onChange={e => setTypeFilter(e.target.value)}
            className="bg-white/5 border border-white/10 rounded-lg px-2 py-1.5 text-sm text-white/70 focus:outline-none"
          >
            <option value="all">All types</option>
            {types.map(t => <option key={t} value={t}>{t}</option>)}
          </select>
          <select
            value={folderFilter}
            onChange={e => setFolderFilter(e.target.value)}
            className="bg-white/5 border border-white/10 rounded-lg px-2 py-1.5 text-sm text-white/70 focus:outline-none"
          >
            <option value="all">All folders</option>
            {folders.map(f => <option key={f} value={f}>{f}</option>)}
          </select>
        </div>

        {/* Doc count */}
        <div className="text-white/30 text-xs">{filtered.length} documents</div>

        {/* List */}
        {isLoading ? (
          <div className="text-white/30 text-sm">Loading docs…</div>
        ) : (
          <div className="flex flex-col gap-1.5">
            {filtered.map(doc => (
              <button
                key={doc.id}
                onClick={() => openDoc(doc)}
                className={`text-left flex items-center gap-3 px-3 py-2.5 rounded-lg border transition-colors ${
                  selectedDoc?.id === doc.id
                    ? 'bg-indigo-500/10 border-indigo-500/30'
                    : 'bg-white/[0.02] border-white/10 hover:border-white/20 hover:bg-white/[0.04]'
                }`}
              >
                <FileText size={14} className="text-white/30 shrink-0" />
                <div className="flex-1 min-w-0">
                  <div className="text-white/80 text-sm font-medium truncate">{doc.title}</div>
                  <div className="text-white/30 text-xs truncate">{doc.relativePath}</div>
                </div>
                <div className="flex flex-col items-end gap-1 shrink-0">
                  <span className={`text-xs px-1.5 py-0.5 rounded border ${TYPE_COLORS[doc.type]}`}>
                    {doc.type}
                  </span>
                  <span className="text-white/25 text-xs">{fmtSize(doc.size)}</span>
                </div>
                <ChevronRight size={12} className="text-white/20 shrink-0" />
              </button>
            ))}
          </div>
        )}
      </div>

      {/* Right: preview panel */}
      {selectedDoc && (
        <div className="flex-1 flex flex-col min-w-0 border border-white/10 rounded-lg bg-white/[0.02] overflow-hidden">
          {/* Header */}
          <div className="flex items-center gap-3 px-4 py-3 border-b border-white/10">
            <div className="flex-1 min-w-0">
              <div className="text-white font-medium text-sm truncate">{selectedDoc.title}</div>
              <div className="text-white/30 text-xs truncate">{selectedDoc.relativePath}</div>
            </div>
            <div className="flex items-center gap-2 shrink-0">
              <span className={`text-xs px-1.5 py-0.5 rounded border ${TYPE_COLORS[selectedDoc.type]}`}>
                {selectedDoc.type}
              </span>
              <span className="text-white/30 text-xs">{fmtDate(selectedDoc.updated_at)}</span>
              <button
                onClick={() => setSelectedDoc(null)}
                className="text-white/30 hover:text-white transition-colors"
              >
                <X size={14} />
              </button>
            </div>
          </div>
          {/* Content */}
          <div className="flex-1 overflow-auto p-5">
            {loadingContent ? (
              <div className="text-white/30 text-sm">Loading…</div>
            ) : docContent ? (
              <div className="prose prose-invert prose-sm max-w-none">
                <ReactMarkdown>{docContent}</ReactMarkdown>
              </div>
            ) : null}
          </div>
        </div>
      )}
    </div>
  )
}

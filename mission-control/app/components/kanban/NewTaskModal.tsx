'use client'

import { useState } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { TaskPriority, TaskStatus } from '@/lib/tasks'

interface NewTaskModalProps {
  open: boolean
  onClose: () => void
  onCreated: () => void
}

export default function NewTaskModal({ open, onClose, onCreated }: NewTaskModalProps) {
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [assignee, setAssignee] = useState('')
  const [priority, setPriority] = useState<TaskPriority>('med')
  const [status, setStatus] = useState<TaskStatus>('backlog')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    if (!title.trim()) { setError('Title is required'); return }
    setLoading(true)
    setError('')
    try {
      const res = await fetch('/api/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title, description, assignee, priority, status }),
      })
      if (!res.ok) throw new Error('Failed to create task')
      setTitle(''); setDescription(''); setAssignee(''); setPriority('med'); setStatus('backlog')
      onCreated()
      onClose()
    } catch (e) {
      setError('Failed to create task')
    } finally {
      setLoading(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={v => !v && onClose()}>
      <DialogContent className="bg-[#1a1a2e] border-white/15 text-white max-w-md">
        <DialogHeader>
          <DialogTitle className="text-white">New Task</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="flex flex-col gap-3 mt-2">
          <div>
            <label className="text-white/60 text-xs mb-1 block">Title *</label>
            <Input
              value={title}
              onChange={e => setTitle(e.target.value)}
              placeholder="Task title..."
              className="bg-white/5 border-white/15 text-white placeholder:text-white/30 focus:border-indigo-500"
            />
          </div>
          <div>
            <label className="text-white/60 text-xs mb-1 block">Description</label>
            <textarea
              value={description}
              onChange={e => setDescription(e.target.value)}
              placeholder="Optional description..."
              rows={3}
              className="w-full bg-white/5 border border-white/15 text-white placeholder:text-white/30 rounded-md px-3 py-2 text-sm focus:outline-none focus:border-indigo-500 resize-none"
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="text-white/60 text-xs mb-1 block">Assignee</label>
              <Input
                value={assignee}
                onChange={e => setAssignee(e.target.value)}
                placeholder="e.g. guardian-dev"
                className="bg-white/5 border-white/15 text-white placeholder:text-white/30 focus:border-indigo-500"
              />
            </div>
            <div>
              <label className="text-white/60 text-xs mb-1 block">Priority</label>
              <select
                value={priority}
                onChange={e => setPriority(e.target.value as TaskPriority)}
                className="w-full bg-white/5 border border-white/15 text-white rounded-md px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
              >
                <option value="low">Low</option>
                <option value="med">Medium</option>
                <option value="high">High</option>
              </select>
            </div>
          </div>
          <div>
            <label className="text-white/60 text-xs mb-1 block">Status</label>
            <select
              value={status}
              onChange={e => setStatus(e.target.value as TaskStatus)}
              className="w-full bg-white/5 border border-white/15 text-white rounded-md px-3 py-2 text-sm focus:outline-none focus:border-indigo-500"
            >
              <option value="backlog">Backlog</option>
              <option value="in_progress">In Progress</option>
              <option value="in_review">In Review</option>
              <option value="done">Done</option>
            </select>
          </div>
          {error && <p className="text-red-400 text-xs">{error}</p>}
          <div className="flex justify-end gap-2 mt-1">
            <Button
              type="button"
              variant="ghost"
              onClick={onClose}
              className="text-white/50 hover:text-white hover:bg-white/10"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={loading}
              className="bg-indigo-600 hover:bg-indigo-500 text-white"
            >
              {loading ? 'Creating...' : 'Create Task'}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}

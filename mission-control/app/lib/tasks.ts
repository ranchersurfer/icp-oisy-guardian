import fs from 'fs/promises'
import { workspacePath } from './workspace'

export type TaskStatus = 'backlog' | 'in_progress' | 'in_review' | 'done'
export type TaskPriority = 'low' | 'med' | 'high'

export interface Task {
  id: string
  title: string
  description?: string
  status: TaskStatus
  assignee?: string
  priority?: TaskPriority
  project_id?: string
  created_at: string
  updated_at: string
}

const TASKS_FILE = workspacePath('tasks.json')

export async function readTasks(): Promise<Task[]> {
  try {
    const raw = await fs.readFile(TASKS_FILE, 'utf-8')
    return JSON.parse(raw) as Task[]
  } catch {
    return []
  }
}

export async function writeTasks(tasks: Task[]): Promise<void> {
  const tmp = TASKS_FILE + '.tmp'
  await fs.writeFile(tmp, JSON.stringify(tasks, null, 2), 'utf-8')
  await fs.rename(tmp, TASKS_FILE)
}

export async function updateTask(id: string, patch: Partial<Task>): Promise<Task | null> {
  const tasks = await readTasks()
  const idx = tasks.findIndex(t => t.id === id)
  if (idx === -1) return null
  tasks[idx] = { ...tasks[idx], ...patch, updated_at: new Date().toISOString() }
  await writeTasks(tasks)
  return tasks[idx]
}

export async function createTask(task: Omit<Task, 'id' | 'created_at' | 'updated_at'>): Promise<Task> {
  const tasks = await readTasks()
  const now = new Date().toISOString()
  const newTask: Task = {
    ...task,
    id: `task-${Date.now()}`,
    created_at: now,
    updated_at: now,
  }
  tasks.push(newTask)
  await writeTasks(tasks)
  return newTask
}

import { NextResponse } from 'next/server'
import { readTasks, createTask } from '@/lib/tasks'

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const assignee = searchParams.get('assignee')
  const status = searchParams.get('status')

  let tasks = await readTasks()

  if (assignee) tasks = tasks.filter(t => t.assignee === assignee)
  if (status) tasks = tasks.filter(t => t.status === status)

  return NextResponse.json(tasks)
}

export async function POST(request: Request) {
  try {
    const body = await request.json()
    const task = await createTask({
      title: body.title,
      description: body.description,
      status: body.status ?? 'backlog',
      assignee: body.assignee,
      priority: body.priority ?? 'med',
      project_id: body.project_id,
    })
    return NextResponse.json(task, { status: 201 })
  } catch (e) {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 })
  }
}

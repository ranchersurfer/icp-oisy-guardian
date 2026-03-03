import { NextResponse } from 'next/server'
import { updateTask } from '@/lib/tasks'

export async function PATCH(
  request: Request,
  { params }: { params: Promise<{ id: string }> }
) {
  const { id } = await params
  try {
    const body = await request.json()
    const updated = await updateTask(id, body)
    if (!updated) return NextResponse.json({ error: 'Task not found' }, { status: 404 })
    return NextResponse.json(updated)
  } catch {
    return NextResponse.json({ error: 'Invalid request' }, { status: 400 })
  }
}

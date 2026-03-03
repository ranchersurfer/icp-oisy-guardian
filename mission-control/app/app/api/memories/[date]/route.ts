import { NextResponse } from 'next/server'
import { readFullMemory } from '@/lib/memories'

export async function GET(
  _request: Request,
  { params }: { params: Promise<{ date: string }> }
) {
  const { date } = await params
  const data = await readFullMemory(date)
  if (!data) return NextResponse.json({ error: 'Not found' }, { status: 404 })
  return NextResponse.json(data)
}

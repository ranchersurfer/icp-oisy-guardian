import { NextResponse } from 'next/server'
import { readAgentsState, readMetrics } from '@/lib/agents'

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const metrics = searchParams.get('metrics')

  if (metrics === '1') {
    const data = await readMetrics()
    return NextResponse.json(data)
  }

  const state = await readAgentsState()
  return NextResponse.json(state)
}

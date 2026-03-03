import { NextResponse } from 'next/server'
import { readMemoryDays, readMemoryCards, readLongTermMemory } from '@/lib/memories'

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const date = searchParams.get('date')
  const longTerm = searchParams.get('longterm')

  if (longTerm === '1') {
    const content = await readLongTermMemory()
    return NextResponse.json({ content })
  }

  if (date) {
    const cards = await readMemoryCards(date)
    return NextResponse.json(cards)
  }

  const days = await readMemoryDays()
  return NextResponse.json(days)
}

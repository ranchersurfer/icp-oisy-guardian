import { NextResponse } from 'next/server'
import { scanDocs, readDocContent } from '@/lib/docs'

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const relPath = searchParams.get('path')

  if (relPath) {
    const content = await readDocContent(relPath)
    return NextResponse.json({ content })
  }

  const docs = await scanDocs()
  return NextResponse.json(docs)
}

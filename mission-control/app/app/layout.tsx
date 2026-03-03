import type { Metadata } from 'next'
import './globals.css'
import AppShell from '@/components/AppShell'
import Providers from '@/components/Providers'

export const metadata: Metadata = {
  title: 'Mission Control',
  description: 'Agent Ops Dashboard',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body className="bg-[#12121f] text-white antialiased">
        <Providers>
          <AppShell>{children}</AppShell>
        </Providers>
      </body>
    </html>
  )
}

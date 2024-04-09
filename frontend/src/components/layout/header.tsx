import Link from 'next/link'
import React from 'react'

type Props = {}

export default function Header({ }: Props) {
  return (
    <header className="sticky top-0 z-50 backdrop-blur-md border-b border-b-slate-200">
      <div className="container h-16 flex items-center justify-between">
        <Link href="/">
          <div className="text-2xl text-slate-800 font-black">魚之魷魂</div>
        </Link>
        <div className="flex gap-6">
          <Link href="/" className="Link">首頁</Link>
          <Link href="/blog" className="Link">文章</Link>
        </div>
      </div>
    </header>
  )
}
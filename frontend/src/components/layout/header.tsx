import React from 'react'

type Props = {}

export default function Header({ }: Props) {
  return (
    <header className="sticky top-0 z-50 backdrop-blur-md border-b border-b-slate-200">
      <div className="container h-16 flex items-center justify-between">
        <a href="/">
          <div className="text-2xl text-slate-800 font-black">魚之魷魂</div>
        </a>
        <div className="flex gap-6">
          <a href="/">首頁</a>
          <a href="/blog">文章</a>
        </div>
      </div>
    </header>
  )
}
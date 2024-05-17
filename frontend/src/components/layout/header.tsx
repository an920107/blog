"use client";

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import React from 'react'

type Props = {}

const links = [
  {path: "/", label: "首頁"},
  {path: "/post", label: "文章"}
]

export default function Header({ }: Props) {
  const pathname = usePathname();

  return (
    <header className="sticky top-0 z-50 backdrop-blur-md border-b border-b-slate-200">
      <div className="container h-16 flex items-center justify-between">
        <Link href="/">
          <div className="text-2xl text-slate-800 font-black">魚之魷魂</div>
        </Link>
        <div className="flex gap-6">
          {
            links.map((e) => (
              <_HeaderLink key={e.path} href={e.path} isCurrent={(pathname + "/").startsWith(e.path + "/")}>{e.label}</_HeaderLink>
            ))
          }
        </div>
      </div>
    </header>
  )
}

function _HeaderLink(props: { href: string, isCurrent: boolean, children: string }) {
  return (
    <Link href={props.href}>
      {props.isCurrent ? (<mark>{props.children}</mark>) : props.children}
    </Link>
  )
}
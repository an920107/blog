import React from 'react'

type Props = {}

export default function Footer({}: Props) {
  return (
    <footer className="border-t border-t-slate-200">
      <div className="container h-24 flex justify-center items-center">
        <p className="">Copyright © {(new Date()).getFullYear()} SquidSpirit</p>
      </div>
    </footer>
  )
}
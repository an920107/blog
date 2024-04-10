import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faGithub, faYoutube } from '@fortawesome/free-brands-svg-icons';
import { faEnvelope } from '@fortawesome/free-solid-svg-icons';
import React from 'react'

type Props = {}

export default function Footer({ }: Props) {
  return (
    <footer className="border-t border-t-slate-200">
      <div className="container flex flex-wrap justify-center items-center py-12 gap-x-6">
        <div className="flex gap-x-4">
        <a href="https://www.youtube.com/@squidspirit16"><FontAwesomeIcon icon={faYoutube} className="h-4 w-4"></FontAwesomeIcon></a>
        <a href="mailto:an920107@gmail.com"><FontAwesomeIcon icon={faEnvelope} className="h-4 w-4"></FontAwesomeIcon></a>
        <a href="https://github.com/an920107/blog"><FontAwesomeIcon icon={faGithub} className="h-4 w-4"></FontAwesomeIcon></a>
        </div>
        <div className="h-6 border-s-2 border-s-slate-200"></div>
        <p>Copyright © {(new Date()).getFullYear()} SquidSpirit</p>
      </div>
    </footer>
  )
}

import Link from "next/link";

import { faGitAlt, faYoutube } from "@fortawesome/free-brands-svg-icons";
import { faEnvelope } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function Footer() {
  return (
    <div className="border-t border-gray-300">
      <div className="mx-auto flex max-w-screen-xl flex-col items-center justify-center gap-4 px-4 py-12 md:flex-row md:px-6">
        <div className="flex flex-row items-center justify-center gap-x-4">
          <Link href="https://www.youtube.com/@squidspirit16" target="_blank">
            <FontAwesomeIcon className="size-4" icon={faYoutube} title="YouTube Channel" />
          </Link>
          <Link href="mailto:squid@squidspirit.com">
            <FontAwesomeIcon className="size-4" icon={faEnvelope} title="Email" />
          </Link>
          <Link href="https://git.squidspirit.com/squid/blog" target="_blank">
            <FontAwesomeIcon className="size-4" icon={faGitAlt} title="Git Repository" />
          </Link>
        </div>
        <Devider className="max-md:hidden" />
        <span className="text-sm">Copyright © 2025 SquidSpirit</span>
      </div>
    </div>
  );
}

function Devider(props: { className?: string }) {
  return (
    <div className={props.className}>
      <div className="h-4 w-0.5 bg-gray-300" />
    </div>
  );
}

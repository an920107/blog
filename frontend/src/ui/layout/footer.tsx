import { faGithub, faYoutube } from "@fortawesome/free-brands-svg-icons";
import { faEnvelope } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";

type Props = {};

export default function Footer({}: Props) {
  return (
    <div className="border-t border-gray-300">
      <div className="mx-auto flex max-w-screen-xl flex-col items-center justify-center gap-4 px-4 py-12 md:flex-row md:px-6">
        <div className="flex flex-row items-center justify-center gap-x-4">
          <FontAwesomeIcon className="size-4" icon={faYoutube} />
          <FontAwesomeIcon className="size-4" icon={faEnvelope} />
          <FontAwesomeIcon className="size-4" icon={faGithub} />
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

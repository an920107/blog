import React from "react";

type Props = {};

export default function Navbar({}: Props) {
  return (
    <div className="border-b border-gray-300">
      <div className="mx-auto flex h-16 max-w-screen-xl flex-row items-center justify-between px-4 md:px-6">
        <a className="text-2xl font-black text-gray-800" href="/">
          魚之魷魂
        </a>
        <div className="flex flex-row items-center gap-x-6">
          <Action label="首頁" link="/" isSelected={true} />
        </div>
      </div>
    </div>
  );
}

function Action(props: { label: string; link: string; isSelected: boolean }) {
  return (
    <div
      className={`rounded px-1.5 ${props.isSelected ? "bg-blue-600" : "bg-transparent"}`}
    >
      <a
        className={`font-extrabold ${props.isSelected ? "text-white" : "text-gray-800"}`}
        href={props.link}
      >
        {props.label}
      </a>
    </div>
  );
}

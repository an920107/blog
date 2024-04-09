"use client"

import React, { useEffect, useState } from 'react'

type Props = {}

const selfTags = [
  "軟體工程師", "暴肝", "碼農", "中央大學", "資訊工程學系", "遊戲", "Linux",
  "實況", "教學", "科技", "程式", "科普", "魷魚", "Squid", "APP", "TypeScript",
  "Flutter", "前端", "後端", "全端", "Python", "Java", "C++", "C",
]

export default function Home({ }: Props) {
  const [tags, setTags] = useState<String[]>([...selfTags]);
  const [tagsVisible, setTagsVisible] = useState<boolean>(false);

  useEffect(() => {
    const interval = setInterval(() => {
      setTags(
        selfTags.map(value => ({ value, sort: Math.random() }))
          .sort((a, b) => a.sort - b.sort)
          .map(value => value.value)
      )
    }, 5000);
  }, [])

  useEffect(() => {
    setTagsVisible(prev => true);
    setTimeout(() => {
      setTagsVisible(prev => false);
    }, 4400);
  }, [tags])

  return (
    <>
      <div className="container fill-screen flex justify-start items-center">
        <div className="w-screen flex flex-col gap-4">
          <h1>Hello 大家好！</h1>
          <h2>我是 Squid 魷魚</h2>
          <br />
          <div className={`relative h-6 max-w-[50rem] transition-opacity duration-500 opacity-${tagsVisible ? "100" : "0"}`}>
            <p className="absolute start-0 end-0 top-0 bottom-0 text-slate-500 font-serif overflow-hidden">{
              tags.map((value, index) => `# ${value} `)
            }</p>
            <div className="absolute end-0 w-36 top-0 bottom-0 bg-gradient-to-r from-transparent to-white"></div>
          </div>
        </div>
      </div>
    </>
  )
}

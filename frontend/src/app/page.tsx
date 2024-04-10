"use client"

import React, { useEffect, useState } from 'react'

type Props = {}

export default function Home({ }: Props) {
  const [tags, setTags] = useState<string[]>([...selfTags]);
  const [tagsVisible, setTagsVisible] = useState<boolean>(false);

  useEffect(() => {
    setInterval(() => {
      setTags(
        selfTags.map(value => ({ value, sort: Math.random() }))
          .sort((a, b) => a.sort - b.sort)
          .map(value => value.value)
      )
    }, 4500);
  }, [])

  useEffect(() => {
    setTagsVisible(prev => true);
    setTimeout(() => {
      setTagsVisible(prev => false);
    }, 3900);
  }, [tags])

  return (
    <>
      <div className="container fill-screen flex justify-start items-center mb-8">
        <div className="w-screen flex flex-col gap-10">
          <h2>Hello 大家好！</h2>
          <h1>我是 <mark>Squid</mark> 魷魚</h1>
          <div className={`relative my-2 h-6 max-w-[50rem] transition-opacity duration-500 ${tagsVisible ? "opacity-100" : "opacity-0"}`}>
            <p className="absolute start-0 end-0 top-0 bottom-0 text-slate-500 font-serif overflow-hidden">{
              tags.map((value, index) => `# ${value} `)
            }</p>
            <div className="absolute end-0 w-36 top-0 bottom-0 bg-gradient-to-r from-transparent to-white"></div>
          </div>
        </div>
      </div>
      <div className="container fill-screen flex justify-start items-center mb-8">
        <div className="flex flex-col">
          {
            history.map((value, index) => (
              <div className="flex flex-row justify-start items-start">
                <div>
                  <div className="h-6 w-6 rounded-full bg-blue-600 me-6 mt-2"></div>
                  <div className="h-4"></div>
                  <div className={"ms-2 mt-2 h-20 w-2 rounded bg-slate-800"}></div>
                  <div className="h-4"></div>
                </div>
                <div className="flex flex-col gap-2">
                  <div className="flex gap-4 items-center">
                    <h3>{value.title}</h3>
                    <p className="font-serif">
                      {value.date.getFullYear()} 年 {(value.date.getMonth() + 1).toString().padStart(2, "0")} 月
                    </p>
                  </div>
                  <p>{value.description}</p>
                </div>
              </div>
            ))
          }
        </div>
      </div>
      {/* <div className="container fill-screen flex justify-start items-center mb-8">
        123
      </div> */}
    </>
  )
}

const selfTags = [
  "軟體工程師", "暴肝", "碼農", "中央大學", "資訊工程學系", "遊戲", "Linux",
  "實況", "教學", "科技", "程式", "科普", "魷魚", "Squid", "APP", "TypeScript",
  "Flutter", "前端", "後端", "全端", "Python", "Java", "C++", "C",
]

type HistoryEvent = {
  title: string;
  date: Date;
  description: string;
}

const history: HistoryEvent[] = [
  {
    title: "建立頻道",
    date: new Date(2016, 2),
    description: "因為看到網路上開始有實況主這樣的角色出現，一時興起便以 Squid 魷魚為名成立 YouTube 頻道。",
  },
  {
    title: "架設個人網站",
    date: new Date(2018, 4),
    description: "頻道已累計數百人訂閱，想架設自己的個人網站，因此開始研究如 Linux、基礎設施等與伺服器架設相關的技術。",
  },
  {
    title: "開發熱門軟體",
    date: new Date(2020, 3),
    description: "Minecraft 伺服器安裝器開發完成！沒想到這項簡單的小工具為這部影片帶來數十萬的觀看，頻道訂閱人數也來到數千人，而我對於軟體開發也更加感興趣。",
  },
  {
    title: "就讀大學",
    date: new Date(2021, 8),
    description: "進入國立中央大學，並且如願就讀資訊工程學系，對於過去自己摸索到的，終於能系統性的進行學習了，但也因此沒了時間繼續更新頻道......",
  },
  {
    title: "擔任助教",
    date: new Date(2022, 8),
    description: "開始擔任系上的服務學習課程助教，而我從其中收穫最多的其實是與貴人相遇，學習軟工技術、參加黑客松獲獎、實習經驗分享，都是受到學長所幫助的。",
  },
  {
    title: "新創公司",
    date: new Date(2023, 11),
    description: "朋友創業成立了公司，邀請我擔任資訊長一職，協助技術整合與前端軟體開發。",
  },
  {
    title: "重寫網站",
    date: new Date(2024, 3),
    description: "過去使用 Wordpress 放置個人內容與教學文章，但現今已有自行撰寫網站的能力了，因此決定也當作練習，以 Next.js 開發新的個人網站。",
  },
]
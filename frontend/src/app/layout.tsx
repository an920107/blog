import "./global.css"

import { Noto_Sans_TC, Noto_Serif_TC } from "next/font/google"

import Footer from "@/components/layout/footer"
import Header from "@/components/layout/header"

const notoSansTc = Noto_Sans_TC({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-sans",
});

const notoSerifTc = Noto_Serif_TC({
  subsets: ["latin"],
  weight: ["200", "300", "400", "500", "600", "700", "900"],
  display: "swap",
  variable: "--font-serif",
});

export const metadata = {
  title: '魚之魷魂',
  description: '程式、科技、教學、實況',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="zh-TW" className={`${notoSansTc.variable} ${notoSerifTc.variable}`}>
      <body>
        <Header />
        <div className="fill-screen flex flex-col">
          {children}
        </div>
        <Footer />
      </body>
    </html>
  )
}

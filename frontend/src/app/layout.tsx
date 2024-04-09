import "./global.css"

import Footer from "@/components/layout/footer"
import Header from "@/components/layout/header"

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
    <html lang="zh-TW">
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous" />
        <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+TC:wght@100..900&family=Noto+Serif+TC&display=swap" rel="stylesheet" />
      </head>
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

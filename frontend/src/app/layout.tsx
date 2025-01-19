import type { Metadata } from "next";
import { Noto_Sans_Mono, Noto_Sans_TC } from "next/font/google";
import localFont from "next/font/local";

import Footer from "@/lib/common/presenter/ui/Footer";
import Navbar from "@/lib/common/presenter/ui/Navbar";

import "./globals.css";

const notoSansTc = Noto_Sans_TC({
  variable: "--font-noto-sans-tc",
  subsets: ["latin"],
});

const notoSansMono = Noto_Sans_Mono({
  variable: "--font-noto-sans-mono",
  subsets: ["latin"],
});

const hackNerdMono = localFont({
  src: "./HackNerdMono.woff2",
  variable: "--font-hack-nerd-mono",
});

export const metadata: Metadata = {
  title: "魚之魷魂 SquidSpirit",
  description: "程式、科技、教學、分享",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="zh-Hant">
      <body className={`${notoSansTc.variable} ${notoSansMono.variable} ${hackNerdMono.variable} antialiased`}>
        <div className="min-h-screen">
          <Navbar />
          {children}
        </div>
        <Footer />
      </body>
    </html>
  );
}

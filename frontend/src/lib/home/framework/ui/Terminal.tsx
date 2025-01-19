"use client";

import { useEffect, useRef, useState } from "react";

export default function Terminal() {
  const [isReady, setIsReady] = useState(false);
  const [currentIndex, setCurrentLineIndex] = useState(0);

  const element = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!element.current) {
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setIsReady(true);
            observer.disconnect();
          }
        });
      },
      { threshold: 1 },
    );
    observer.observe(element.current);

    return () => observer.disconnect();
  }, [currentIndex]);

  function onLineCompleted() {
    if (currentIndex < lines.length - 1) {
      setCurrentLineIndex((prev) => prev + 1);
    }
  }

  return (
    <div
      ref={element}
      className={`bg-true-gray-700 border-true-gray-800 flex w-full flex-col gap-y-1.5 rounded-2xl border-4 p-4 pb-28 font-mono font-medium text-gray-50 transition-opacity duration-300 md:gap-y-2.5 md:rounded-3xl md:border-8 md:p-8 md:pb-32 md:text-xl ${isReady ? "opacity-100" : "opacity-0"}`}
    >
      {lines.slice(0, currentIndex).map((line, index) => (
        <NormalLine key={index} text={line} />
      ))}
      {isReady ? <LastLine key={currentIndex} text={lines[currentIndex]} onCompleted={onLineCompleted} /> : null}
    </div>
  );
}

function NormalLine(props: { text: string }) {
  return (
    <div className="flex w-full flex-row gap-x-1.5 md:gap-x-2">
      <span className="text-green-400">❯</span>
      <span>{props.text}</span>
    </div>
  );
}

function LastLine(props: { text: string; onCompleted?: () => void }) {
  const [timeText, setTimeText] = useState("");
  const [textToShow, setTextToShow] = useState("");

  useEffect(() => {
    let interval: NodeJS.Timeout | undefined = undefined;

    setTimeout(() => {
      interval = setInterval(() => {
        setTextToShow((prev) => {
          if (prev.length < props.text.length) {
            return prev + props.text[prev.length];
          } else {
            clearInterval(interval);
            return prev;
          }
        });
      }, 50);
    }, 300);

    return () => clearInterval(interval);
  }, [props.text]);

  useEffect(() => {
    if (textToShow.length === props.text.length) {
      setTimeout(() => {
        props.onCompleted?.();
      }, 300);
    }
  }, [props, textToShow]);

  useEffect(() => {
    setTimeText(dateToString(new Date()));
    const interval = setInterval(() => {
      setTimeText(dateToString(new Date()));
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex w-full flex-col pt-1.5 leading-5 md:pt-2.5 md:leading-7">
      <div className="flex w-full flex-row flex-nowrap items-center gap-x-1.5 text-nowrap md:gap-x-2">
        <span>
          ╭─  squid{" "}
          <span className="text-blue-500">
            ~<span className="max-md:hidden">/Documents/blog</span>
          </span>
        </span>
        <div className="h-0.5 w-full bg-gray-50" />
        <span> {timeText}</span>
      </div>
      <div className="flex w-full flex-row gap-x-1.5 md:gap-x-2">
        <span>
          ╰─<span className="text-green-400">❯</span>
        </span>
        <span>{textToShow}</span>
        <Cursor />
      </div>
    </div>
  );
}

function Cursor() {
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    const interval = setInterval(() => {
      setIsVisible((prev) => !prev);
      setTimeout(() => {
        setIsVisible((prev) => !prev);
      }, 500);
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return <span className={isVisible ? "" : "hidden"}>█</span>;
}

function dateToString(date: Date) {
  return date.toLocaleString("en-US", { hour: "2-digit", minute: "2-digit", second: "2-digit", hour12: false });
}

const lines = [
  "大家好，我是 Squid 魷魚",
  "身為一位軟體工程師",
  "平常最喜歡埋首於程式碼的世界",
  "鑽研各種新奇有趣的技術",
  "在這裡",
  "我會分享我的技術筆記、開發心得",
  "還有各式各樣實用工具的評測與介紹",
  "一起探索數位世界的無限可能吧！",
];

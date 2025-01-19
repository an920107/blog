"use client";

import { useEffect, useRef, useState } from "react";

export default function AnimatedMark(props: { text: string; direction: "left" | "right" }) {
  const [isVisible, setIsVisible] = useState(false);

  const element = useRef<HTMLSpanElement | null>(null);

  const origin = props.direction === "left" ? "origin-left" : "origin-right";

  useEffect(() => {
    if (!element.current) {
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setIsVisible(true);
            observer.disconnect();
          }
        });
      },
      { threshold: 1 },
    );
    observer.observe(element.current);

    return () => observer.disconnect();
  }, []);

  return (
    <span
      ref={element}
      className={`rounded-md bg-blue-600 px-1 py-0.5 text-white transition-transform delay-500 duration-1000 md:rounded-lg md:px-2.5 md:py-2 ${origin} ${isVisible ? "scale-x-100" : "scale-x-0"} `}
    >
      <span className="scale-x-100">{props.text}</span>
    </span>
  );
}

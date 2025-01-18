"use client";

import { useEffect, useRef, useState } from "react";
import { Provider } from "react-redux";

import { useTagDispatch, useTagSelector } from "@/lib/home/presenter/tagHook";
import { tagShuffledAction } from "@/lib/home/presenter/tagReducer";
import tagStore from "@/lib/home/presenter/tagStore";

export default function SelfTags() {
  return (
    <Provider store={tagStore}>
      <SelfTagsProvided />
    </Provider>
  );
}

function SelfTagsProvided() {
  const tags = useTagSelector((state) => state.tags);
  const dispatch = useTagDispatch();

  // Initialize with placeholder to prevent flickering
  const [tagsToShow, setTagsToShow] = useState<string[]>([""]);
  const [isTagsVisible, setIsTagsVisible] = useState(false);

  const timer = useRef<NodeJS.Timeout | undefined>(undefined);

  // On mount
  useEffect(() => {
    timer.current = setInterval(() => {
      dispatch(tagShuffledAction());
    }, 4000);

    return () => {
      clearInterval(timer.current);
      timer.current = undefined;
    };
  }, [dispatch]);

  // On tags changed
  useEffect(() => {
    setIsTagsVisible(false);

    setTimeout(() => {
      setTagsToShow(tags);
      setIsTagsVisible(true);
    }, 500);
  }, [tags]);

  return (
    <div
      className={`relative w-full max-w-screen-md transition-opacity duration-500 ${isTagsVisible ? "opacity-100" : "opacity-0"}`}
    >
      <div className="absolute inset-0 bg-gradient-to-r from-transparent via-transparent via-60% to-white" />
      <div className="flex flex-row items-center gap-x-2 overflow-hidden">
        {tagsToShow.map((tag) => (
          <Hashtag key={tag} tag={tag} />
        ))}
      </div>
    </div>
  );
}

function Hashtag(props: { tag: string }) {
  return <span className="text-nowrap text-gray-400"># {props.tag}</span>;
}

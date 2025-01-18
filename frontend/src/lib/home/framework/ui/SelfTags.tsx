"use client";

import { useEffect, useState } from "react";
import { Provider } from "react-redux";

import { useTagDispatch, useTagSelector } from "@/lib/home/presenter/tagHooks";
import { tagStartedAction, tagStoppedAction } from "@/lib/home/presenter/tagSlice";
import tagStore from "@/lib/home/presenter/tagStore";

export default function SelfTags() {
  return (
    <Provider store={tagStore}>
      <SelfTagsProvided />
    </Provider>
  );
}

function SelfTagsProvided() {
  const tags = useTagSelector((state) => state.tag.tags);
  const dispatch = useTagDispatch();

  const [isTagsVisible, setIsTagsVisible] = useState(false);

  useEffect(() => {
    dispatch(tagStartedAction({ interval: 4000 }));
    return () => {
      dispatch(tagStoppedAction());
      setIsTagsVisible(false);
    };
  }, [dispatch]);

  useEffect(() => {
    if (tags.length === 0) return;
    setIsTagsVisible(true);
    setTimeout(() => {
      setIsTagsVisible(false);
    }, 3500);
  }, [tags]);

  return (
    <div
      className={`relative w-full max-w-screen-md transition-opacity duration-500 ${isTagsVisible ? "opacity-100" : "opacity-0"}`}
    >
      <div className="absolute inset-0 bg-gradient-to-r from-transparent via-transparent via-60% to-white" />
      <div className="flex flex-row items-center gap-x-2 overflow-hidden">
        {tags.map((tag) => (
          <Hashtag key={tag} tag={tag} />
        ))}
      </div>
    </div>
  );
}

function Hashtag(props: { tag: string }) {
  return <span className="text-nowrap text-gray-400"># {props.tag}</span>;
}

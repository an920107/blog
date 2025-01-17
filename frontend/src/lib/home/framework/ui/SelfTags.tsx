"use client";

import { Provider } from "react-redux";
import tagStore from "../../presenter/tagStore";
import { useTagDispatch, useTagSelector } from "../../presenter/tagHooks";
import { useEffect, useState } from "react";
import { tagStartedAction, tagStoppedAction } from "../../presenter/tagSlice";

export default function SelfTags() {
  return (
    <Provider store={tagStore}>
      <_SelfTags />
    </Provider>
  );
}

function _SelfTags() {
  const tags = useTagSelector((state) => state.tag.tags);
  const dispatch = useTagDispatch();

  const [isTagsVisible, setIsTagsVisible] = useState(false);

  useEffect(() => {
    dispatch(tagStartedAction({ interval: 4000 }));
    return () => {
      dispatch(tagStoppedAction());
      setIsTagsVisible(false);
    };
  }, []);

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

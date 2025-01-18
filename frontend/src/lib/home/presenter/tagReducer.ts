import { createAction, createReducer } from "@reduxjs/toolkit";

import shuffleArray from "@/lib/util/shuffleArray";

export const tagShuffledAction = createAction("tag/shuffled");

const tagReducer = createReducer<TagState>(
  () => ({
    tags: shuffleArray(tagsCollection),
  }),
  (builder) => {
    builder.addCase(tagShuffledAction, (state) => {
      state.tags = shuffleArray(tagsCollection);
    });
  },
);

export default tagReducer;
export type TagAction = ReturnType<typeof tagShuffledAction>;

export interface TagState {
  tags: string[];
  timer?: NodeJS.Timeout;
}

const tagsCollection = [
  "APP",
  "C++",
  "Design Pattern",
  "Docker",
  "Flutter",
  "Go",
  "Java",
  "LINER",
  "Linux",
  "Python",
  "Squid",
  "TypeScript",
  "中央大學",
  "全端",
  "分享",
  "前端",
  "後端",
  "教學",
  "暴肝",
  "知識",
  "碼農",
  "科技",
  "科普",
  "程式設計",
  "資工系",
  "軟體工程",
  "遊戲",
  "魷魚",
];

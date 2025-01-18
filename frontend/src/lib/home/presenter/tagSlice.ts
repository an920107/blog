import { PayloadAction, createSlice } from "@reduxjs/toolkit";

import tagStore from "@/lib/home/presenter/tagStore";
import shuffleArray from "@/lib/util/shuffleArray";

export interface TagState {
  tags: string[];
  timer?: NodeJS.Timeout;
}

export interface TagStartedActionPayload {
  interval: number;
}

export const tagSlice = createSlice({
  name: "tag",
  initialState: {
    tags: [],
    timer: undefined,
  } as TagState,
  reducers: {
    started: (state, action: PayloadAction<TagStartedActionPayload>) => {
      state.tags = shuffleArray(tagsCollection);
      state.timer = setInterval(() => {
        tagStore.dispatch(tagSlice.actions.shuffled());
      }, action.payload.interval);
    },
    shuffled: (state) => {
      state.tags = shuffleArray(tagsCollection);
    },
    stopped: (state) => {
      clearInterval(state.timer);
      state.timer = undefined;
    },
  },
});

export const tagStartedAction = tagSlice.actions.started;
export const tagStoppedAction = tagSlice.actions.stopped;

const tagReducer = tagSlice.reducer;
export default tagReducer;

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

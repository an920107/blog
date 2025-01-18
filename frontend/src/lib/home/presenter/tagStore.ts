import { configureStore } from "@reduxjs/toolkit";

import tagReducer from "@/lib/home/presenter/tagSlice";

const tagStore = configureStore({
  reducer: {
    tag: tagReducer,
  },
});

export default tagStore;

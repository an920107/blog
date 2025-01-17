import { configureStore } from "@reduxjs/toolkit";
import tagReducer from "./tagSlice";

const tagStore = configureStore({
  reducer: {
    tag: tagReducer,
  },
});

export default tagStore;

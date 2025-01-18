import { configureStore } from "@reduxjs/toolkit";

import tagReducer, { TagAction, TagState } from "@/lib/home/presenter/tagReducer";

export default configureStore<TagState, TagAction>({
  reducer: tagReducer,
});

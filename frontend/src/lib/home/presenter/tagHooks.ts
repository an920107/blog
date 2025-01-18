import { useDispatch, useSelector } from "react-redux";

import tagStore from "@/lib/home/presenter/tagStore";

export const useTagDispatch = useDispatch.withTypes<typeof tagStore.dispatch>();
export const useTagSelector = useSelector.withTypes<ReturnType<typeof tagStore.getState>>();

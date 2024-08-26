import { create } from "zustand";
import {SearchResult} from "@/types.ts";

export enum ActionType {
  Add,
  Search
}

type State = {
  resp?: SearchResult;
  action: ActionType;
  isLoading: boolean
};

type Action = {
  setResp: (resp?: SearchResult) => void;
  setAction: (action: ActionType) => void;
  setIsLoading: (isLoading: boolean) => void;
};

type Store = State & Action;

const useStore = create<Store>()((set) => ({
  action: ActionType.Search,
  setResp: (resp?: SearchResult) => set(() => ({ resp })),
  setAction: (action: ActionType) => set(() => ({ action })),
    isLoading: false,
    setIsLoading: (isLoading: boolean) => set(() => ({ isLoading }))
}));

export default useStore;

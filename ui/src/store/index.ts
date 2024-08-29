import { create } from "zustand";
import {SearchResults} from "@/types.ts";

export enum ActionType {
  Add,
  Search
}

type State = {
  resp: SearchResults;
  action: ActionType;
  isLoading: boolean
};

type Action = {
  setResp: (resp: SearchResults) => void;
  setAction: (action: ActionType) => void;
  setIsLoading: (isLoading: boolean) => void;
};

type Store = State & Action;

const useStore = create<Store>()((set) => ({
  resp: [],
  action: ActionType.Search,
  setResp: (resp?: SearchResults) => set(() => ({ resp })),
  setAction: (action: ActionType) => set(() => ({ action })),
    isLoading: false,
    setIsLoading: (isLoading: boolean) => set(() => ({ isLoading }))
}));

export default useStore;

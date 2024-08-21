import { SearchResult } from "@/hook/useSearch";
import { create } from "zustand";

type State = {
  resp?: SearchResult;
};

type Action = {
  setResp: (resp?: SearchResult) => void;
};

type Store = State & Action;

const useStore = create<Store>()((set) => ({
  setResp: (resp?: SearchResult) => set(() => ({ resp })),
}));

export default useStore;

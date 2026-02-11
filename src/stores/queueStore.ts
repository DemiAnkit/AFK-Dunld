import { create } from "zustand";
import { downloadApi } from "../services/tauriApi";
import type { QueueInfo } from "../types/download";
import toast from "react-hot-toast";

interface QueueState {
  queueInfo: QueueInfo | null;
  isLoading: boolean;
  error: string | null;
  fetchQueueInfo: () => Promise<void>;
  setMaxConcurrent: (max: number) => Promise<void>;
}

export const useQueueStore = create<QueueState>((set) => ({
  queueInfo: null,
  isLoading: false,
  error: null,

  fetchQueueInfo: async () => {
    set({ isLoading: true, error: null });
    try {
      const queueInfo = await downloadApi.getQueueInfo();
      set({ queueInfo, isLoading: false });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to fetch queue info";
      set({ error: errorMessage, isLoading: false });
      console.error("Failed to fetch queue info:", error);
    }
  },

  setMaxConcurrent: async (max: number) => {
    try {
      await downloadApi.setMaxConcurrent(max);
      set((state) => ({
        queueInfo: state.queueInfo 
          ? { ...state.queueInfo, maxConcurrent: max }
          : null,
      }));
      toast.success(`Max concurrent downloads set to ${max}`);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to set max concurrent";
      set({ error: errorMessage });
      toast.error(errorMessage);
      throw error;
    }
  },
}));

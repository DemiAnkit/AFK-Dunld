import { create } from "zustand";

interface QueueState {
  queue: string[];
  isProcessing: boolean;
  addToQueue: (url: string) => void;
  removeFromQueue: (url: string) => void;
  processQueue: () => Promise<void>;
}

export const useQueueStore = create<QueueState>((set, get) => ({
  queue: [],
  isProcessing: false,

  addToQueue: (url: string) => {
    set((state) => ({
      queue: [...state.queue, url],
    }));
  },

  removeFromQueue: (url: string) => {
    set((state) => ({
      queue: state.queue.filter((item) => item !== url),
    }));
  },

  processQueue: async () => {
    const { queue, isProcessing } = get();
    if (isProcessing || queue.length === 0) return;

    set({ isProcessing: true });
    // TODO: Implement queue processing
    console.log("Processing queue...", queue);
    set({ isProcessing: false });
  },
}));

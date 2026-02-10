import { create } from "zustand";

export type DownloadStatus = 
  | "Pending" 
  | "Downloading" 
  | "Paused" 
  | "Completed" 
  | "Failed" 
  | "Cancelled" 
  | "Queued";

export interface DownloadProgress {
  id: string;
  downloaded_bytes: number;
  total_bytes: number;
  speed: number;
  eta: number;
}

export interface Download {
  id: string;
  url: string;
  file_name: string;
  file_path: string;
  status: DownloadStatus;
  total_bytes: number;
  downloaded_bytes: number;
  speed: number;
  eta: number;
  error_message?: string;
  created_at: string;
  completed_at?: string;
}

interface DownloadState {
  downloads: Download[];
  fetchDownloads: () => Promise<void>;
  addDownload: (url: string) => Promise<void>;
  updateProgress: (progress: DownloadProgress) => void;
  pauseDownload: (id: string) => Promise<void>;
  resumeDownload: (id: string) => Promise<void>;
  cancelDownload: (id: string) => Promise<void>;
  removeDownload: (id: string) => Promise<void>;
  retryDownload: (id: string) => Promise<void>;
}

export const useDownloadStore = create<DownloadState>((set, get) => ({
  downloads: [],

  fetchDownloads: async () => {
    // Mock data for now - replace with actual Tauri API call
    set({ downloads: [] });
  },

  addDownload: async (url: string) => {
    console.log("Adding download:", url);
    // TODO: Implement with Tauri API
  },

  updateProgress: (progress: DownloadProgress) => {
    set((state) => ({
      downloads: state.downloads.map((d) =>
        d.id === progress.id
          ? {
              ...d,
              downloaded_bytes: progress.downloaded_bytes,
              total_bytes: progress.total_bytes,
              speed: progress.speed,
              eta: progress.eta,
            }
          : d
      ),
    }));
  },

  pauseDownload: async (id: string) => {
    console.log("Pausing download:", id);
  },

  resumeDownload: async (id: string) => {
    console.log("Resuming download:", id);
  },

  cancelDownload: async (id: string) => {
    console.log("Cancelling download:", id);
  },

  removeDownload: async (id: string) => {
    set((state) => ({
      downloads: state.downloads.filter((d) => d.id !== id),
    }));
  },

  retryDownload: async (id: string) => {
    console.log("Retrying download:", id);
  },
}));

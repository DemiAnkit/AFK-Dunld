import { create } from "zustand";
import { downloadService } from "../services/downloadService";
import type { Download, DownloadProgress as DownloadProgressType } from "../types/download";
import toast from "react-hot-toast";

interface DownloadState {
  downloads: Download[];
  isLoading: boolean;
  error: string | null;
  fetchDownloads: () => Promise<void>;
  addDownload: (url: string, options?: { savePath?: string; fileName?: string; category?: string }) => Promise<void>;
  updateProgress: (progress: DownloadProgressType) => void;
  updateDownload: (download: Download) => void;
  pauseDownload: (id: string) => Promise<void>;
  resumeDownload: (id: string) => Promise<void>;
  cancelDownload: (id: string) => Promise<void>;
  removeDownload: (id: string, deleteFile?: boolean) => Promise<void>;
  retryDownload: (id: string) => Promise<void>;
  pauseAll: () => Promise<void>;
  resumeAll: () => Promise<void>;
  cancelAll: () => Promise<void>;
  addBatchDownloads: (urls: string[], savePath?: string) => Promise<void>;
  pauseSelected: (ids: string[]) => Promise<void>;
  resumeSelected: (ids: string[]) => Promise<void>;
  cancelSelected: (ids: string[]) => Promise<void>;
  removeSelected: (ids: string[], deleteFiles?: boolean) => Promise<void>;
}

export const useDownloadStore = create<DownloadState>((set, get) => ({
  downloads: [],
  isLoading: false,
  error: null,

  fetchDownloads: async () => {
    set({ isLoading: true, error: null });
    try {
      const downloads = await downloadService.getAllDownloads();
      set({ downloads, isLoading: false });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to fetch downloads";
      set({ error: errorMessage, isLoading: false });
      toast.error(errorMessage);
    }
  },

  addDownload: async (url: string, options = {}) => {
    set({ error: null });
    try {
      const download = await downloadService.addDownload(url, {
        save_path: options.savePath,
        file_name: options.fileName,
        category: options.category,
      });
      
      set((state) => ({
        downloads: [...state.downloads, download],
      }));
      
      toast.success(`Download started: ${download.fileName}`);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to add download";
      set({ error: errorMessage });
      toast.error(errorMessage);
      throw error;
    }
  },

  updateProgress: (progress: DownloadProgressType) => {
    set((state) => ({
      downloads: state.downloads.map((d) =>
        d.id === progress.id
          ? {
              ...d,
              downloadedSize: progress.downloadedSize,
              totalSize: progress.totalSize,
              speed: progress.speed,
              eta: progress.eta,
              status: progress.status,
            }
          : d
      ),
    }));
  },

  updateDownload: (download: Download) => {
    set((state) => ({
      downloads: state.downloads.map((d) =>
        d.id === download.id ? download : d
      ),
    }));
  },

  pauseDownload: async (id: string) => {
    try {
      await downloadService.pauseDownload(id);
      set((state) => ({
        downloads: state.downloads.map((d) =>
          d.id === id ? { ...d, status: "paused" as const } : d
        ),
      }));
      toast.success("Download paused");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to pause download";
      toast.error(errorMessage);
      throw error;
    }
  },

  resumeDownload: async (id: string) => {
    try {
      await downloadService.resumeDownload(id);
      set((state) => ({
        downloads: state.downloads.map((d) =>
          d.id === id ? { ...d, status: "downloading" as const } : d
        ),
      }));
      toast.success("Download resumed");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to resume download";
      toast.error(errorMessage);
      throw error;
    }
  },

  cancelDownload: async (id: string) => {
    try {
      await downloadService.cancelDownload(id);
      set((state) => ({
        downloads: state.downloads.map((d) =>
          d.id === id ? { ...d, status: "cancelled" as const } : d
        ),
      }));
      toast.success("Download cancelled");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to cancel download";
      toast.error(errorMessage);
      throw error;
    }
  },

  removeDownload: async (id: string, deleteFile = false) => {
    try {
      await downloadService.removeDownload(id, deleteFile);
      set((state) => ({
        downloads: state.downloads.filter((d) => d.id !== id),
      }));
      toast.success(deleteFile ? "Download removed and file deleted" : "Download removed");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to remove download";
      toast.error(errorMessage);
      throw error;
    }
  },

  retryDownload: async (id: string) => {
    try {
      await downloadService.retryDownload(id);
      toast.success("Download retry initiated");
      // Fetch downloads to get updated state
      await get().fetchDownloads();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to retry download";
      toast.error(errorMessage);
      throw error;
    }
  },

  pauseAll: async () => {
    try {
      await downloadService.pauseAll();
      toast.success("All downloads paused");
      await get().fetchDownloads();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to pause all downloads";
      toast.error(errorMessage);
      throw error;
    }
  },

  resumeAll: async () => {
    try {
      await downloadService.resumeAll();
      toast.success("All downloads resumed");
      await get().fetchDownloads();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to resume all downloads";
      toast.error(errorMessage);
      throw error;
    }
  },

  cancelAll: async () => {
    try {
      await downloadService.cancelAll();
      toast.success("All downloads cancelled");
      await get().fetchDownloads();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to cancel all downloads";
      toast.error(errorMessage);
      throw error;
    }
  },

  addBatchDownloads: async (urls: string[], savePath?: string) => {
    try {
      const downloads = await downloadService.addBatchDownloads(urls, savePath);
      set((state) => ({
        downloads: [...state.downloads, ...downloads],
      }));
      toast.success(`${downloads.length} downloads added`);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to add batch downloads";
      toast.error(errorMessage);
      throw error;
    }
  },

  pauseSelected: async (ids: string[]) => {
    const errors: string[] = [];
    let successCount = 0;

    for (const id of ids) {
      try {
        await downloadService.pauseDownload(id);
        successCount++;
      } catch (error) {
        errors.push(id);
        console.error(`Failed to pause download ${id}:`, error);
      }
    }

    // Update state for successful pauses
    set((state) => ({
      downloads: state.downloads.map((d) =>
        ids.includes(d.id) && !errors.includes(d.id) ? { ...d, status: "paused" as const } : d
      ),
    }));

    if (successCount > 0) {
      toast.success(`Paused ${successCount} download${successCount > 1 ? 's' : ''}`);
    }
    if (errors.length > 0) {
      toast.error(`Failed to pause ${errors.length} download${errors.length > 1 ? 's' : ''}`);
    }
  },

  resumeSelected: async (ids: string[]) => {
    const errors: string[] = [];
    let successCount = 0;

    for (const id of ids) {
      try {
        await downloadService.resumeDownload(id);
        successCount++;
      } catch (error) {
        errors.push(id);
        console.error(`Failed to resume download ${id}:`, error);
      }
    }

    // Update state for successful resumes
    set((state) => ({
      downloads: state.downloads.map((d) =>
        ids.includes(d.id) && !errors.includes(d.id) ? { ...d, status: "downloading" as const } : d
      ),
    }));

    if (successCount > 0) {
      toast.success(`Resumed ${successCount} download${successCount > 1 ? 's' : ''}`);
    }
    if (errors.length > 0) {
      toast.error(`Failed to resume ${errors.length} download${errors.length > 1 ? 's' : ''}`);
    }
  },

  cancelSelected: async (ids: string[]) => {
    const errors: string[] = [];
    let successCount = 0;

    for (const id of ids) {
      try {
        await downloadService.cancelDownload(id);
        successCount++;
      } catch (error) {
        errors.push(id);
        console.error(`Failed to cancel download ${id}:`, error);
      }
    }

    // Update state for successful cancellations
    set((state) => ({
      downloads: state.downloads.map((d) =>
        ids.includes(d.id) && !errors.includes(d.id) ? { ...d, status: "cancelled" as const } : d
      ),
    }));

    if (successCount > 0) {
      toast.success(`Cancelled ${successCount} download${successCount > 1 ? 's' : ''}`);
    }
    if (errors.length > 0) {
      toast.error(`Failed to cancel ${errors.length} download${errors.length > 1 ? 's' : ''}`);
    }
  },

  removeSelected: async (ids: string[], deleteFiles = false) => {
    const errors: string[] = [];
    let successCount = 0;

    for (const id of ids) {
      try {
        await downloadService.removeDownload(id, deleteFiles);
        successCount++;
      } catch (error) {
        errors.push(id);
        console.error(`Failed to remove download ${id}:`, error);
      }
    }

    // Remove successful deletions from state
    set((state) => ({
      downloads: state.downloads.filter((d) => !ids.includes(d.id) || errors.includes(d.id)),
    }));

    if (successCount > 0) {
      toast.success(
        deleteFiles 
          ? `Removed ${successCount} download${successCount > 1 ? 's' : ''} and deleted file${successCount > 1 ? 's' : ''}`
          : `Removed ${successCount} download${successCount > 1 ? 's' : ''}`
      );
    }
    if (errors.length > 0) {
      toast.error(`Failed to remove ${errors.length} download${errors.length > 1 ? 's' : ''}`);
    }
  },
}));

// Re-export types for convenience
export type { Download, DownloadProgressType as DownloadProgress };

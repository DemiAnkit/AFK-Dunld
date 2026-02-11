// src/services/tauriApi.ts
import { invoke } from '@tauri-apps/api/core';
import type { Download, DownloadProgress, FileInfo, DownloadStats, QueueInfo } from '../types/download';
import type { VideoInfo, QualityOption, YouTubeDownloadOptions } from '../types/youtube';

export interface AddDownloadRequest {
  url: string;
  save_path?: string;
  segments?: number;
  max_retries?: number;
  expected_checksum?: string;
  checksum_type?: string;
  file_name?: string;
  category?: string;
  priority?: number;
}

export interface BatchDownloadItem {
  url: string;
  fileName?: string;
  category?: string;
}

// Helper to check if running in Tauri context
const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
};

// Download commands
export const downloadApi = {
  addDownload: async (request: AddDownloadRequest): Promise<Download> => {
    if (!isTauri()) {
      throw new Error('Not running in Tauri context. Please use: npm run tauri dev');
    }
    return await invoke<Download>('add_download', { request });
  },

  pauseDownload: async (id: string): Promise<void> => {
    return await invoke('pause_download', { id });
  },

  resumeDownload: async (id: string): Promise<void> => {
    return await invoke('resume_download', { id });
  },

  cancelDownload: async (id: string): Promise<void> => {
    return await invoke('cancel_download', { id });
  },

  removeDownload: async (id: string, deleteFile: boolean = false): Promise<void> => {
    return await invoke('remove_download', { id, deleteFile });
  },

  retryDownload: async (id: string): Promise<void> => {
    return await invoke('retry_download', { id });
  },

  getAllDownloads: async (): Promise<Download[]> => {
    return await invoke<Download[]>('get_all_downloads');
  },

  getDownloadProgress: async (id: string): Promise<DownloadProgress | null> => {
    return await invoke<DownloadProgress | null>('get_download_progress', { id });
  },

  getFileInfo: async (url: string): Promise<FileInfo> => {
    return await invoke<FileInfo>('get_file_info', { url });
  },

  addBatchDownloads: async (urls: string[], savePath?: string): Promise<Download[]> => {
    return await invoke<Download[]>('add_batch_downloads', { urls, savePath });
  },

  pauseAll: async (): Promise<void> => {
    return await invoke('pause_all');
  },

  resumeAll: async (): Promise<void> => {
    return await invoke('resume_all');
  },

  cancelAll: async (): Promise<void> => {
    return await invoke('cancel_all');
  },

  openFile: async (id: string): Promise<void> => {
    return await invoke('open_file', { id });
  },

  openFileLocation: async (id: string): Promise<void> => {
    try {
      return await invoke('open_file_location', { id });
    } catch (error) {
      // Fallback: try to open using shell plugin
      console.log("Fallback: Attempting to open file location with alternative method");
      throw error;
    }
  },

  getGlobalStats: async (): Promise<DownloadStats> => {
    return await invoke<DownloadStats>('get_global_stats');
  },

  setSpeedLimit: async (limit: number | null): Promise<void> => {
    return await invoke('set_speed_limit', { limit });
  },

  getQueueInfo: async (): Promise<QueueInfo> => {
    return await invoke<QueueInfo>('get_queue_info');
  },

  setMaxConcurrent: async (max: number): Promise<void> => {
    return await invoke('set_max_concurrent', { max });
  },
};

// YouTube/Video download commands
export const youtubeApi = {
  checkYtDlpInstalled: async (): Promise<boolean> => {
    return await invoke<boolean>('check_ytdlp_installed');
  },

  getVideoInfo: async (url: string): Promise<VideoInfo> => {
    return await invoke<VideoInfo>('get_video_info', { url });
  },

  getVideoQualities: async (url: string): Promise<QualityOption[]> => {
    return await invoke<QualityOption[]>('get_video_qualities', { url });
  },

  checkIsPlaylist: async (url: string): Promise<boolean> => {
    return await invoke<boolean>('check_is_playlist', { url });
  },

  downloadVideo: async (options: YouTubeDownloadOptions): Promise<Download> => {
    return await invoke<Download>('add_download', { request: options });
  },
};

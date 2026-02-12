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
      // Mock response for development
      console.log('Mock: addDownload called with:', request);
      const mockDownload: Download = {
        id: Date.now().toString(),
        url: request.url,
        finalUrl: request.url,
        fileName: request.file_name || request.url.split('/').pop() || 'unknown',
        savePath: request.save_path || '/downloads',
        totalSize: null,
        downloadedSize: 0,
        status: 'queued',
        speed: 0,
        eta: null,
        segments: request.segments || 4,
        supportsRange: true,
        contentType: 'application/octet-stream',
        etag: null,
        expectedChecksum: request.expected_checksum || null,
        actualChecksum: null,
        checksumAlgorithm: request.checksum_type as any || null,
        retryCount: 0,
        errorMessage: null,
        createdAt: new Date().toISOString(),
        completedAt: null,
        priority: request.priority || 0,
        category: request.category || null,
      };
      return mockDownload;
    }
    return await invoke<Download>('add_download', { request });
  },

  pauseDownload: async (id: string): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: pauseDownload called with:', id);
      return;
    }
    return await invoke('pause_download', { id });
  },

  resumeDownload: async (id: string): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: resumeDownload called with:', id);
      return;
    }
    return await invoke('resume_download', { id });
  },

  cancelDownload: async (id: string): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: cancelDownload called with:', id);
      return;
    }
    return await invoke('cancel_download', { id });
  },

  removeDownload: async (id: string, deleteFile: boolean = false): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: removeDownload called with:', id, deleteFile);
      return;
    }
    return await invoke('remove_download', { id, deleteFile });
  },

  retryDownload: async (id: string): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: retryDownload called with:', id);
      return;
    }
    return await invoke('retry_download', { id });
  },

  getAllDownloads: async (): Promise<Download[]> => {
    if (!isTauri()) {
      console.log('Mock: getAllDownloads called');
      return [];
    }
    return await invoke<Download[]>('get_all_downloads');
  },

  getDownloadProgress: async (id: string): Promise<DownloadProgress | null> => {
    if (!isTauri()) {
      console.log('Mock: getDownloadProgress called with:', id);
      return null;
    }
    return await invoke<DownloadProgress | null>('get_download_progress', { id });
  },

  getFileInfo: async (url: string): Promise<FileInfo> => {
    if (!isTauri()) {
      console.log('Mock: getFileInfo called with:', url);
      return {
        fileName: url.split('/').pop() || 'unknown',
        contentType: 'application/octet-stream',
        totalSize: null,
        supportsRange: true,
      };
    }
    return await invoke<FileInfo>('get_file_info', { url });
  },

  addBatchDownloads: async (urls: string[], savePath?: string): Promise<Download[]> => {
    if (!isTauri()) {
      console.log('Mock: addBatchDownloads called with:', urls, savePath);
      return [];
    }
    return await invoke<Download[]>('add_batch_downloads', { urls, savePath });
  },

  pauseAll: async (): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: pauseAll called');
      return;
    }
    return await invoke('pause_all');
  },

  resumeAll: async (): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: resumeAll called');
      return;
    }
    return await invoke('resume_all');
  },

  cancelAll: async (): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: cancelAll called');
      return;
    }
    return await invoke('cancel_all');
  },

  openFile: async (id: string): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: openFile called with:', id);
      return;
    }
    return await invoke('open_file', { id });
  },

  openFileLocation: async (id: string): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: openFileLocation called with:', id);
      return;
    }
    try {
      return await invoke('open_file_location', { id });
    } catch (error) {
      // Fallback: try to open using shell plugin
      console.log("Fallback: Attempting to open file location with alternative method");
      throw error;
    }
  },

  getGlobalStats: async (): Promise<DownloadStats> => {
    if (!isTauri()) {
      console.log('Mock: getGlobalStats called');
      return {
        totalDownloads: 0,
        activeDownloads: 0,
        completedDownloads: 0,
        failedDownloads: 0,
        pausedDownloads: 0,
        queuedDownloads: 0,
        totalDownloaded: 0,
        currentSpeed: 0,
        totalSpeed: 0,
      };
    }
    return await invoke<DownloadStats>('get_global_stats');
  },

  setSpeedLimit: async (limit: number | null): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: setSpeedLimit called with:', limit);
      return;
    }
    return await invoke('set_speed_limit', { limit });
  },

  getQueueInfo: async (): Promise<QueueInfo> => {
    if (!isTauri()) {
      console.log('Mock: getQueueInfo called');
      return {
        maxConcurrent: 3,
        currentActive: 0,
        queuedCount: 0,
        activeCount: 0,
      };
    }
    return await invoke<QueueInfo>('get_queue_info');
  },

  setMaxConcurrent: async (max: number): Promise<void> => {
    if (!isTauri()) {
      console.log('Mock: setMaxConcurrent called with:', max);
      return;
    }
    return await invoke('set_max_concurrent', { max });
  },

  checkFileExists: async (id: string): Promise<boolean> => {
    if (!isTauri()) {
      console.log('Mock: checkFileExists called with:', id);
      return true;
    }
    return await invoke<boolean>('check_file_exists', { id });
  },
};

// YouTube/Video download commands
export const youtubeApi = {
  checkYtDlpInstalled: async (): Promise<boolean> => {
    if (!isTauri()) {
      console.log('Mock: checkYtDlpInstalled called');
      return false;
    }
    return await invoke<boolean>('check_ytdlp_installed');
  },

  getVideoInfo: async (url: string): Promise<VideoInfo> => {
    if (!isTauri()) {
      console.log('Mock: getVideoInfo called with:', url);
      return {
        title: 'Mock Video Title',
        duration: 300,
        filesize: 50000000,
        thumbnail: 'https://example.com/thumbnail.jpg',
        uploader: 'Mock Uploader',
        upload_date: '2023-01-01',
        view_count: 1000000,
        is_playlist: false,
        playlist_count: null,
      };
    }
    return await invoke<VideoInfo>('get_video_info', { url });
  },

  getVideoQualities: async (url: string): Promise<QualityOption[]> => {
    if (!isTauri()) {
      console.log('Mock: getVideoQualities called with:', url);
      return [
        { format_id: '22', resolution: '720p', ext: 'mp4', filesize: 50000000, fps: 30, has_audio: true },
        { format_id: '137', resolution: '1080p', ext: 'mp4', filesize: 100000000, fps: 30, has_audio: false },
      ];
    }
    return await invoke<QualityOption[]>('get_video_qualities', { url });
  },

  checkIsPlaylist: async (url: string): Promise<boolean> => {
    if (!isTauri()) {
      console.log('Mock: checkIsPlaylist called with:', url);
      return url.includes('playlist') || url.includes('watch?v=');
    }
    return await invoke<boolean>('check_is_playlist', { url });
  },

  downloadVideo: async (options: YouTubeDownloadOptions): Promise<Download> => {
    if (!isTauri()) {
      console.log('Mock: downloadVideo called with:', options);
      const mockDownload: Download = {
        id: Date.now().toString(),
        url: options.url,
        finalUrl: options.url,
        fileName: options.file_name || 'youtube-video.mp4',
        savePath: options.save_path || '/downloads',
        totalSize: null,
        downloadedSize: 0,
        status: 'queued',
        speed: 0,
        eta: null,
        segments: 4,
        supportsRange: true,
        contentType: 'video/mp4',
        etag: null,
        expectedChecksum: null,
        actualChecksum: null,
        checksumAlgorithm: null,
        retryCount: 0,
        errorMessage: null,
        createdAt: new Date().toISOString(),
        completedAt: null,
        priority: options.priority || 0,
        category: options.category || 'youtube',
      };
      return mockDownload;
    }
    return await invoke<Download>('add_download', { request: options });
  },
};

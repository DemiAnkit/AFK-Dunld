import { useCallback, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Download, DownloadProgress, FileInfo, BatchDownloadItem, DownloadStats, QueueInfo } from '../types/download';

export interface UseDownloadsReturn {
  downloads: Download[];
  loading: boolean;
  error: string | null;
  stats: DownloadStats | null;
  queueInfo: QueueInfo | null;
  refreshDownloads: () => Promise<void>;
  addDownload: (url: string, savePath?: string, fileName?: string) => Promise<string>;
  addBatchDownloads: (items: BatchDownloadItem[]) => Promise<string[]>;
  pauseDownload: (id: string) => Promise<void>;
  resumeDownload: (id: string) => Promise<void>;
  cancelDownload: (id: string) => Promise<void>;
  removeDownload: (id: string, deleteFile?: boolean) => Promise<void>;
  retryDownload: (id: string) => Promise<void>;
  getFileInfo: (url: string) => Promise<FileInfo>;
  pauseAll: () => Promise<void>;
  resumeAll: () => Promise<void>;
  cancelAll: () => Promise<void>;
  openFile: (id: string) => Promise<void>;
  openFileLocation: (id: string) => Promise<void>;
  setSpeedLimit: (id: string | null, limit: number) => Promise<void>;
  setMaxConcurrent: (count: number) => Promise<void>;
  getDownloadProgress: (id: string) => Promise<DownloadProgress>;
}

export function useDownloads(): UseDownloadsReturn {
  const [downloads, setDownloads] = useState<Download[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [stats, setStats] = useState<DownloadStats | null>(null);
  const [queueInfo, setQueueInfo] = useState<QueueInfo | null>(null);

  const refreshDownloads = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await invoke<Download[]>('get_all_downloads');
      setDownloads(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch downloads');
    } finally {
      setLoading(false);
    }
  }, []);

  const refreshStats = useCallback(async () => {
    try {
      const data = await invoke<DownloadStats>('get_global_stats');
      setStats(data);
    } catch (err) {
      console.error('Failed to fetch stats:', err);
    }
  }, []);

  const refreshQueueInfo = useCallback(async () => {
    try {
      const data = await invoke<QueueInfo>('get_queue_info');
      setQueueInfo(data);
    } catch (err) {
      console.error('Failed to fetch queue info:', err);
    }
  }, []);

  useEffect(() => {
    refreshDownloads();
    refreshStats();
    refreshQueueInfo();
  }, [refreshDownloads, refreshStats, refreshQueueInfo]);

  const addDownload = useCallback(async (
    url: string, 
    savePath?: string, 
    fileName?: string
  ): Promise<string> => {
    try {
      const id = await invoke<string>('add_download', { 
        url, 
        savePath, 
        fileName 
      });
      await refreshDownloads();
      return id;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to add download');
    }
  }, [refreshDownloads]);

  const addBatchDownloads = useCallback(async (
    items: BatchDownloadItem[]
  ): Promise<string[]> => {
    try {
      const ids = await invoke<string[]>('add_batch_downloads', { items });
      await refreshDownloads();
      return ids;
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to add batch downloads');
    }
  }, [refreshDownloads]);

  const pauseDownload = useCallback(async (id: string): Promise<void> => {
    try {
      await invoke('pause_download', { id });
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to pause download');
    }
  }, [refreshDownloads]);

  const resumeDownload = useCallback(async (id: string): Promise<void> => {
    try {
      await invoke('resume_download', { id });
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to resume download');
    }
  }, [refreshDownloads]);

  const cancelDownload = useCallback(async (id: string): Promise<void> => {
    try {
      await invoke('cancel_download', { id });
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to cancel download');
    }
  }, [refreshDownloads]);

  const removeDownload = useCallback(async (
    id: string, 
    deleteFile: boolean = false
  ): Promise<void> => {
    try {
      await invoke('remove_download', { id, deleteFile });
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to remove download');
    }
  }, [refreshDownloads]);

  const retryDownload = useCallback(async (id: string): Promise<void> => {
    try {
      await invoke('retry_download', { id });
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to retry download');
    }
  }, [refreshDownloads]);

  const getFileInfo = useCallback(async (url: string): Promise<FileInfo> => {
    try {
      return await invoke<FileInfo>('get_file_info', { url });
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to get file info');
    }
  }, []);

  const pauseAll = useCallback(async (): Promise<void> => {
    try {
      await invoke('pause_all');
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to pause all downloads');
    }
  }, [refreshDownloads]);

  const resumeAll = useCallback(async (): Promise<void> => {
    try {
      await invoke('resume_all');
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to resume all downloads');
    }
  }, [refreshDownloads]);

  const cancelAll = useCallback(async (): Promise<void> => {
    try {
      await invoke('cancel_all');
      await refreshDownloads();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to cancel all downloads');
    }
  }, [refreshDownloads]);

  const openFile = useCallback(async (id: string): Promise<void> => {
    try {
      await invoke('open_file', { id });
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to open file');
    }
  }, []);

  const openFileLocation = useCallback(async (id: string): Promise<void> => {
    try {
      await invoke('open_file_location', { id });
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to open file location');
    }
  }, []);

  const setSpeedLimit = useCallback(async (
    id: string | null, 
    limit: number
  ): Promise<void> => {
    try {
      await invoke('set_speed_limit', { id, limit });
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to set speed limit');
    }
  }, []);

  const setMaxConcurrent = useCallback(async (count: number): Promise<void> => {
    try {
      await invoke('set_max_concurrent', { count });
      await refreshQueueInfo();
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to set max concurrent');
    }
  }, [refreshQueueInfo]);

  const getDownloadProgress = useCallback(async (id: string): Promise<DownloadProgress> => {
    try {
      return await invoke<DownloadProgress>('get_download_progress', { id });
    } catch (err) {
      throw new Error(err instanceof Error ? err.message : 'Failed to get download progress');
    }
  }, []);

  return {
    downloads,
    loading,
    error,
    stats,
    queueInfo,
    refreshDownloads,
    addDownload,
    addBatchDownloads,
    pauseDownload,
    resumeDownload,
    cancelDownload,
    removeDownload,
    retryDownload,
    getFileInfo,
    pauseAll,
    resumeAll,
    cancelAll,
    openFile,
    openFileLocation,
    setSpeedLimit,
    setMaxConcurrent,
    getDownloadProgress,
  };
}

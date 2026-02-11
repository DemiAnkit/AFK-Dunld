// src/services/downloadService.ts
import { downloadApi, type AddDownloadRequest } from './tauriApi';
import type { Download, DownloadProgress, FileInfo } from '../types/download';

export class DownloadService {
  async addDownload(url: string, options?: Partial<AddDownloadRequest>): Promise<Download> {
    const request: AddDownloadRequest = {
      url,
      ...options,
    };
    return await downloadApi.addDownload(request);
  }

  async addBatchDownloads(urls: string[], savePath?: string): Promise<Download[]> {
    return await downloadApi.addBatchDownloads(urls, savePath);
  }

  async pauseDownload(id: string): Promise<void> {
    return await downloadApi.pauseDownload(id);
  }

  async resumeDownload(id: string): Promise<void> {
    return await downloadApi.resumeDownload(id);
  }

  async cancelDownload(id: string): Promise<void> {
    return await downloadApi.cancelDownload(id);
  }

  async removeDownload(id: string, deleteFile: boolean = false): Promise<void> {
    return await downloadApi.removeDownload(id, deleteFile);
  }

  async retryDownload(id: string): Promise<void> {
    return await downloadApi.retryDownload(id);
  }

  async getAllDownloads(): Promise<Download[]> {
    return await downloadApi.getAllDownloads();
  }

  async getDownloadProgress(id: string): Promise<DownloadProgress | null> {
    return await downloadApi.getDownloadProgress(id);
  }

  async getFileInfo(url: string): Promise<FileInfo> {
    return await downloadApi.getFileInfo(url);
  }

  async pauseAll(): Promise<void> {
    return await downloadApi.pauseAll();
  }

  async resumeAll(): Promise<void> {
    return await downloadApi.resumeAll();
  }

  async cancelAll(): Promise<void> {
    return await downloadApi.cancelAll();
  }

  async openFile(id: string): Promise<void> {
    return await downloadApi.openFile(id);
  }

  async openFileLocation(id: string): Promise<void> {
    return await downloadApi.openFileLocation(id);
  }

  async setSpeedLimit(limit: number | null): Promise<void> {
    return await downloadApi.setSpeedLimit(limit);
  }

  async setMaxConcurrent(max: number): Promise<void> {
    return await downloadApi.setMaxConcurrent(max);
  }
}

export const downloadService = new DownloadService();

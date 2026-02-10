export type DownloadStatus = 
  | 'pending' 
  | 'downloading' 
  | 'paused' 
  | 'completed' 
  | 'failed' 
  | 'cancelled' 
  | 'queued';

export type ChecksumType = 'md5' | 'sha1' | 'sha256' | null;

export interface Download {
  id: string;
  url: string;
  fileName: string;
  savePath: string;
  totalSize: number | null;
  downloadedSize: number;
  status: DownloadStatus;
  segments: number;
  retries: number;
  maxRetries: number;
  supportsRange: boolean;
  contentType: string | null;
  etag: string | null;
  expectedChecksum: string | null;
  checksumType: ChecksumType;
  errorMessage: string | null;
  createdAt: string;
  completedAt: string | null;
  priority: number;
  speedLimit: number | null;
  category: string | null;
}

export interface DownloadProgress {
  id: string;
  downloadedBytes: number;
  totalBytes: number | null;
  speed: number; // bytes per second
  eta: number | null; // seconds
  progress: number; // 0-100
}

export interface FileInfo {
  fileName: string;
  contentType: string | null;
  totalSize: number | null;
  supportsRange: boolean;
  etag: string | null;
  lastModified: string | null;
}

export interface DownloadStats {
  totalDownloads: number;
  activeDownloads: number;
  completedDownloads: number;
  failedDownloads: number;
  pausedDownloads: number;
  queuedDownloads: number;
  totalDownloaded: number; // bytes
  currentSpeed: number; // bytes per second
}

export interface QueueInfo {
  maxConcurrent: number;
  currentActive: number;
  queuedCount: number;
}

export interface BatchDownloadItem {
  url: string;
  fileName?: string;
  category?: string;
}

export interface DownloadFilter {
  status?: DownloadStatus | DownloadStatus[];
  category?: string;
  search?: string;
  dateFrom?: string;
  dateTo?: string;
}

export interface DownloadSort {
  field: 'createdAt' | 'completedAt' | 'fileName' | 'progress' | 'priority';
  order: 'asc' | 'desc';
}

// Download status - matches Rust backend DownloadStatus enum
export type DownloadStatus = 
  | 'queued'
  | 'connecting'
  | 'downloading' 
  | 'paused' 
  | 'completed' 
  | 'failed' 
  | 'cancelled'
  | 'merging'
  | 'verifying';

export type ChecksumType = 'md5' | 'sha1' | 'sha256' | null;

// Main Download interface - matches Rust DownloadTask
export interface Download {
  id: string;
  url: string;
  finalUrl: string | null;
  fileName: string;
  savePath: string;
  totalSize: number | null;
  downloadedSize: number;
  status: DownloadStatus;
  speed: number;
  eta: number | null;
  segments: number;
  supportsRange: boolean;
  contentType: string | null;
  etag: string | null;
  expectedChecksum: string | null;
  actualChecksum: string | null;
  checksumAlgorithm: ChecksumType;
  retryCount: number;
  errorMessage: string | null;
  createdAt: string;
  completedAt: string | null;
  priority: number;
  category: string | null;
}

// Download progress event - matches Rust DownloadProgress
export interface DownloadProgress {
  id: string;
  downloadedSize: number;
  totalSize: number | null;
  speed: number; // bytes per second
  eta: number | null; // seconds
  status: DownloadStatus;
  percent: number; // 0-100
  errorMessage: string | null;
}

// File info - matches Rust FileInfo
export interface FileInfo {
  fileName: string;
  contentType: string | null;
  totalSize: number | null;
  supportsRange: boolean;
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

export interface QueueInfo {
  activeCount: number;
  queuedCount: number;
  maxConcurrent: number;
}

export interface DownloadStats {
  totalSpeed: number;
  totalDownloaded: number;
  activeDownloads: number;
  completedDownloads: number;
  failedDownloads: number;
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

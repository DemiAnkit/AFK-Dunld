// src/components/downloads/DownloadItem.tsx
import { 
    Pause, Play, X, Trash2, RotateCcw, 
    FolderOpen, CheckCircle2,
    AlertCircle, Clock, Loader2
  } from "lucide-react";
  import { useDownloadStore, Download } from "../../stores/downloadStore";
  import { formatBytes, formatSpeed, formatEta } from "../../utils/format";
  import { invoke } from "@tauri-apps/api/core";
  
  interface DownloadItemProps {
    download: Download;
  }
  
  export function DownloadItem({ download }: DownloadItemProps) {
    const { 
      pauseDownload, resumeDownload, cancelDownload, 
      removeDownload, retryDownload 
    } = useDownloadStore();
  
    const progress = download.total_bytes
      ? (download.downloaded_bytes / download.total_bytes) * 100
      : 0;
  
    const statusIcon = {
      Pending: <Clock className="w-4 h-4 text-gray-400" />,
      Queued: <Clock className="w-4 h-4 text-yellow-400" />,
      Downloading: <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />,
      Paused: <Pause className="w-4 h-4 text-orange-400" />,
      Completed: <CheckCircle2 className="w-4 h-4 text-green-400" />,
      Failed: <AlertCircle className="w-4 h-4 text-red-400" />,
      Cancelled: <X className="w-4 h-4 text-gray-400" />,
      Merging: <Loader2 className="w-4 h-4 text-purple-400 animate-spin" />,
      Verifying: <Loader2 className="w-4 h-4 text-cyan-400 animate-spin" />,
    }[download.status];
  
    const statusColor = {
      Pending: "bg-gray-500",
      Queued: "bg-yellow-500",
      Downloading: "bg-blue-500",
      Paused: "bg-orange-500",
      Completed: "bg-green-500",
      Failed: "bg-red-500",
      Cancelled: "bg-gray-500",
      Merging: "bg-purple-500",
      Verifying: "bg-cyan-500",
    }[download.status];

    const getFileTypeIcon = (fileName: string) => {
      const ext = fileName.split('.').pop()?.toLowerCase();
      if (['mp4', 'avi', 'mkv', 'mov', 'wmv'].includes(ext || '')) {
        return '🎥';
      }
      if (['mp3', 'wav', 'flac', 'aac'].includes(ext || '')) {
        return '🎵';
      }
      if (['jpg', 'jpeg', 'png', 'gif', 'webp'].includes(ext || '')) {
        return '🖼️';
      }
      if (['pdf', 'doc', 'docx', 'txt'].includes(ext || '')) {
        return '📄';
      }
      if (['zip', 'rar', '7z', 'tar', 'gz'].includes(ext || '')) {
        return '📦';
      }
      return '📁';
    };
  
    return (
      <div className="grid grid-cols-12 gap-4 px-4 py-3 items-center 
                      border-b border-gray-800/50 hover:bg-gray-900/50 
                      transition-colors group">
        {/* Checkbox + File Name */}
        <div className="col-span-5 flex items-center gap-3 min-w-0">
          <input
            type="checkbox"
            className="w-3.5 h-3.5 bg-gray-700 border-gray-600 rounded text-blue-600 focus:ring-blue-500 focus:ring-2 flex-shrink-0"
          />
          <div className="flex-shrink-0 text-lg">{getFileTypeIcon(download.file_name)}</div>
          <div className="flex-shrink-0">{statusIcon}</div>
          <div className="min-w-0 flex-1">
            <p className="text-sm font-medium text-white truncate">
              {download.file_name}
            </p>
            <p className="text-xs text-gray-500 truncate">
              {download.url}
            </p>
          </div>
        </div>
  
        {/* Size */}
        <div className="col-span-2 text-sm text-gray-300">
          <div className="flex flex-col">
            <span>{download.total_bytes ? formatBytes(download.total_bytes) : formatBytes(download.downloaded_bytes)}</span>
            {download.status !== "Completed" && download.total_bytes && (
              <span className="text-xs text-gray-500">
                {formatBytes(download.downloaded_bytes)} downloaded
              </span>
            )}
          </div>
        </div>
  
        {/* Progress Bar */}
        <div className="col-span-2">
          <div className="flex items-center gap-2">
            <div className="flex-1 h-2 bg-gray-800 rounded-full overflow-hidden">
              <div
                className={`h-full ${statusColor} rounded-full transition-all 
                           duration-300 ease-out`}
                style={{ width: `${Math.min(progress, 100)}%` }}
              />
            </div>
            <span className="text-xs text-gray-400 w-12 text-right">
              {progress.toFixed(1)}%
            </span>
          </div>
        </div>
  
        {/* Speed */}
        <div className="col-span-1 text-sm text-gray-300">
          <div className="flex flex-col">
            {download.status === "Downloading" ? (
              <>
                <span>{formatSpeed(download.speed)}</span>
                <span className="text-xs text-gray-500">current</span>
              </>
            ) : (
              "-"
            )}
          </div>
        </div>
  
        {/* Time Left */}
        <div className="col-span-1 text-sm text-gray-300">
          {download.status === "Downloading" && download.eta ? (
            formatEta(download.eta)
          ) : (
            <span className="text-gray-500">-</span>
          )}
        </div>
  
        {/* Actions */}
        <div className="col-span-1 flex items-center justify-end gap-1">
          {download.status === "Downloading" && (
            <button
              onClick={() => pauseDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Pause"
            >
              <Pause className="w-3.5 h-3.5 text-orange-400" />
            </button>
          )}
  
          {download.status === "Paused" && (
            <button
              onClick={() => resumeDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Resume"
            >
              <Play className="w-3.5 h-3.5 text-green-400" />
            </button>
          )}
  
          {["Downloading", "Paused", "Queued"].includes(download.status) && (
            <button
              onClick={() => cancelDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Cancel"
            >
              <X className="w-3.5 h-3.5 text-red-400" />
            </button>
          )}
  
          {["Failed", "Cancelled"].includes(download.status) && (
            <button
              onClick={() => retryDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Retry"
            >
              <RotateCcw className="w-3.5 h-3.5 text-blue-400" />
            </button>
          )}
  
          {download.status === "Completed" && (
            <button
              onClick={() => invoke("open_folder", { 
                path: download.file_path 
              })}
              className="p-1.5 hover:bg-gray-700 rounded transition-colors"
              title="Open Folder"
            >
              <FolderOpen className="w-3.5 h-3.5 text-blue-400" />
            </button>
          )}
  
          <button
            onClick={() => removeDownload(download.id)}
            className="p-1.5 hover:bg-gray-700 rounded transition-colors 
                       opacity-0 group-hover:opacity-100"
            title="Remove"
          >
            <Trash2 className="w-3.5 h-3.5 text-gray-400" />
          </button>
        </div>
      </div>
    );
  }

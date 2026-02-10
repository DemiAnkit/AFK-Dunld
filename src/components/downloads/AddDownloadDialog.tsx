// src/components/downloads/DownloadItem.tsx
import { 
    Pause, Play, X, Trash2, RotateCcw, 
    FolderOpen, File, MoreVertical, CheckCircle2,
    AlertCircle, Clock, Loader2
  } from "lucide-react";
  import { useDownloadStore, Download } from "../../stores/downloadStore";
  import { formatBytes, formatSpeed, formatEta } from "../../utils/format";
  import { useState } from "react";
  
  interface DownloadItemProps {
    download: Download;
  }
  
  export function DownloadItem({ download }: DownloadItemProps) {
    const { 
      pauseDownload, resumeDownload, cancelDownload, 
      removeDownload, retryDownload 
    } = useDownloadStore();
    const [showMenu, setShowMenu] = useState(false);
  
    const progress = download.total_size
      ? (download.downloaded_size / download.total_size) * 100
      : 0;
  
    const statusIcon = {
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
      Queued: "bg-yellow-500",
      Downloading: "bg-blue-500",
      Paused: "bg-orange-500",
      Completed: "bg-green-500",
      Failed: "bg-red-500",
      Cancelled: "bg-gray-500",
      Merging: "bg-purple-500",
      Verifying: "bg-cyan-500",
    }[download.status];
  
    return (
      <div className="grid grid-cols-12 gap-4 px-4 py-3 items-center 
                      border-b border-gray-800/50 hover:bg-gray-900/50 
                      transition-colors group">
        {/* File Name */}
        <div className="col-span-4 flex items-center gap-3 min-w-0">
          <div className="flex-shrink-0">{statusIcon}</div>
          <div className="min-w-0">
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
          {download.total_size
            ? `${formatBytes(download.downloaded_size)} / ${formatBytes(download.total_size)}`
            : formatBytes(download.downloaded_size)}
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
          {download.status === "Downloading"
            ? formatSpeed(download.speed)
            : "-"}
        </div>
  
        {/* ETA */}
        <div className="col-span-1 text-sm text-gray-300">
          {download.status === "Downloading" && download.eta
            ? formatEta(download.eta)
            : "-"}
        </div>
  
        {/* Actions */}
        <div className="col-span-2 flex items-center justify-center gap-1">
          {download.status === "Downloading" && (
            <button
              onClick={() => pauseDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded-md transition-colors"
              title="Pause"
            >
              <Pause className="w-4 h-4 text-orange-400" />
            </button>
          )}
  
          {download.status === "Paused" && (
            <button
              onClick={() => resumeDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded-md transition-colors"
              title="Resume"
            >
              <Play className="w-4 h-4 text-green-400" />
            </button>
          )}
  
          {["Downloading", "Paused", "Queued"].includes(download.status) && (
            <button
              onClick={() => cancelDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded-md transition-colors"
              title="Cancel"
            >
              <X className="w-4 h-4 text-red-400" />
            </button>
          )}
  
          {["Failed", "Cancelled"].includes(download.status) && (
            <button
              onClick={() => retryDownload(download.id)}
              className="p-1.5 hover:bg-gray-700 rounded-md transition-colors"
              title="Retry"
            >
              <RotateCcw className="w-4 h-4 text-blue-400" />
            </button>
          )}
  
          {download.status === "Completed" && (
            <button
              onClick={() => invoke("open_folder", { 
                path: download.save_path 
              })}
              className="p-1.5 hover:bg-gray-700 rounded-md transition-colors"
              title="Open Folder"
            >
              <FolderOpen className="w-4 h-4 text-blue-400" />
            </button>
          )}
  
          <button
            onClick={() => removeDownload(download.id, false)}
            className="p-1.5 hover:bg-gray-700 rounded-md transition-colors 
                       opacity-0 group-hover:opacity-100"
            title="Remove"
          >
            <Trash2 className="w-4 h-4 text-gray-400" />
          </button>
        </div>
      </div>
    );
  }
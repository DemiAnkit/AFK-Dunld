// src/components/dialogs/FileDetailsDialog.tsx
import { X, File, Download as DownloadIcon, Calendar, FolderOpen, Link as LinkIcon, Gauge } from "lucide-react";
import { Download } from "../../stores/downloadStore";
import { formatBytes, formatSpeed } from "../../utils/format";
import { format } from "date-fns";

interface FileDetailsDialogProps {
  download: Download;
  onClose: () => void;
}

export function FileDetailsDialog({ download, onClose }: FileDetailsDialogProps) {
  const formatDateTime = (dateString: string) => {
    try {
      return format(new Date(dateString), 'PPpp');
    } catch {
      return dateString;
    }
  };

  return (
    <div 
      className="fixed inset-0 flex items-center justify-center z-50 p-4"
      style={{ backgroundColor: 'rgba(0, 0, 0, 0.75)' }}
      onClick={onClose}
    >
      <div 
        className="bg-gray-900 rounded-xl shadow-2xl w-full max-w-2xl border border-gray-800"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-800">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-blue-600/20 rounded-lg">
              <File className="w-6 h-6 text-blue-500" />
            </div>
            <div>
              <h2 className="text-xl font-semibold text-white">File Details</h2>
              <p className="text-sm text-gray-400">Download information</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-800 rounded-lg transition-colors group"
          >
            <X size={20} className="text-gray-400 group-hover:text-white" />
          </button>
        </div>

        {/* Body */}
        <div className="p-6 space-y-4">
          {/* File Name */}
          <div className="flex items-start gap-3">
            <File className="w-5 h-5 text-gray-400 mt-0.5" />
            <div className="flex-1">
              <p className="text-sm text-gray-400">File Name</p>
              <p className="text-white font-medium break-all">{download.fileName || 'Unknown'}</p>
            </div>
          </div>

          {/* Download Status */}
          <div className="flex items-start gap-3">
            <DownloadIcon className="w-5 h-5 text-gray-400 mt-0.5" />
            <div className="flex-1">
              <p className="text-sm text-gray-400">Download Status</p>
              <p className="text-white font-medium capitalize">{download.status}</p>
            </div>
          </div>

          {/* Size */}
          <div className="flex items-start gap-3">
            <Gauge className="w-5 h-5 text-gray-400 mt-0.5" />
            <div className="flex-1">
              <p className="text-sm text-gray-400">Size</p>
              <p className="text-white font-medium">
                {download.totalSize ? formatBytes(download.totalSize) : 
                 download.downloadedSize ? formatBytes(download.downloadedSize) : 'Unknown'}
              </p>
            </div>
          </div>

          {/* Speed */}
          {download.status === 'downloading' && download.speed && (
            <div className="flex items-start gap-3">
              <Gauge className="w-5 h-5 text-gray-400 mt-0.5" />
              <div className="flex-1">
                <p className="text-sm text-gray-400">Download Speed</p>
                <p className="text-white font-medium">{formatSpeed(download.speed)}</p>
              </div>
            </div>
          )}

          {/* Added Date/Time */}
          <div className="flex items-start gap-3">
            <Calendar className="w-5 h-5 text-gray-400 mt-0.5" />
            <div className="flex-1">
              <p className="text-sm text-gray-400">Added Date/Time</p>
              <p className="text-white font-medium">{formatDateTime(download.createdAt)}</p>
            </div>
          </div>

          {/* Location */}
          <div className="flex items-start gap-3">
            <FolderOpen className="w-5 h-5 text-gray-400 mt-0.5" />
            <div className="flex-1">
              <p className="text-sm text-gray-400">Location</p>
              <p className="text-white font-medium break-all">{download.savePath || 'Not set'}</p>
            </div>
          </div>

          {/* URL */}
          <div className="flex items-start gap-3">
            <LinkIcon className="w-5 h-5 text-gray-400 mt-0.5" />
            <div className="flex-1">
              <p className="text-sm text-gray-400">Download URL</p>
              <p className="text-blue-400 font-medium break-all text-sm">{download.url || 'Unknown'}</p>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end p-6 border-t border-gray-800">
          <button
            onClick={onClose}
            className="px-5 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors font-medium"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

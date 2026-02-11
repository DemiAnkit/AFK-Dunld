// src/components/downloads/DownloadTable.tsx
import { useDownloadStore } from "../../stores/downloadStore";
import { DownloadTableRow } from "./DownloadTableRow";

interface DownloadTableProps {
  filter: "all" | "downloading" | "completed" | "failed" | "missing" | "torrent" | "video" | "music";
}

export function DownloadTable({ filter }: DownloadTableProps) {
  const { downloads } = useDownloadStore();

  const filteredDownloads = downloads.filter((d) => {
    switch (filter) {
      case "downloading":
        return ["downloading", "connecting", "queued", "paused"].includes(d.status);
      case "completed":
        return d.status === "completed";
      case "failed":
        return ["failed", "cancelled"].includes(d.status);
      case "missing":
        return d.status === "failed";
      case "torrent":
        return d.category === "torrent";
      case "video":
        return d.category === "video" || /\.(mp4|avi|mkv|mov|wmv)$/i.test(d.fileName || '');
      case "music":
        return d.category === "music" || /\.(mp3|wav|flac|aac)$/i.test(d.fileName || '');
      default:
        return true;
    }
  });

  return (
    <div className="flex flex-col h-full bg-gray-950">
      {/* Table Header */}
      <div className="grid grid-cols-12 gap-4 px-4 py-2 bg-gray-900 border-b border-gray-800 text-xs font-semibold text-gray-400 uppercase tracking-wider">
        <div className="col-span-1 flex items-center">
          <input type="checkbox" className="w-3.5 h-3.5" title="Select All" />
        </div>
        <div className="col-span-4">File Name</div>
        <div className="col-span-1">Status</div>
        <div className="col-span-1">Size</div>
        <div className="col-span-1">Speed</div>
        <div className="col-span-2">Added Date/Time</div>
        <div className="col-span-2">Actions</div>
      </div>

      {/* Table Body */}
      <div className="flex-1 overflow-y-auto">
        {filteredDownloads.length === 0 ? (
          <div className="flex items-center justify-center h-full text-gray-500">
            <div className="text-center">
              <Download className="w-16 h-16 mx-auto mb-4 opacity-20" />
              <p className="text-lg">No downloads found</p>
              <p className="text-sm mt-2">Click "Add Download" to get started</p>
            </div>
          </div>
        ) : (
          filteredDownloads.map((download) => (
            <DownloadTableRow key={download.id} download={download} />
          ))
        )}
      </div>
    </div>
  );
}

function Download({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
    </svg>
  );
}

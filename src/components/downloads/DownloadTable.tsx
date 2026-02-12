// src/components/downloads/DownloadTable.tsx
import { useDownloadStore } from "../../stores/downloadStore";
import { useUIStore } from "../../stores/uiStore";
import { DownloadTableRow } from "./DownloadTableRow";
import { Download } from "lucide-react";

interface DownloadTableProps {
  filter: "all" | "downloading" | "completed" | "failed" | "missing" | "torrent" | "video" | "music" | "youtube";
}

export function DownloadTable({ filter }: DownloadTableProps) {
  const { downloads } = useDownloadStore();
  const { searchQuery } = useUIStore();

  // View mode is available for future grid view implementation
  // const { viewMode } = useUIStore();
  
  // Filter by status/category first
  let filteredDownloads = downloads.filter((d) => {
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
      case "youtube":
        return d.category === "youtube" || /(youtube|youtu\.be)/i.test(d.url || '');
      default:
        return true;
    }
  });

  // Then filter by search query
  if (searchQuery.trim()) {
    const query = searchQuery.toLowerCase();
    filteredDownloads = filteredDownloads.filter((d) => {
      const fileNameMatch = d.fileName?.toLowerCase().includes(query);
      const urlMatch = d.url?.toLowerCase().includes(query);
      const statusMatch = d.status?.toLowerCase().includes(query);
      const categoryMatch = d.category?.toLowerCase().includes(query);
      return fileNameMatch || urlMatch || statusMatch || categoryMatch;
    });
  }

  return (
    <div className="flex flex-col h-full bg-gray-950">
      {/* Table Header */}
      <div className="grid grid-cols-12 gap-4 px-4 py-3 bg-gray-900/80 border-b border-gray-800 text-xs font-semibold text-gray-400 uppercase tracking-wider backdrop-blur-sm">
        <div className="col-span-1 flex items-center">
          <input 
            type="checkbox" 
            className="w-3.5 h-3.5 rounded border-gray-600 bg-gray-800 text-blue-600 focus:ring-blue-500/20 focus:ring-offset-0" 
            title="Select All" 
          />
        </div>
        <div className="col-span-4">File Name</div>
        <div className="col-span-1">Status</div>
        <div className="col-span-1">Size</div>
        <div className="col-span-1">Speed</div>
        <div className="col-span-2">Added Date/Time</div>
        <div className="col-span-2 text-right pr-4">Actions</div>
      </div>

      {/* Table Body */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        {filteredDownloads.length === 0 ? (
          <div className="flex items-center justify-center h-full text-gray-500">
            <div className="text-center">
              <div className="w-20 h-20 mx-auto mb-4 rounded-full bg-gray-800/50 flex items-center justify-center">
                <Download className="w-10 h-10 opacity-30" />
              </div>
              <p className="text-lg font-medium text-gray-400">
                {searchQuery ? 'No downloads match your search' : 'No downloads found'}
              </p>
              <p className="text-sm mt-2 text-gray-600">
                {searchQuery ? 'Try a different search term' : 'Click "Add Download" to get started'}
              </p>
            </div>
          </div>
        ) : (
          <div className="divide-y divide-gray-800/30">
            {filteredDownloads.map((download) => (
              <DownloadTableRow key={download.id} download={download} />
            ))}
          </div>
        )}
      </div>
      
      {/* Results count footer */}
      {filteredDownloads.length > 0 && (
        <div className="px-4 py-2 bg-gray-900/50 border-t border-gray-800 text-xs text-gray-500">
          Showing {filteredDownloads.length} of {downloads.length} downloads
          {searchQuery && ` (filtered by "${searchQuery}")`}
        </div>
      )}
    </div>
  );
}

// src/components/downloads/DownloadList.tsx
import { useDownloadStore } from "../../stores/downloadStore";
import { DownloadItem } from "./DownloadItem";
import { motion, AnimatePresence } from "framer-motion";
import { Download as DownloadIcon } from "lucide-react";

interface DownloadListProps {
  filter: "all" | "downloading" | "completed" | "failed";
}

export function DownloadList({ filter }: DownloadListProps) {
  const { downloads } = useDownloadStore();

  const filteredDownloads = downloads.filter((d) => {
    switch (filter) {
      case "downloading":
        return ["downloading", "queued", "paused", "connecting"].includes(d.status);
      case "completed":
        return d.status === "completed";
      case "failed":
        return ["failed", "cancelled"].includes(d.status);
      default:
        return true;
    }
  });

  const getFilterTitle = () => {
    switch (filter) {
      case "downloading": return "Downloading";
      case "completed": return "Completed";
      case "failed": return "Failed";
      default: return "All Downloads";
    }
  };

  return (
    <div className="flex flex-col h-full p-6">
      {/* Header */}
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-white mb-2 flex items-center gap-3">
          <span className="bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent">
            {getFilterTitle()}
          </span>
        </h1>
        <div className="flex items-center gap-4 text-sm text-gray-400">
          <span className="font-medium">{filteredDownloads.length} {filteredDownloads.length === 1 ? 'item' : 'items'}</span>
        </div>
      </div>

      {/* Table Header */}
      <div className="border-b border-gray-800 mb-2">
        <div className="grid grid-cols-12 gap-4 px-4 py-3 text-xs font-semibold text-gray-400 uppercase tracking-wider">
          <div className="col-span-5 flex items-center gap-2">
            <input
              type="checkbox"
              className="w-3.5 h-3.5 bg-gray-700 border-gray-600 rounded text-blue-600 focus:ring-blue-500 focus:ring-2"
            />
            <span>File Name</span>
          </div>
          <div className="col-span-2">Size</div>
          <div className="col-span-2">Progress</div>
          <div className="col-span-1">Speed</div>
          <div className="col-span-1">Time Left</div>
          <div className="col-span-1 text-center">Actions</div>
        </div>
      </div>

      {/* Download Items */}
      <div className="flex-1 overflow-auto">
        <AnimatePresence>
          {filteredDownloads.length > 0 ? (
            filteredDownloads.map((download, index) => (
              <motion.div
                key={download.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, x: -100 }}
                transition={{ delay: index * 0.03 }}
              >
                <DownloadItem download={download} />
              </motion.div>
            ))
          ) : (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex flex-col items-center justify-center h-64 text-gray-500"
            >
              <DownloadIcon className="w-16 h-16 mb-4 opacity-20" />
              <p className="text-lg font-medium mb-1">No downloads found</p>
              <p className="text-sm">
                {filter === "all" ? "Add a URL to start downloading" : `No ${filter} downloads`}
              </p>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
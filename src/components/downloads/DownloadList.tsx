// src/components/downloads/DownloadList.tsx
import { useEffect } from "react";
import { useDownloadStore, Download, DownloadStatus } from "../../stores/downloadStore";
import { DownloadItem } from "./DownloadItem";
import { motion, AnimatePresence } from "framer-motion";
import { Download as DownloadIcon, Search } from "lucide-react";
import { useState } from "react";

interface DownloadListProps {
  filter: "all" | "downloading" | "completed" | "failed";
}

export function DownloadList({ filter }: DownloadListProps) {
  const { downloads, fetchDownloads } = useDownloadStore();
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    fetchDownloads();
  }, []);

  const filteredDownloads = downloads
    .filter((d) => {
      switch (filter) {
        case "downloading":
          return ["Downloading", "Queued", "Paused"].includes(d.status);
        case "completed":
          return d.status === "Completed";
        case "failed":
          return ["Failed", "Cancelled"].includes(d.status);
        default:
          return true;
      }
    })
    .filter((d) =>
      d.file_name.toLowerCase().includes(searchQuery.toLowerCase())
    );

  return (
    <div className="flex flex-col h-full">
      {/* Search Bar */}
      <div className="relative mb-4">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
        <input
          type="text"
          placeholder="Search downloads..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full pl-10 pr-4 py-2 bg-gray-900 border border-gray-700 
                     rounded-lg text-white placeholder-gray-500 
                     focus:outline-none focus:border-blue-500 transition-colors"
        />
      </div>

      {/* Column Headers */}
      <div className="grid grid-cols-12 gap-4 px-4 py-2 text-xs font-semibold 
                      text-gray-400 uppercase tracking-wider border-b border-gray-800">
        <div className="col-span-4">File Name</div>
        <div className="col-span-2">Size</div>
        <div className="col-span-2">Progress</div>
        <div className="col-span-1">Speed</div>
        <div className="col-span-1">ETA</div>
        <div className="col-span-2 text-center">Actions</div>
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
                transition={{ delay: index * 0.05 }}
              >
                <DownloadItem download={download} />
              </motion.div>
            ))
          ) : (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex flex-col items-center justify-center 
                         h-64 text-gray-500"
            >
              <DownloadIcon className="w-16 h-16 mb-4 opacity-20" />
              <p className="text-lg">No downloads yet</p>
              <p className="text-sm">
                Add a URL to start downloading
              </p>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
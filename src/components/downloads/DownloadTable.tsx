// src/components/downloads/DownloadTable.tsx
import { useState } from "react";
import { useDownloadStore } from "../../stores/downloadStore";
import { useUIStore } from "../../stores/uiStore";
import { DownloadTableRow } from "./DownloadTableRow";
import { DownloadGridView } from "./DownloadGridView";
import { Download, ArrowUpDown, ArrowUp, ArrowDown, Pause, Play, Trash2, X } from "lucide-react";
import { DownloadSort } from "../../types/download";

interface DownloadTableProps {
  filter: "all" | "downloading" | "completed" | "failed" | "missing" | "torrent" | "video" | "music" | "youtube";
}

export function DownloadTable({ filter }: DownloadTableProps) {
  const { downloads, pauseSelected, resumeSelected, removeSelected } = useDownloadStore();
  const { searchQuery, selectedDownloads, selectAll, clearSelection, viewMode } = useUIStore();
  const [sort, setSort] = useState<DownloadSort>({ field: "createdAt", order: "desc" });

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

  // Sort downloads - prioritize active downloads at top
  filteredDownloads = [...filteredDownloads].sort((a, b) => {
    // First, prioritize by status - active downloads at top (including paused)
    const activeStatuses = ['downloading', 'connecting', 'queued', 'paused'];
    const aIsActive = activeStatuses.includes(a.status);
    const bIsActive = activeStatuses.includes(b.status);
    
    if (aIsActive && !bIsActive) return -1;
    if (!aIsActive && bIsActive) return 1;
    
    // Then apply the selected sort
    let comparison = 0;
    switch (sort.field) {
      case "fileName":
        comparison = (a.fileName || "").localeCompare(b.fileName || "");
        break;
      case "createdAt":
        comparison = new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
        break;
      case "completedAt":
        const aCompleted = a.completedAt ? new Date(a.completedAt).getTime() : 0;
        const bCompleted = b.completedAt ? new Date(b.completedAt).getTime() : 0;
        comparison = aCompleted - bCompleted;
        break;
      case "progress":
        const aProgress = a.totalSize ? (a.downloadedSize / a.totalSize) : 0;
        const bProgress = b.totalSize ? (b.downloadedSize / b.totalSize) : 0;
        comparison = aProgress - bProgress;
        break;
      case "priority":
        comparison = (a.priority || 0) - (b.priority || 0);
        break;
      default:
        comparison = 0;
    }
    return sort.order === "asc" ? comparison : -comparison;
  });

  const handleSort = (field: DownloadSort["field"]) => {
    setSort((current) => ({
      field,
      order: current.field === field && current.order === "asc" ? "desc" : "asc",
    }));
  };

  const getSortIcon = (field: DownloadSort["field"]) => {
    if (sort.field !== field) {
      return <ArrowUpDown className="w-3 h-3 opacity-30" />;
    }
    return sort.order === "asc" ? (
      <ArrowUp className="w-3 h-3 text-blue-400" />
    ) : (
      <ArrowDown className="w-3 h-3 text-blue-400" />
    );
  };

  const handleSelectAll = () => {
    const selectionSize = selectedDownloads instanceof Set ? selectedDownloads.size : 0;
    if (selectionSize === filteredDownloads.length && filteredDownloads.length > 0) {
      clearSelection();
    } else {
      selectAll(filteredDownloads.map(d => d.id));
    }
  };

  const selectionSize = selectedDownloads instanceof Set ? selectedDownloads.size : 0;
  const isAllSelected = filteredDownloads.length > 0 && selectionSize === filteredDownloads.length;
  const isSomeSelected = selectionSize > 0 && selectionSize < filteredDownloads.length;

  const selectedIds = Array.from(selectedDownloads);
  const selectedDownloadsList = downloads.filter(d => selectedIds.includes(d.id));
  
  const canPauseSelected = selectedDownloadsList.some(d => 
    d.status === 'downloading' || d.status === 'connecting' || d.status === 'queued'
  );
  const canResumeSelected = selectedDownloadsList.some(d => d.status === 'paused');

  const handlePauseSelected = async () => {
    const toPause = selectedDownloadsList.filter(d => 
      d.status === 'downloading' || d.status === 'connecting' || d.status === 'queued'
    );
    if (toPause.length > 0) {
      await pauseSelected(toPause.map(d => d.id));
    }
  };

  const handleResumeSelected = async () => {
    const toResume = selectedDownloadsList.filter(d => d.status === 'paused');
    if (toResume.length > 0) {
      await resumeSelected(toResume.map(d => d.id));
    }
  };


  const handleRemoveSelected = async () => {
    if (selectedIds.length > 0) {
      if (window.confirm(`Remove ${selectedIds.length} download${selectedIds.length > 1 ? 's' : ''}?`)) {
        await removeSelected(selectedIds, false);
        clearSelection();
      }
    }
  };

  return (
    <div className="flex flex-col h-full bg-gray-50 dark:bg-gray-950">
      {/* Bulk Actions Bar - Shows when items are selected */}
      {selectionSize > 0 && (
        <div className="bg-blue-100 dark:bg-blue-900/50 border-b border-blue-200 dark:border-blue-700 px-4 py-3 flex items-center justify-between backdrop-blur-sm">
          <div className="flex items-center gap-3">
            <span className="text-sm font-medium text-blue-700 dark:text-blue-300">
              {selectionSize} selected
            </span>
            <div className="flex items-center gap-2">
              {canPauseSelected && (
                <button
                  onClick={handlePauseSelected}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-orange-100 dark:bg-orange-500/20 hover:bg-orange-200 dark:hover:bg-orange-500/30 
                           text-orange-700 dark:text-orange-300 rounded-lg transition-all duration-200 text-xs font-medium
                           border border-orange-200 dark:border-orange-500/30 hover:border-orange-300 dark:hover:border-orange-500/50"
                  title="Pause selected downloads (P)"
                >
                  <Pause className="w-3.5 h-3.5" />
                  <span>Pause</span>
                </button>
              )}
              {canResumeSelected && (
                <button
                  onClick={handleResumeSelected}
                  className="flex items-center gap-1.5 px-3 py-1.5 bg-green-100 dark:bg-green-500/20 hover:bg-green-200 dark:hover:bg-green-500/30 
                           text-green-700 dark:text-green-300 rounded-lg transition-all duration-200 text-xs font-medium
                           border border-green-200 dark:border-green-500/30 hover:border-green-300 dark:hover:border-green-500/50"
                  title="Resume selected downloads (R)"
                >
                  <Play className="w-3.5 h-3.5" />
                  <span>Resume</span>
                </button>
              )}
              <button
                onClick={handleRemoveSelected}
                className="flex items-center gap-1.5 px-3 py-1.5 bg-red-100 dark:bg-red-500/20 hover:bg-red-200 dark:hover:bg-red-500/30 
                         text-red-700 dark:text-red-300 rounded-lg transition-all duration-200 text-xs font-medium
                         border border-red-200 dark:border-red-500/30 hover:border-red-300 dark:hover:border-red-500/50"
                title="Remove selected downloads (Delete)"
              >
                <Trash2 className="w-3.5 h-3.5" />
                <span>Remove</span>
              </button>
            </div>
          </div>
          <button
            onClick={clearSelection}
            className="p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700/50 rounded-lg transition-colors"
            title="Clear selection (Esc)"
          >
            <X className="w-4 h-4 text-gray-600 dark:text-gray-400" />
          </button>
        </div>
      )}
      
      {/* Grid View */}
      {viewMode === 'grid' ? (
        <div className="flex-1 overflow-y-auto scrollbar-thin">
          {filteredDownloads.length === 0 ? (
            <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
              <div className="text-center">
                <div className="w-20 h-20 mx-auto mb-4 rounded-full bg-gray-200 dark:bg-gray-800/50 flex items-center justify-center">
                  <Download className="w-10 h-10 opacity-30" />
                </div>
                <p className="text-lg font-medium text-gray-600 dark:text-gray-400">
                  {searchQuery ? 'No downloads match your search' : 'No downloads found'}
                </p>
                <p className="text-sm mt-2 text-gray-500 dark:text-gray-500">
                  {searchQuery ? 'Try a different search term' : 'Click "Add Download" to get started'}
                </p>
              </div>
            </div>
          ) : (
            <DownloadGridView downloads={filteredDownloads} />
          )}
        </div>
      ) : (
        <>
          {/* Table Header - List View Only */}
          <div className="grid gap-4 px-4 py-3 bg-gray-100 dark:bg-gray-900/80 border-b border-gray-200 dark:border-gray-800 text-xs font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wider backdrop-blur-sm sticky top-0 z-10"
               style={{ gridTemplateColumns: 'auto 1fr 120px 100px 100px 180px 140px' }}>
            <div className="flex items-center justify-center">
              <input 
                type="checkbox"
                checked={isAllSelected}
                ref={(el) => {
                  if (el) el.indeterminate = isSomeSelected;
                }}
                onChange={handleSelectAll}
                className="w-3.5 h-3.5 rounded border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-blue-600 dark:text-blue-500 focus:ring-blue-500/20 focus:ring-offset-0 cursor-pointer" 
                title={isAllSelected ? "Deselect All" : "Select All"} 
              />
            </div>
            <button 
              onClick={() => handleSort("fileName")}
              className="flex items-center gap-2 hover:text-gray-900 dark:hover:text-white transition-colors text-left"
            >
              File Name
              {getSortIcon("fileName")}
            </button>
            <div className="flex items-center">Status</div>
            <div className="flex items-center">Size</div>
            <div className="flex items-center">Speed</div>
            <button 
              onClick={() => handleSort("createdAt")}
              className="flex items-center gap-2 hover:text-gray-900 dark:hover:text-white transition-colors text-left"
            >
              Date/Time
              {getSortIcon("createdAt")}
            </button>
            <div className="flex items-center justify-end pr-4">Actions</div>
          </div>

          {/* Table Body - List View */}
          <div className="flex-1 overflow-y-auto scrollbar-thin">
            {filteredDownloads.length === 0 ? (
              <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
                <div className="text-center">
                  <div className="w-20 h-20 mx-auto mb-4 rounded-full bg-gray-200 dark:bg-gray-800/50 flex items-center justify-center">
                    <Download className="w-10 h-10 opacity-30" />
                  </div>
                  <p className="text-lg font-medium text-gray-600 dark:text-gray-400">
                    {searchQuery ? 'No downloads match your search' : 'No downloads found'}
                  </p>
                  <p className="text-sm mt-2 text-gray-500 dark:text-gray-500">
                    {searchQuery ? 'Try a different search term' : 'Click "Add Download" to get started'}
                  </p>
                </div>
              </div>
            ) : (
              <div className="divide-y divide-gray-200 dark:divide-gray-800/30">
                {filteredDownloads.map((download) => (
                  <DownloadTableRow key={download.id} download={download} />
                ))}
              </div>
            )}
          </div>
        </>
      )}
      
      {/* Results count footer */}
      {filteredDownloads.length > 0 && (
        <div className="px-4 py-2 bg-gray-100 dark:bg-gray-900/50 border-t border-gray-200 dark:border-gray-800 text-xs text-gray-500 dark:text-gray-500">
          {viewMode === 'grid' ? (
            <span>Showing {filteredDownloads.length} of {downloads.length} downloads in grid view</span>
          ) : (
            <span>Showing {filteredDownloads.length} of {downloads.length} downloads{searchQuery && ` (filtered by "${searchQuery}")`}</span>
          )}
        </div>
      )}
    </div>
  );
}

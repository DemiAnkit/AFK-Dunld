// src/components/layout/Header.tsx
import { Plus, FolderOpen, Pause, Play, Download } from "lucide-react";
import { useUIStore } from "../../stores/uiStore";
import { useDownloadStore } from "../../stores/downloadStore";
import toast from "react-hot-toast";

export function Header() {
  const { setAddDialogOpen } = useUIStore();
  const { downloads, pauseAll, resumeAll } = useDownloadStore();

  const activeDownloads = downloads.filter(
    (d) => d.status === "downloading" || d.status === "queued" || d.status === "connecting"
  );
  
  const pausedDownloads = downloads.filter(d => d.status === "paused");

  const handlePauseAll = async () => {
    if (activeDownloads.length === 0) {
      toast.error("No active downloads to pause");
      return;
    }
    try {
      await pauseAll();
    } catch (error) {
      console.error("Failed to pause all:", error);
    }
  };

  const handleResumeAll = async () => {
    if (pausedDownloads.length === 0) {
      toast.error("No paused downloads to resume");
      return;
    }
    try {
      await resumeAll();
    } catch (error) {
      console.error("Failed to resume all:", error);
    }
  };

  const handleOpenDownloadFolder = () => {
    // TODO: Implement open download folder
    toast.success("Opening download folder...");
  };

  return (
    <header className="bg-gradient-to-r from-gray-900 via-gray-900 to-gray-800 border-b border-gray-800 px-6 py-5 shadow-lg">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          {/* App Icon/Logo */}
          <div className="p-3 bg-blue-600/20 rounded-xl border border-blue-500/30">
            <Download className="w-7 h-7 text-blue-400" />
          </div>
          
          {/* Title & Stats */}
          <div>
            <h1 className="text-2xl font-bold text-white flex items-center gap-2">
              AFK-Dunld
              <span className="text-sm font-normal text-gray-500">Download Manager</span>
            </h1>
            <div className="flex items-center gap-4 mt-1">
              <p className="text-sm text-gray-400 flex items-center gap-1.5">
                <div className={`w-2 h-2 rounded-full ${activeDownloads.length > 0 ? 'bg-green-500 animate-pulse' : 'bg-gray-600'}`} />
                {activeDownloads.length > 0
                  ? `${activeDownloads.length} active download${activeDownloads.length !== 1 ? "s" : ""}`
                  : "No active downloads"}
              </p>
              <span className="text-gray-700">•</span>
              <p className="text-sm text-gray-400">
                {downloads.length} total
              </p>
            </div>
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-2">
          <button
            onClick={() => setAddDialogOpen(true)}
            className="flex items-center gap-2 px-5 py-2.5 bg-blue-600 hover:bg-blue-700 
                       text-white rounded-lg transition-all font-medium shadow-lg shadow-blue-500/30
                       hover:shadow-blue-500/50 hover:scale-105"
          >
            <Plus className="w-5 h-5" />
            <span>Add Download</span>
          </button>

          <div className="h-8 w-px bg-gray-700 mx-1" />

          <button
            onClick={handleOpenDownloadFolder}
            className="p-2.5 text-gray-400 hover:text-white hover:bg-gray-800 
                       rounded-lg transition-colors group"
            title="Open Download Folder"
          >
            <FolderOpen className="w-5 h-5 group-hover:scale-110 transition-transform" />
          </button>

          <button
            onClick={handlePauseAll}
            disabled={activeDownloads.length === 0}
            className="p-2.5 text-gray-400 hover:text-orange-400 hover:bg-gray-800 
                       rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed group"
            title="Pause All Downloads"
          >
            <Pause className="w-5 h-5 group-hover:scale-110 transition-transform" />
          </button>

          <button
            onClick={handleResumeAll}
            disabled={pausedDownloads.length === 0}
            className="p-2.5 text-gray-400 hover:text-green-400 hover:bg-gray-800 
                       rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed group"
            title="Resume All Downloads"
          >
            <Play className="w-5 h-5 group-hover:scale-110 transition-transform" />
          </button>
        </div>
      </div>
    </header>
  );
}

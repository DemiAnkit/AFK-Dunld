// src/components/layout/Header.tsx
import { Plus, FolderOpen, Pause, Play } from "lucide-react";
import { useUIStore } from "../../stores/uiStore";
import { useDownloadStore } from "../../stores/downloadStore";

export function Header() {
  const { setAddDialogOpen } = useUIStore();
  const { downloads } = useDownloadStore();

  const activeDownloads = downloads.filter(
    (d) => d.status === "Downloading" || d.status === "Queued"
  ).length;

  return (
    <header className="bg-gray-900 border-b border-gray-800 px-6 py-4">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold text-white">Downloads</h2>
          <p className="text-sm text-gray-500">
            {activeDownloads > 0
              ? `${activeDownloads} active download${activeDownloads !== 1 ? "s" : ""}`
              : "No active downloads"}
          </p>
        </div>

        <div className="flex items-center gap-3">
          <button
            onClick={() => setAddDialogOpen(true)}
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 
                       text-white rounded-lg transition-colors"
          >
            <Plus className="w-4 h-4" />
            <span>Add Download</span>
          </button>

          <button
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800 
                       rounded-lg transition-colors"
            title="Open Download Folder"
          >
            <FolderOpen className="w-5 h-5" />
          </button>

          <button
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800 
                       rounded-lg transition-colors"
            title="Pause All"
          >
            <Pause className="w-5 h-5" />
          </button>

          <button
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800 
                       rounded-lg transition-colors"
            title="Resume All"
          >
            <Play className="w-5 h-5" />
          </button>
        </div>
      </div>
    </header>
  );
}

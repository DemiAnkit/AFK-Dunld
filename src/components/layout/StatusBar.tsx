// src/components/layout/StatusBar.tsx
import { ArrowDown, ArrowUp } from "lucide-react";
import { useDownloadStore } from "../../stores/downloadStore";
import { formatSpeed } from "../../utils/format";

export function StatusBar() {
  const { downloads } = useDownloadStore();

  // Calculate download speed (active downloads only)
  const activeDownloads = downloads.filter(d => 
    d.status === "downloading" || d.status === "connecting"
  );
  
  const downloadSpeed = activeDownloads.reduce((sum, d) => sum + (d.speed || 0), 0);
  const uploadSpeed = 0; // TODO: Implement upload speed for torrents

  return (
    <footer className="bg-gray-900 border-t border-gray-800 px-4 py-2">
      <div className="flex items-center gap-6 text-sm">
        {/* Download Speed */}
        <div className="flex items-center gap-2">
          <ArrowDown className={`w-4 h-4 ${downloadSpeed > 0 ? 'text-green-400' : 'text-gray-500'}`} />
          <span className="text-gray-400">Download Speed:</span>
          <span className={`font-medium ${downloadSpeed > 0 ? 'text-green-400' : 'text-gray-300'}`}>
            {formatSpeed(downloadSpeed)}
          </span>
        </div>

        {/* Separator */}
        <div className="h-4 w-px bg-gray-700" />

        {/* Upload Speed */}
        <div className="flex items-center gap-2">
          <ArrowUp className={`w-4 h-4 ${uploadSpeed > 0 ? 'text-blue-400' : 'text-gray-500'}`} />
          <span className="text-gray-400">Upload Speed:</span>
          <span className={`font-medium ${uploadSpeed > 0 ? 'text-blue-400' : 'text-gray-300'}`}>
            {formatSpeed(uploadSpeed)}
          </span>
        </div>
      </div>
    </footer>
  );
}

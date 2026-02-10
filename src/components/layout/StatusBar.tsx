// src/components/layout/StatusBar.tsx
import { Wifi, HardDrive, Clock } from "lucide-react";
import { useDownloadStore } from "../../stores/downloadStore";
import { formatBytes, formatSpeed } from "../../utils/format";

export function StatusBar() {
  const { downloads } = useDownloadStore();

  const totalSpeed = downloads.reduce((sum, d) => sum + (d.speed || 0), 0);
  const totalDownloaded = downloads.reduce(
    (sum, d) => sum + (d.downloaded_bytes || 0),
    0
  );

  return (
    <footer className="bg-gray-900 border-t border-gray-800 px-4 py-2">
      <div className="flex items-center justify-between text-sm text-gray-400">
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2">
            <Wifi className="w-4 h-4" />
            <span>{formatSpeed(totalSpeed)}</span>
          </div>

          <div className="flex items-center gap-2">
            <HardDrive className="w-4 h-4" />
            <span>{formatBytes(totalDownloaded)} downloaded</span>
          </div>

          <div className="flex items-center gap-2">
            <Clock className="w-4 h-4" />
            <span>{downloads.length} total downloads</span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <div className="w-2 h-2 rounded-full bg-green-500" />
          <span>Connected</span>
        </div>
      </div>
    </footer>
  );
}

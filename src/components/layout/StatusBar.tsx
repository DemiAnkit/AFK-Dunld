// src/components/layout/StatusBar.tsx
import { ArrowDown, Activity, Download, CheckCircle2, XCircle, Clock } from "lucide-react";
import { useDownloadStore } from "../../stores/downloadStore";
import { formatSpeed, formatBytes } from "../../utils/format";
import { useEffect, useState } from "react";
import { getGlobalStats, type GlobalStats } from "../../services/phase1Api";

export function StatusBar() {
  const { downloads } = useDownloadStore();
  const [stats, setStats] = useState<GlobalStats | null>(null);

  // Fetch global stats periodically
  useEffect(() => {
    const fetchStats = async () => {
      try {
        const globalStats = await getGlobalStats();
        setStats(globalStats);
      } catch (error) {
        console.error('Failed to fetch global stats:', error);
      }
    };

    fetchStats();
    const interval = setInterval(fetchStats, 2000); // Update every 2 seconds

    return () => clearInterval(interval);
  }, []);

  // Calculate download speed (active downloads only)
  const activeDownloads = downloads.filter(d => 
    d.status === "downloading" || d.status === "connecting"
  );
  
  const downloadSpeed = stats?.current_speed || activeDownloads.reduce((sum, d) => sum + (d.speed || 0), 0);
  
  const formatETA = (seconds: number | null) => {
    if (!seconds) return 'N/A';
    if (seconds < 60) return `${Math.round(seconds)}s`;
    if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
    return `${Math.round(seconds / 3600)}h`;
  };

  return (
    <footer className="bg-gray-900 border-t border-gray-800 px-4 py-2">
      <div className="flex items-center gap-4 text-xs">
        {/* Download Speed */}
        <div className="flex items-center gap-2">
          <ArrowDown className={`w-4 h-4 ${downloadSpeed > 0 ? 'text-green-400' : 'text-gray-500'}`} />
          <span className="text-gray-400">Download:</span>
          <span className={`font-medium ${downloadSpeed > 0 ? 'text-green-400' : 'text-gray-300'}`}>
            {formatSpeed(downloadSpeed)}
          </span>
        </div>

        {/* Separator */}
        <div className="h-4 w-px bg-gray-700" />

        {/* Active Downloads */}
        {stats && (
          <>
            <div className="flex items-center gap-2">
              <Activity className="w-4 h-4 text-blue-400" />
              <span className="text-gray-400">Active:</span>
              <span className="font-medium text-blue-400">{stats.active_downloads}</span>
            </div>

            <div className="h-4 w-px bg-gray-700" />

            {/* Completed */}
            <div className="flex items-center gap-2">
              <CheckCircle2 className="w-4 h-4 text-green-400" />
              <span className="text-gray-400">Done:</span>
              <span className="font-medium text-green-400">{stats.completed_downloads}</span>
            </div>

            <div className="h-4 w-px bg-gray-700" />

            {/* Failed */}
            <div className="flex items-center gap-2">
              <XCircle className="w-4 h-4 text-red-400" />
              <span className="text-gray-400">Failed:</span>
              <span className="font-medium text-red-400">{stats.failed_downloads}</span>
            </div>

            <div className="h-4 w-px bg-gray-700" />

            {/* Total Downloaded */}
            <div className="flex items-center gap-2">
              <Download className="w-4 h-4 text-purple-400" />
              <span className="text-gray-400">Downloaded:</span>
              <span className="font-medium text-purple-400">{formatBytes(stats.total_downloaded_bytes)}</span>
            </div>

            {/* ETA if available */}
            {stats.estimated_time_remaining && (
              <>
                <div className="h-4 w-px bg-gray-700" />
                <div className="flex items-center gap-2">
                  <Clock className="w-4 h-4 text-yellow-400" />
                  <span className="text-gray-400">ETA:</span>
                  <span className="font-medium text-yellow-400">{formatETA(stats.estimated_time_remaining)}</span>
                </div>
              </>
            )}
          </>
        )}
      </div>
    </footer>
  );
}

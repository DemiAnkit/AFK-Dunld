// src/components/common/SystemInfoDialog.tsx
import { useEffect, useState } from 'react';
import { X, Monitor, HardDrive, Cpu, MemoryStick, Info } from 'lucide-react';
import { getSystemInfo, type SystemInfo } from '../../services/phase1Api';
import { formatBytes } from '../../utils/format';

interface SystemInfoDialogProps {
  onClose: () => void;
}

export function SystemInfoDialog({ onClose }: SystemInfoDialogProps) {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchSystemInfo = async () => {
      try {
        const info = await getSystemInfo();
        setSystemInfo(info);
      } catch (error) {
        console.error('Failed to fetch system info:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchSystemInfo();
  }, []);

  const diskUsagePercent = systemInfo 
    ? ((systemInfo.total_disk_space - systemInfo.available_disk_space) / systemInfo.total_disk_space) * 100 
    : 0;

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-2xl shadow-2xl w-full max-w-2xl mx-4 border border-gray-700">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-gray-700">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-blue-500/10 rounded-lg">
              <Info className="w-6 h-6 text-blue-400" />
            </div>
            <h2 className="text-2xl font-bold text-white">System Information</h2>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-700 rounded-lg transition-colors"
          >
            <X className="w-5 h-5 text-gray-400" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {loading ? (
            <div className="flex justify-center py-12">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
            </div>
          ) : systemInfo ? (
            <>
              {/* Operating System */}
              <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-700">
                <div className="flex items-center gap-3 mb-3">
                  <Monitor className="w-5 h-5 text-purple-400" />
                  <h3 className="text-lg font-semibold text-white">Operating System</h3>
                </div>
                <div className="grid grid-cols-2 gap-4 ml-8">
                  <div>
                    <p className="text-sm text-gray-400">Platform</p>
                    <p className="text-white font-medium capitalize">{systemInfo.os}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-400">Version</p>
                    <p className="text-white font-medium">{systemInfo.os_version}</p>
                  </div>
                </div>
              </div>

              {/* Disk Space */}
              <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-700">
                <div className="flex items-center gap-3 mb-3">
                  <HardDrive className="w-5 h-5 text-green-400" />
                  <h3 className="text-lg font-semibold text-white">Disk Space (Download Directory)</h3>
                </div>
                <div className="ml-8 space-y-3">
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <p className="text-sm text-gray-400">Total</p>
                      <p className="text-white font-medium">{formatBytes(systemInfo.total_disk_space)}</p>
                    </div>
                    <div>
                      <p className="text-sm text-gray-400">Available</p>
                      <p className="text-green-400 font-medium">{formatBytes(systemInfo.available_disk_space)}</p>
                    </div>
                  </div>
                  {/* Disk usage bar */}
                  <div className="space-y-1">
                    <div className="flex justify-between text-xs text-gray-400">
                      <span>Used {diskUsagePercent.toFixed(1)}%</span>
                      <span>{formatBytes(systemInfo.total_disk_space - systemInfo.available_disk_space)} used</span>
                    </div>
                    <div className="w-full h-2 bg-gray-700 rounded-full overflow-hidden">
                      <div 
                        className={`h-full transition-all duration-300 ${
                          diskUsagePercent > 90 ? 'bg-red-500' : 
                          diskUsagePercent > 75 ? 'bg-yellow-500' : 
                          'bg-green-500'
                        }`}
                        style={{ width: `${diskUsagePercent}%` }}
                      />
                    </div>
                  </div>
                </div>
              </div>

              {/* CPU & Memory */}
              <div className="grid grid-cols-2 gap-4">
                <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-700">
                  <div className="flex items-center gap-3 mb-3">
                    <Cpu className="w-5 h-5 text-blue-400" />
                    <h3 className="text-lg font-semibold text-white">CPU</h3>
                  </div>
                  <div className="ml-8">
                    <p className="text-sm text-gray-400">Cores</p>
                    <p className="text-2xl text-white font-bold">{systemInfo.cpu_count}</p>
                  </div>
                </div>

                <div className="bg-gray-900/50 rounded-xl p-4 border border-gray-700">
                  <div className="flex items-center gap-3 mb-3">
                    <MemoryStick className="w-5 h-5 text-orange-400" />
                    <h3 className="text-lg font-semibold text-white">Memory</h3>
                  </div>
                  <div className="ml-8">
                    <p className="text-sm text-gray-400">Total RAM</p>
                    <p className="text-2xl text-white font-bold">{formatBytes(systemInfo.total_memory)}</p>
                  </div>
                </div>
              </div>
            </>
          ) : (
            <div className="text-center py-12 text-gray-400">
              Failed to load system information
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-end p-6 border-t border-gray-700">
          <button
            onClick={onClose}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg 
                     transition-colors font-medium"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

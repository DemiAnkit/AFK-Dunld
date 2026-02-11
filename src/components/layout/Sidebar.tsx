// src/components/layout/Sidebar.tsx
import { NavLink } from "react-router-dom";
import { 
  Download, 
  CheckCircle, 
  AlertCircle, 
  FolderOpen,
  FileVideo,
  FileAudio,
  FileImage,
  FileText,
  Archive,
  HardDrive,
  Clock
} from "lucide-react";
import { useUIStore } from "../../stores/uiStore";
import { useDownloadStore } from "../../stores/downloadStore";
import { useMemo } from "react";

export function Sidebar() {
  const { sidebarCollapsed } = useUIStore();
  const { downloads } = useDownloadStore();

  // Calculate dynamic counts from actual downloads
  const counts = useMemo(() => {
    const all = downloads.length;
    const downloading = downloads.filter(d => 
      ['downloading', 'connecting', 'queued'].includes(d.status)
    ).length;
    const completed = downloads.filter(d => d.status === 'completed').length;
    const failed = downloads.filter(d => d.status === 'failed').length;
    
    return { all, downloading, completed, failed };
  }, [downloads]);

  const categories = [
    { path: "/", icon: Download, label: "All Downloads", count: counts.all },
    { path: "/downloading", icon: Clock, label: "Downloading", count: counts.downloading },
    { path: "/completed", icon: CheckCircle, label: "Completed", count: counts.completed },
    { path: "/failed", icon: AlertCircle, label: "Failed", count: counts.failed },
  ];

  const fileTypes = [
    { icon: FileVideo, label: "Videos", count: 8 },
    { icon: FileAudio, label: "Audio", count: 5 },
    { icon: FileImage, label: "Images", count: 6 },
    { icon: FileText, label: "Documents", count: 3 },
    { icon: Archive, label: "Archives", count: 2 },
  ];

  const storage = [
    { icon: HardDrive, label: "Storage", count: null },
    { icon: FolderOpen, label: "Folders", count: 5 },
  ];

  return (
    <aside 
      className={`bg-gray-950/95 border-r border-gray-800/80 flex flex-col transition-[width] duration-300 ${
        sidebarCollapsed ? "w-16" : "w-64"
      }`}
    >
      {/* Logo */}
      <div className="px-4 py-3 border-b border-gray-800/80">
        <div className="flex items-center gap-3">
          <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-blue-500 to-indigo-500 shadow-lg shadow-blue-500/30">
            <Download className="w-5 h-5 text-white" />
          </div>
          {!sidebarCollapsed && (
            <div>
              <h1 className="font-semibold text-sm text-white tracking-tight">
                AFK Download
              </h1>
              <p className="text-[11px] text-gray-500 leading-tight">
                Smart download manager
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-2 py-3 overflow-y-auto">
        {/* Downloads Section */}
        {!sidebarCollapsed && (
          <div className="px-3 pb-2 text-[11px] font-medium uppercase tracking-[0.12em] text-gray-500">
            Downloads
          </div>
        )}

        {categories.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              [
                "group flex items-center rounded-md px-3",
                sidebarCollapsed ? "py-2.5 justify-center" : "py-2 gap-3",
                "text-sm font-medium transition-all duration-150",
                "outline-none focus-visible:ring-2 focus-visible:ring-blue-500/70 focus-visible:ring-offset-0",
                isActive
                  ? "bg-blue-600/95 text-white shadow-sm shadow-blue-600/40"
                  : "text-gray-400 hover:text-white hover:bg-gray-800/80"
              ].join(" ")
            }
          >
            <item.icon
              className={`h-5 w-5 flex-shrink-0 transition-transform duration-150 ${
                sidebarCollapsed ? "" : "group-hover:scale-105"
              }`}
            />
            {!sidebarCollapsed && (
              <>
                <span className="flex-1 truncate">{item.label}</span>
                <span className="text-xs text-gray-500 bg-gray-800 px-2 py-0.5 rounded-full">
                  {item.count}
                </span>
              </>
            )}
          </NavLink>
        ))}

        {/* File Types Section */}
        {!sidebarCollapsed && (
          <>
            <div className="px-3 py-2 mt-4 text-[11px] font-medium uppercase tracking-[0.12em] text-gray-500">
              File Types
            </div>
            {fileTypes.map((item, index) => (
              <div
                key={index}
                className="flex items-center rounded-md px-3 py-2 gap-3 text-sm font-medium text-gray-400 hover:text-white hover:bg-gray-800/80 cursor-pointer transition-all duration-150"
              >
                <item.icon className="h-5 w-5 flex-shrink-0" />
                <span className="flex-1 truncate">{item.label}</span>
                <span className="text-xs text-gray-500 bg-gray-800 px-2 py-0.5 rounded-full">
                  {item.count}
                </span>
              </div>
            ))}
          </>
        )}

        {/* Storage Section */}
        {!sidebarCollapsed && (
          <>
            <div className="px-3 py-2 mt-4 text-[11px] font-medium uppercase tracking-[0.12em] text-gray-500">
              Storage
            </div>
            {storage.map((item, index) => (
              <div
                key={index}
                className="flex items-center rounded-md px-3 py-2 gap-3 text-sm font-medium text-gray-400 hover:text-white hover:bg-gray-800/80 cursor-pointer transition-all duration-150"
              >
                <item.icon className="h-5 w-5 flex-shrink-0" />
                <span className="flex-1 truncate">{item.label}</span>
                {item.count !== null && (
                  <span className="text-xs text-gray-500 bg-gray-800 px-2 py-0.5 rounded-full">
                    {item.count}
                  </span>
                )}
              </div>
            ))}
          </>
        )}
      </nav>

      {/* Version */}
      {!sidebarCollapsed && (
        <div className="px-4 py-3 border-t border-gray-800/80 text-[11px] text-gray-500 flex items-center justify-between">
          <span className="uppercase tracking-[0.18em] text-gray-600">
            Version
          </span>
          <span className="font-mono text-gray-300">v0.1.0</span>
        </div>
      )}
    </aside>
  );
}

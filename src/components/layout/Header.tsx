// src/components/layout/Header.tsx
import { Plus, FolderOpen, Pause, Play, Download, Search, List, LayoutGrid, Settings } from "lucide-react";
import { useUIStore } from "../../stores/uiStore";
import { useDownloadStore } from "../../stores/downloadStore";
import { useSettingsStore } from "../../stores/settingsStore";
import { KeyboardShortcutsHelp } from "../common/KeyboardShortcutsHelp";
import { MenuBar } from "../common/MenuBar";
import toast from "react-hot-toast";
import { useState, useEffect } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import { downloadApi } from "../../services/tauriApi";

export function Header() {
  const { setAddDialogOpen, searchQuery, setSearchQuery, viewMode, setViewMode } = useUIStore();
  const { downloads, pauseAll, resumeAll } = useDownloadStore();
  const { settings } = useSettingsStore();
  const [scrolled, setScrolled] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();

  const activeDownloads = downloads.filter(
    (d) => d.status === "downloading" || d.status === "queued" || d.status === "connecting"
  );
  
  const pausedDownloads = downloads.filter(d => d.status === "paused");
  const completedDownloads = downloads.filter(d => d.status === "completed");

  // Track scroll for glassmorphism effect
  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 10);
    };
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

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

  const handleOpenDownloadFolder = async () => {
    try {
      await downloadApi.openDownloadFolder();
      toast.success("Opening download folder...");
    } catch (error: any) {
      console.error("Failed to open folder:", error);
      toast.error(error?.message || "Failed to open download folder");
    }
  };

  return (
    <header className={`sticky top-0 z-40 transition-all duration-300 ${
      scrolled 
        ? 'bg-white/95 dark:bg-gray-950/95 backdrop-blur-xl border-b border-gray-200 dark:border-gray-800/80 shadow-lg' 
        : 'bg-gradient-to-r from-gray-900 via-gray-900 to-gray-800 border-b border-gray-800'
    }`}>
      {/* Menu Bar */}
      <div className="px-4 py-1.5 border-b border-gray-800/50 bg-gray-900/50">
        <MenuBar />
      </div>
      
      <div className="px-6 py-4">
        <div className="flex items-center justify-between gap-6">
          {/* Left Section - Logo & Stats */}
          <div className="flex items-center gap-4 flex-shrink-0">
            {/* App Icon/Logo with glow effect */}
            <div className="relative">
              <div className="absolute inset-0 bg-blue-500/20 blur-xl rounded-full" />
              <div className="relative p-3 bg-gradient-to-br from-blue-600/30 to-purple-600/30 rounded-xl border border-blue-500/30 backdrop-blur-sm">
                <Download className="w-7 h-7 text-blue-400" />
              </div>
            </div>
            
            {/* Title & Stats */}
            <div>
              <h1 className="text-2xl font-bold bg-gradient-to-r from-white to-gray-400 bg-clip-text text-transparent">
                AFK-Dunld
              </h1>
              <div className="flex items-center gap-3 mt-0.5">
                <span className="flex items-center gap-1.5 text-xs text-gray-400">
                  <span className={`w-1.5 h-1.5 rounded-full ${activeDownloads.length > 0 ? 'bg-green-500 animate-pulse' : 'bg-gray-600'}`} />
                  {activeDownloads.length} active
                </span>
                <span className="text-gray-700">|</span>
                <span className="text-xs text-gray-500">
                  {completedDownloads.length} completed
                </span>
                <span className="text-gray-700">|</span>
                <span className="text-xs text-gray-500">
                  {downloads.length} total
                </span>
              </div>
            </div>
          </div>

          {/* Center Section - Search Bar */}
          <div className="flex-1 max-w-2xl">
            <div className="relative group">
              <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                <Search className="h-5 w-5 text-gray-500 group-focus-within:text-blue-400 transition-colors" />
              </div>
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search downloads by name, URL, or status..."
                className="w-full pl-11 pr-4 py-2.5 bg-gray-100 dark:bg-gray-900/80 border border-gray-300 dark:border-gray-700/50 rounded-xl text-gray-900 dark:text-white 
                         text-sm text-white placeholder-gray-500
                         focus:outline-none focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/20
                         transition-all duration-200 hover:border-gray-600"
              />
              {searchQuery && (
                <button
                  onClick={() => setSearchQuery('')}
                  className="absolute inset-y-0 right-0 pr-3 flex items-center"
                >
                  <span className="text-gray-500 hover:text-gray-300 text-xs bg-gray-800 px-2 py-1 rounded">
                    Clear
                  </span>
                </button>
              )}
            </div>
          </div>

          {/* Right Section - Actions */}
          <div className="flex items-center gap-2 flex-shrink-0">
            {/* View Mode Toggle */}
            <div className="flex items-center bg-gray-800/50 rounded-xl p-1 mr-2 border border-gray-700/50">
              <button
                onClick={() => setViewMode('list')}
                className={`p-2 rounded-lg transition-all duration-200 ${
                  viewMode === 'list' 
                    ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/30' 
                    : 'text-gray-400 hover:text-white hover:bg-gray-700/50'
                }`}
                title="List View"
              >
                <List className="w-4 h-4" />
              </button>
              <button
                onClick={() => setViewMode('grid')}
                className={`p-2 rounded-lg transition-all duration-200 ${
                  viewMode === 'grid' 
                    ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/30' 
                    : 'text-gray-400 hover:text-white hover:bg-gray-700/50'
                }`}
                title="Grid View"
              >
                <LayoutGrid className="w-4 h-4" />
              </button>
            </div>

            <div className="h-8 w-px bg-gray-700/50 mx-1" />

            {/* Primary Action Button */}
            <button
              onClick={() => setAddDialogOpen(true)}
              className="flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-blue-600 to-blue-700 
                       hover:from-blue-500 hover:to-blue-600 text-white rounded-xl 
                       transition-all duration-200 font-semibold 
                       shadow-lg shadow-blue-500/25 hover:shadow-blue-500/40 hover:shadow-xl
                       hover:scale-105 active:scale-95 border border-blue-500/30
                       group relative overflow-hidden"
            >
              <div className="absolute inset-0 bg-gradient-to-r from-transparent via-white/10 to-transparent translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-700" />
              <Plus className="w-5 h-5 relative z-10 group-hover:rotate-90 transition-transform duration-300" />
              <span className="relative z-10">Add Download</span>
            </button>

            <div className="h-8 w-px bg-gray-700/50 mx-1" />

            {/* Secondary Actions */}
            <button
              onClick={handleOpenDownloadFolder}
              className="p-2.5 text-gray-400 hover:text-blue-400 hover:bg-blue-500/10 
                       rounded-xl transition-all duration-200 group border border-transparent
                       hover:border-blue-500/30"
              title="Open Download Folder"
            >
              <FolderOpen className="w-5 h-5 group-hover:scale-110 transition-transform" />
            </button>

            <button
              onClick={handlePauseAll}
              disabled={activeDownloads.length === 0}
              className="p-2.5 text-gray-400 hover:text-orange-400 hover:bg-orange-500/10 
                       rounded-xl transition-all duration-200 disabled:opacity-40 
                       disabled:cursor-not-allowed group border border-transparent
                       hover:border-orange-500/30"
              title="Pause All Downloads"
            >
              <Pause className="w-5 h-5 group-hover:scale-110 transition-transform" />
            </button>

            <button
              onClick={handleResumeAll}
              disabled={pausedDownloads.length === 0}
              className="p-2.5 text-gray-400 hover:text-green-400 hover:bg-green-500/10 
                       rounded-xl transition-all duration-200 disabled:opacity-40 
                       disabled:cursor-not-allowed group border border-transparent
                       hover:border-green-500/30"
              title="Resume All Downloads"
            >
              <Play className="w-5 h-5 group-hover:scale-110 transition-transform group-hover:translate-x-0.5" />
            </button>

            <button
              onClick={() => navigate('/settings')}
              className={`p-2 rounded-lg transition-colors ${
                location.pathname === '/settings'
                  ? 'bg-blue-600 hover:bg-blue-700 text-white'
                  : 'bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'
              }`}
              title="Settings"
              aria-label="Open Settings"
            >
              <Settings className="w-5 h-5" />
            </button>
            
            <KeyboardShortcutsHelp />
          </div>
        </div>
      </div>
    </header>
  );
}

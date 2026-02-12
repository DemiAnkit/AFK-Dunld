// src/App.tsx
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";
import { useEffect, useState } from "react";
import { Header } from "./components/layout/Header";
import { TabNavigation } from "./components/layout/TabNavigation";
import { StatusBar } from "./components/layout/StatusBar";
import { DownloadTable } from "./components/downloads/DownloadTable";
import { SettingsPage } from "./components/settings/SettingsPage";
import { AddDownloadDialog } from "./components/downloads/AddDownloadDialog";
import { AddCategoryDialog } from "./components/dialogs/AddCategoryDialog";
import { useDownloadEvents } from "./hooks/useTauriEvents";
import { useTheme } from "./hooks/useTheme";
import { useDownloadStore } from "./stores/downloadStore";
import { useSettingsStore } from "./stores/settingsStore";
import { useUIStore } from "./stores/uiStore";
import { useYouTubeDownload } from "./hooks/useYouTubeDownload";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";
import { AlertCircle, X } from "lucide-react";

const queryClient = new QueryClient();

function AppContent() {
  const { fetchDownloads } = useDownloadStore();
  const { loadSettings } = useSettingsStore();
  const { isAddDialogOpen, setAddDialogOpen, isAddCategoryDialogOpen, setAddCategoryDialogOpen, addCategory } = useUIStore();
  const { checkInstallation, isInstalled } = useYouTubeDownload();
  const [showYtdlpWarning, setShowYtdlpWarning] = useState(false);
  
  useDownloadEvents();
  useTheme();
  useKeyboardShortcuts();

  // Load initial data
  useEffect(() => {
    console.log("App initialized - Loading data...");
    
    const initializeApp = async () => {
      try {
        // Load downloads
        await fetchDownloads();
        console.log("✅ Downloads loaded successfully");
      } catch (err) {
        console.error("❌ Failed to fetch downloads:", err);
      }
      
      try {
        // Load settings
        await loadSettings();
        console.log("✅ Settings loaded successfully");
      } catch (err) {
        console.error("❌ Failed to load settings:", err);
      }
      
      try {
        // Check yt-dlp installation
        const installed = await checkInstallation();
        if (!installed) {
          console.log("⚠️ yt-dlp not installed");
          setShowYtdlpWarning(true);
        } else {
          console.log("✅ yt-dlp is installed");
        }
      } catch (err) {
        console.error("❌ Failed to check yt-dlp installation:", err);
        // Don't show warning if check fails - might be in dev mode
      }
    };
    
    initializeApp();
  }, [fetchDownloads, loadSettings, checkInstallation]);

  return (
    <div className="flex flex-col h-screen bg-gray-950 text-white">
      <Header />
      
      {/* yt-dlp Installation Warning */}
      {showYtdlpWarning && isInstalled === false && (
        <div className="bg-yellow-900/30 border-b border-yellow-700/50 px-4 py-3">
          <div className="flex items-center justify-between max-w-7xl mx-auto">
            <div className="flex items-center gap-3">
              <AlertCircle className="w-5 h-5 text-yellow-400" />
              <div>
                <p className="text-sm font-medium text-yellow-200">
                  yt-dlp is not installed - YouTube downloads will not work
                </p>
                <p className="text-xs text-yellow-300/80 mt-0.5">
                  Install it to enable downloads from YouTube, Vimeo, and 1000+ other platforms.{' '}
                  <a 
                    href="https://github.com/yt-dlp/yt-dlp#installation" 
                    target="_blank" 
                    rel="noopener noreferrer"
                    className="underline hover:text-yellow-200"
                  >
                    Installation Guide
                  </a>
                </p>
              </div>
            </div>
            <button
              onClick={() => setShowYtdlpWarning(false)}
              className="p-1 hover:bg-yellow-800/30 rounded transition-colors"
            >
              <X className="w-4 h-4 text-yellow-300" />
            </button>
          </div>
        </div>
      )}
      
      <TabNavigation />
      <main className="flex-1 overflow-hidden bg-gray-950">
        <Routes>
          <Route path="/" element={<DownloadTable filter="all" />} />
          <Route path="/missing" element={<DownloadTable filter="missing" />} />
          <Route path="/downloading" element={<DownloadTable filter="downloading" />} />
          <Route path="/completed" element={<DownloadTable filter="completed" />} />
          <Route path="/youtube" element={<DownloadTable filter="youtube" />} />
          <Route path="/torrent" element={<DownloadTable filter="torrent" />} />
          <Route path="/video" element={<DownloadTable filter="video" />} />
          <Route path="/music" element={<DownloadTable filter="music" />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </main>
      <StatusBar />
      
      {/* Add Download Dialog */}
      {isAddDialogOpen && (
        <AddDownloadDialog onClose={() => setAddDialogOpen(false)} />
      )}

      {/* Add Category Dialog */}
      {isAddCategoryDialogOpen && (
        <AddCategoryDialog 
          onClose={() => setAddCategoryDialogOpen(false)}
          onAdd={(category) => addCategory(category)}
        />
      )}
      
      <Toaster
        position="bottom-right"
        toastOptions={{
          style: {
            background: "#1f2937",
            color: "#fff",
            border: "1px solid #374151",
          },
        }}
      />
    </div>
  );
}

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <AppContent />
      </BrowserRouter>
    </QueryClientProvider>
  );
}
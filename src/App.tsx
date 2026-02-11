// src/App.tsx
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";
import { useEffect } from "react";
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

const queryClient = new QueryClient();

function AppContent() {
  const { fetchDownloads } = useDownloadStore();
  const { loadSettings } = useSettingsStore();
  const { isAddDialogOpen, setAddDialogOpen, isAddCategoryDialogOpen, setAddCategoryDialogOpen, addCategory } = useUIStore();
  
  useDownloadEvents();
  useTheme();

  // Load initial data
  useEffect(() => {
    console.log("App initialized - Loading data...");
    fetchDownloads().catch(err => console.error("Failed to fetch downloads:", err));
    loadSettings().catch(err => console.error("Failed to load settings:", err));
  }, [fetchDownloads, loadSettings]);

  return (
    <div className="flex flex-col h-screen bg-gray-950 text-white">
      <Header />
      <TabNavigation />
      <main className="flex-1 overflow-hidden bg-gray-950">
        <Routes>
          <Route path="/" element={<DownloadTable filter="all" />} />
          <Route path="/missing" element={<DownloadTable filter="missing" />} />
          <Route path="/downloading" element={<DownloadTable filter="downloading" />} />
          <Route path="/completed" element={<DownloadTable filter="completed" />} />
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
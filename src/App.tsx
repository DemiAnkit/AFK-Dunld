// src/App.tsx
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "react-hot-toast";
import { Sidebar } from "./components/layout/Sidebar";
import { Header } from "./components/layout/Header";
import { DownloadList } from "./components/downloads/DownloadList";
import { SettingsPage } from "./components/settings/SettingsPage";
import { StatusBar } from "./components/layout/StatusBar";
import { useDownloadEvents } from "./hooks/useTauriEvents";
import { useTheme } from "./hooks/useTheme";

const queryClient = new QueryClient();

function AppContent() {
  useDownloadEvents();
  useTheme();

  return (
    <div className="flex h-screen bg-gray-950 text-white">
      <Sidebar />
      <div className="flex flex-col flex-1 overflow-hidden">
        <Header />
        <main className="flex-1 overflow-auto p-4">
          <Routes>
            <Route path="/" element={<DownloadList filter="all" />} />
            <Route path="/downloading" element={<DownloadList filter="downloading" />} />
            <Route path="/completed" element={<DownloadList filter="completed" />} />
            <Route path="/failed" element={<DownloadList filter="failed" />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Routes>
        </main>
        <StatusBar />
      </div>
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
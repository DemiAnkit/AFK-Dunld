// src/hooks/useTauriEvents.tsx
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useDownloadStore } from "../stores/downloadStore";
import type { Download, DownloadProgress } from "../types/download";
import toast from "react-hot-toast";

export function useDownloadEvents() {
  const { updateProgress, updateDownload, fetchDownloads } = useDownloadStore();

  useEffect(() => {
    const listeners: (() => void)[] = [];

    const setup = async () => {
      try {
        // Download added
        const unlisten0 = await listen<Download>(
          "download-added",
          (event) => {
            toast.success(`📥 Added: ${event.payload.fileName}`);
            fetchDownloads(); // Refresh the downloads list
          }
        );
        listeners.push(unlisten0);

        // Download progress updates
        const unlisten1 = await listen<DownloadProgress>(
          "download-progress",
          (event) => {
            updateProgress(event.payload);
          }
        );
        listeners.push(unlisten1);

        // Download completed
        const unlisten2 = await listen<Download>(
          "download-complete",
          (event) => {
            toast.success(`✅ ${event.payload.fileName} completed!`);
            updateDownload(event.payload);
          }
        );
        listeners.push(unlisten2);

        // Download failed
        const unlisten3 = await listen<Download>(
          "download-failed",
          (event) => {
            toast.error(
              `❌ ${event.payload.fileName} failed: ${event.payload.errorMessage || 'Unknown error'}`
            );
            updateDownload(event.payload);
          }
        );
        listeners.push(unlisten3);

        // Download paused
        const unlisten4 = await listen<Download>(
          "download-paused",
          (event) => {
            toast(`⏸️ ${event.payload.fileName} paused`);
            updateDownload(event.payload);
          }
        );
        listeners.push(unlisten4);

        // Download resumed
        const unlisten5 = await listen<Download>(
          "download-resumed",
          (event) => {
            toast(`▶️ ${event.payload.fileName} resumed`);
            updateDownload(event.payload);
          }
        );
        listeners.push(unlisten5);

        // File deleted from disk
        const unlisten6 = await listen<{ id: string; file_name: string; message: string }>(
          "file-deleted",
          (event) => {
            toast(
              `⚠️ "${event.payload.file_name}" was deleted from the download folder`,
              { 
                duration: 5000,
                icon: '⚠️'
              }
            );
            // Refresh downloads to get updated status
            fetchDownloads();
          }
        );
        listeners.push(unlisten6);

        // Clipboard URL detected
        const unlisten7 = await listen<string>(
          "clipboard-url-detected",
          (event) => {
            toast(
              (t) => (
                <div className="flex items-center gap-2">
                  <span>URL detected! Download?</span>
                  <button
                    onClick={() => {
                      useDownloadStore.getState().addDownload(event.payload);
                      toast.dismiss(t.id);
                    }}
                    className="px-2 py-1 bg-blue-500 hover:bg-blue-600 rounded text-sm transition-colors"
                  >
                    Yes
                  </button>
                  <button
                    onClick={() => toast.dismiss(t.id)}
                    className="px-2 py-1 bg-gray-600 hover:bg-gray-700 rounded text-sm transition-colors"
                  >
                    No
                  </button>
                </div>
              ),
              { duration: 5000 }
            );
          }
        );
        listeners.push(unlisten7);

        // Global speed update (optional - for status bar)
        const unlisten8 = await listen<number>(
          "global-speed-update",
          (event) => {
            // You can add this to UI store if needed
            console.log("Global speed:", event.payload);
          }
        );
        listeners.push(unlisten8);

      } catch (error) {
        console.error("Failed to setup event listeners:", error);
      }
    };

    setup();

    return () => {
      listeners.forEach((unlisten) => unlisten());
    };
  }, [updateProgress, updateDownload, fetchDownloads]);
}

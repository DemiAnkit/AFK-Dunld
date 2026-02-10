// src/hooks/useTauriEvents.ts
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useDownloadStore, DownloadProgress, Download } from "../stores/downloadStore";
import toast from "react-hot-toast";

export function useDownloadEvents() {
  const { updateProgress, fetchDownloads } = useDownloadStore();

  useEffect(() => {
    const listeners: (() => void)[] = [];

    const setup = async () => {
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
          toast.success(`✅ ${event.payload.file_name} completed!`);
          fetchDownloads();
        }
      );
      listeners.push(unlisten2);

      // Download failed
      const unlisten3 = await listen<Download>(
        "download-failed",
        (event) => {
          toast.error(
            `❌ ${event.payload.file_name} failed: ${event.payload.error_message}`
          );
          fetchDownloads();
        }
      );
      listeners.push(unlisten3);

      // Clipboard URL detected
      const unlisten4 = await listen<string>(
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
                  className="px-2 py-1 bg-blue-500 rounded text-sm"
                >
                  Yes
                </button>
              </div>
            ),
            { duration: 5000 }
          );
        }
      );
      listeners.push(unlisten4);
    };

    setup();

    return () => {
      listeners.forEach((unlisten) => unlisten());
    };
  }, []);
}
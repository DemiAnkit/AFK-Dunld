import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Settings {
  theme: "light" | "dark" | "system";
  downloadPath: string;
  maxConcurrentDownloads: number;
  maxDownloadSpeed: number;
  autoStartDownloads: boolean;
  showNotifications: boolean;
  minimizeToTray: boolean;
}

const defaultSettings: Settings = {
  theme: "dark",
  downloadPath: "",
  maxConcurrentDownloads: 3,
  maxDownloadSpeed: 0,
  autoStartDownloads: true,
  showNotifications: true,
  minimizeToTray: true,
};

interface SettingsState {
  settings: Settings;
  updateSettings: (settings: Partial<Settings>) => void;
  loadSettings: () => Promise<void>;
}

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      settings: defaultSettings,

      updateSettings: (newSettings: Partial<Settings>) => {
        set((state) => ({
          settings: { ...state.settings, ...newSettings },
        }));
      },

      loadSettings: async () => {
        // TODO: Load from Tauri API
        console.log("Loading settings...");
      },
    }),
    {
      name: "afk-download-settings",
    }
  )
);

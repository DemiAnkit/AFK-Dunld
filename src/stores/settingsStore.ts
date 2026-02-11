import { create } from "zustand";
import { settingsService, type AppSettings } from "../services/settingsService";
import { downloadApi } from "../services/tauriApi";
import toast from "react-hot-toast";

interface SettingsState {
  settings: AppSettings | null;
  isLoading: boolean;
  error: string | null;
  loadSettings: () => Promise<void>;
  updateSettings: (settings: Partial<AppSettings>) => Promise<void>;
  resetSettings: () => Promise<void>;
  setDownloadPath: (path: string) => Promise<void>;
  selectDownloadPath: () => Promise<void>;
  setMaxConcurrent: (max: number) => Promise<void>;
  setSpeedLimit: (limit: number) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  isLoading: false,
  error: null,

  loadSettings: async () => {
    set({ isLoading: true, error: null });
    try {
      const settings = await settingsService.getSettings();
      set({ settings, isLoading: false });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to load settings";
      set({ error: errorMessage, isLoading: false });
      console.error("Failed to load settings:", error);
    }
  },

  updateSettings: async (newSettings: Partial<AppSettings>) => {
    set({ error: null });
    try {
      await settingsService.updateSettings(newSettings);
      
      // Update local state
      set((state) => ({
        settings: state.settings ? { ...state.settings, ...newSettings } : null,
      }));

      // Apply specific settings that need backend updates
      if (newSettings.maxConcurrentDownloads !== undefined) {
        await downloadApi.setMaxConcurrent(newSettings.maxConcurrentDownloads);
      }

      if (newSettings.maxDownloadSpeed !== undefined) {
        await downloadApi.setSpeedLimit(newSettings.maxDownloadSpeed || null);
      }

      toast.success("Settings updated");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to update settings";
      set({ error: errorMessage });
      toast.error(errorMessage);
      throw error;
    }
  },

  resetSettings: async () => {
    try {
      const settings = await settingsService.resetSettings();
      set({ settings });
      toast.success("Settings reset to defaults");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to reset settings";
      set({ error: errorMessage });
      toast.error(errorMessage);
      throw error;
    }
  },

  setDownloadPath: async (path: string) => {
    try {
      await settingsService.setDownloadPath(path);
      set((state) => ({
        settings: state.settings ? { ...state.settings, downloadPath: path } : null,
      }));
      toast.success("Download path updated");
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to set download path";
      toast.error(errorMessage);
      throw error;
    }
  },

  selectDownloadPath: async () => {
    try {
      const path = await settingsService.selectDownloadPath();
      if (path) {
        await get().setDownloadPath(path);
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to select download path";
      toast.error(errorMessage);
      throw error;
    }
  },

  setMaxConcurrent: async (max: number) => {
    try {
      await downloadApi.setMaxConcurrent(max);
      await get().updateSettings({ maxConcurrentDownloads: max });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to set max concurrent downloads";
      toast.error(errorMessage);
      throw error;
    }
  },

  setSpeedLimit: async (limit: number) => {
    try {
      await downloadApi.setSpeedLimit(limit || null);
      await get().updateSettings({ maxDownloadSpeed: limit });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "Failed to set speed limit";
      toast.error(errorMessage);
      throw error;
    }
  },
}));

// Re-export types for convenience
export type { AppSettings as Settings };

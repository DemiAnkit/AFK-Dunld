import { create } from "zustand";

interface UIState {
  isAddDialogOpen: boolean;
  isSettingsOpen: boolean;
  selectedDownloadId: string | null;
  sidebarCollapsed: boolean;
  
  setAddDialogOpen: (open: boolean) => void;
  setSettingsOpen: (open: boolean) => void;
  setSelectedDownload: (id: string | null) => void;
  toggleSidebar: () => void;
}

export const useUIStore = create<UIState>((set) => ({
  isAddDialogOpen: false,
  isSettingsOpen: false,
  selectedDownloadId: null,
  sidebarCollapsed: false,

  setAddDialogOpen: (open: boolean) => set({ isAddDialogOpen: open }),
  setSettingsOpen: (open: boolean) => set({ isSettingsOpen: open }),
  setSelectedDownload: (id: string | null) => set({ selectedDownloadId: id }),
  toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
}));

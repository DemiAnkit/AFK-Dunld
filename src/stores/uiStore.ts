import { create } from "zustand";
import { persist } from "zustand/middleware";

interface UIState {
  isAddDialogOpen: boolean;
  isSettingsOpen: boolean;
  isAddCategoryDialogOpen: boolean;
  selectedDownloadId: string | null;
  sidebarCollapsed: boolean;
  customCategories: string[];
  
  setAddDialogOpen: (open: boolean) => void;
  setSettingsOpen: (open: boolean) => void;
  setAddCategoryDialogOpen: (open: boolean) => void;
  setSelectedDownload: (id: string | null) => void;
  toggleSidebar: () => void;
  addCategory: (category: string) => void;
  removeCategory: (category: string) => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      isAddDialogOpen: false,
      isSettingsOpen: false,
      isAddCategoryDialogOpen: false,
      selectedDownloadId: null,
      sidebarCollapsed: false,
      customCategories: [],

      setAddDialogOpen: (open: boolean) => set({ isAddDialogOpen: open }),
      setSettingsOpen: (open: boolean) => set({ isSettingsOpen: open }),
      setAddCategoryDialogOpen: (open: boolean) => set({ isAddCategoryDialogOpen: open }),
      setSelectedDownload: (id: string | null) => set({ selectedDownloadId: id }),
      toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
      addCategory: (category: string) =>
        set((state) => ({
          customCategories: [...state.customCategories, category],
        })),
      removeCategory: (category: string) =>
        set((state) => ({
          customCategories: state.customCategories.filter((c) => c !== category),
        })),
    }),
    {
      name: "afk-dunld-ui-storage",
    }
  )
);

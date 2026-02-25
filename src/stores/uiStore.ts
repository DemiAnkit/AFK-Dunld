import { create } from "zustand";
import { persist } from "zustand/middleware";

interface UIState {
  isAddDialogOpen: boolean;
  isSettingsOpen: boolean;
  isAddCategoryDialogOpen: boolean;
  isKeyboardShortcutsOpen: boolean;
  selectedDownloadId: string | null;
  sidebarCollapsed: boolean;
  customCategories: string[];
  searchQuery: string;
  viewMode: 'list' | 'grid';
  selectedDownloads: Set<string>;
  lastSelectedId: string | null;
  fontSize: 'small' | 'medium' | 'large';
  toolbarSize: 'small' | 'large';
  
  setAddDialogOpen: (open: boolean) => void;
  setSettingsOpen: (open: boolean) => void;
  setAddCategoryDialogOpen: (open: boolean) => void;
  setKeyboardShortcutsOpen: (open: boolean) => void;
  setSelectedDownload: (id: string | null) => void;
  toggleSidebar: () => void;
  addCategory: (category: string) => void;
  removeCategory: (category: string) => void;
  setSearchQuery: (query: string) => void;
  setViewMode: (mode: 'list' | 'grid') => void;
  setFontSize: (size: 'small' | 'medium' | 'large') => void;
  setToolbarSize: (size: 'small' | 'large') => void;
  toggleSelection: (id: string, isShiftKey?: boolean, isCtrlKey?: boolean, allIds?: string[]) => void;
  selectAll: (ids: string[]) => void;
  clearSelection: () => void;
  isSelected: (id: string) => boolean;
}

// Helper to ensure selectedDownloads is always a Set
const ensureSet = (value: any): Set<string> => {
  if (value instanceof Set) return value;
  if (Array.isArray(value)) return new Set(value);
  return new Set();
};

export const useUIStore = create<UIState>()(
  persist(
    (set, get) => ({
      isAddDialogOpen: false,
      isSettingsOpen: false,
      isAddCategoryDialogOpen: false,
      isKeyboardShortcutsOpen: false,
      selectedDownloadId: null,
      sidebarCollapsed: false,
      customCategories: [],
      searchQuery: '',
      viewMode: 'list',
      selectedDownloads: new Set<string>(),
      lastSelectedId: null,
      fontSize: 'medium',
      toolbarSize: 'large',

      setAddDialogOpen: (open: boolean) => set({ isAddDialogOpen: open }),
      setSettingsOpen: (open: boolean) => set({ isSettingsOpen: open }),
      setAddCategoryDialogOpen: (open: boolean) => set({ isAddCategoryDialogOpen: open }),
      setKeyboardShortcutsOpen: (open: boolean) => set({ isKeyboardShortcutsOpen: open }),
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
      setSearchQuery: (query: string) => set({ searchQuery: query }),
      setViewMode: (mode: 'list' | 'grid') => set({ viewMode: mode }),
      setFontSize: (size: 'small' | 'medium' | 'large') => set({ fontSize: size }),
      setToolbarSize: (size: 'small' | 'large') => set({ toolbarSize: size }),
      
      toggleSelection: (id: string, isShiftKey = false, isCtrlKey = false, allIds = []) => {
        const { selectedDownloads, lastSelectedId } = get();
        const newSelection = ensureSet(selectedDownloads);
        
        if (isShiftKey && lastSelectedId && allIds.length > 0) {
          // Range selection with Shift
          const lastIndex = allIds.indexOf(lastSelectedId);
          const currentIndex = allIds.indexOf(id);
          
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            
            for (let i = start; i <= end; i++) {
              newSelection.add(allIds[i]);
            }
          }
        } else if (isCtrlKey) {
          // Multi-selection with Ctrl (toggle individual)
          if (newSelection.has(id)) {
            newSelection.delete(id);
          } else {
            newSelection.add(id);
          }
        } else {
          // Single selection (clear others)
          if (newSelection.has(id) && newSelection.size === 1) {
            newSelection.clear();
          } else {
            newSelection.clear();
            newSelection.add(id);
          }
        }
        
        set({ 
          selectedDownloads: newSelection,
          lastSelectedId: id
        });
      },
      
      selectAll: (ids: string[]) => {
        set({ selectedDownloads: new Set(ids) });
      },
      
      clearSelection: () => {
        set({ selectedDownloads: new Set(), lastSelectedId: null });
      },
      
      isSelected: (id: string) => {
        const selectedDownloads = ensureSet(get().selectedDownloads);
        return selectedDownloads.has(id);
      },
    }),
    {
      name: "afk-dunld-ui-storage",
      partialize: (state) => ({
        ...state,
        // Don't persist selection state - it should be reset on app reload
        selectedDownloads: undefined,
        lastSelectedId: undefined,
      }),
    }
  )
);

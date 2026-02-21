import { useState, useRef, useEffect } from "react";
import { FileText, Eye, HelpCircle, FolderOpen, Settings, RefreshCw, X, ExternalLink, Info, Download } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useUIStore } from "../../stores/uiStore";
import { useDownloadStore } from "../../stores/downloadStore";
import { downloadApi } from "../../services/tauriApi";
import toast from "react-hot-toast";

interface MenuItem {
  label: string;
  icon?: React.ReactNode;
  shortcut?: string;
  action?: () => void;
  divider?: boolean;
  disabled?: boolean;
}

interface Menu {
  label: string;
  icon: React.ReactNode;
  items: MenuItem[];
}

export function MenuBar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const menuRef = useRef<HTMLDivElement>(null);
  const navigate = useNavigate();
  const { setAddDialogOpen } = useUIStore();
  const { downloads, pauseAll, resumeAll } = useDownloadStore();

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setOpenMenu(null);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleOpenDownloadFolder = async () => {
    try {
      await downloadApi.openDownloadFolder();
      toast.success("Opening download folder...");
    } catch (error: any) {
      console.error("Failed to open folder:", error);
      toast.error(error?.message || "Failed to open download folder");
    }
    setOpenMenu(null);
  };

  const handleRefreshDownloads = async () => {
    try {
      await useDownloadStore.getState().fetchDownloads();
      toast.success("Downloads refreshed");
    } catch (error) {
      console.error("Failed to refresh downloads:", error);
    }
    setOpenMenu(null);
  };

  const activeDownloads = downloads.filter(
    (d) => d.status === "downloading" || d.status === "queued" || d.status === "connecting"
  );
  const pausedDownloads = downloads.filter((d) => d.status === "paused");

  const menus: Menu[] = [
    {
      label: "File",
      icon: <FileText size={16} />,
      items: [
        {
          label: "New Download",
          icon: <Download size={16} />,
          shortcut: "Ctrl+N",
          action: () => {
            setAddDialogOpen(true);
            setOpenMenu(null);
          },
        },
        {
          label: "Open Download Folder",
          icon: <FolderOpen size={16} />,
          action: () => {
            handleOpenDownloadFolder();
          },
        },
        { label: "", divider: true },
        {
          label: "Settings",
          icon: <Settings size={16} />,
          shortcut: "Ctrl+,",
          action: () => {
            navigate("/settings");
            setOpenMenu(null);
          },
        },
      ],
    },
    {
      label: "View",
      icon: <Eye size={16} />,
      items: [
        {
          label: "Refresh Downloads",
          icon: <RefreshCw size={16} />,
          shortcut: "F5",
          action: () => {
            handleRefreshDownloads();
          },
        },
        { label: "", divider: true },
        {
          label: "All Downloads",
          action: () => {
            navigate("/");
            setOpenMenu(null);
          },
        },
        {
          label: "Downloading",
          action: () => {
            navigate("/downloading");
            setOpenMenu(null);
          },
        },
        {
          label: "Completed",
          action: () => {
            navigate("/completed");
            setOpenMenu(null);
          },
        },
        {
          label: "History",
          action: () => {
            navigate("/history");
            setOpenMenu(null);
          },
        },
        {
          label: "Queue",
          action: () => {
            navigate("/queue");
            setOpenMenu(null);
          },
        },
      ],
    },
    {
      label: "Help",
      icon: <HelpCircle size={16} />,
      items: [
        {
          label: "Keyboard Shortcuts",
          icon: <ExternalLink size={16} />,
          action: () => {
            setOpenMenu(null);
          },
        },
        {
          label: "About AFK-Dunld",
          icon: <Info size={16} />,
          action: () => {
            setOpenMenu(null);
          },
        },
      ],
    },
  ];

  const handleMenuClick = (menuLabel: string) => {
    setOpenMenu(openMenu === menuLabel ? null : menuLabel);
  };

  return (
    <div ref={menuRef} className="flex items-center gap-1">
      {menus.map((menu) => (
        <div key={menu.label} className="relative">
          <button
            onClick={() => handleMenuClick(menu.label)}
            className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-150 ${
              openMenu === menu.label
                ? "bg-gray-700 text-white"
                : "text-gray-300 hover:bg-gray-800 hover:text-white"
            }`}
          >
            {menu.icon}
            {menu.label}
          </button>

          {openMenu === menu.label && (
            <div className="absolute top-full left-0 mt-1 min-w-48 bg-gray-900 border border-gray-700 rounded-xl shadow-xl py-1 z-50 overflow-hidden">
              {menu.items.map((item, index) =>
                item.divider ? (
                  <div key={index} className="my-1 border-t border-gray-700" />
                ) : (
                  <button
                    key={index}
                    onClick={() => {
                      item.action?.();
                      setOpenMenu(null);
                    }}
                    disabled={item.disabled}
                    className={`w-full flex items-center justify-between px-4 py-2.5 text-sm transition-colors ${
                      item.disabled
                        ? "text-gray-600 cursor-not-allowed"
                        : "text-gray-300 hover:bg-gray-800 hover:text-white"
                    }`}
                  >
                    <div className="flex items-center gap-3">
                      {item.icon}
                      {item.label}
                    </div>
                    {item.shortcut && (
                      <kbd className="ml-4 px-2 py-0.5 text-xs bg-gray-800 text-gray-500 rounded border border-gray-700">
                        {item.shortcut}
                      </kbd>
                    )}
                  </button>
                )
              )}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

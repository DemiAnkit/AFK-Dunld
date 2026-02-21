import { useState, useRef, useEffect } from "react";
import { FileText, Eye, HelpCircle, FolderOpen, Settings, RefreshCw, ExternalLink, Info, Download, Sun, Moon, Type, Square, Check } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useUIStore } from "../../stores/uiStore";
import { useDownloadStore } from "../../stores/downloadStore";
import { downloadApi } from "../../services/tauriApi";
import { useTheme } from "../../hooks/useTheme";
import toast from "react-hot-toast";

interface MenuItem {
  label: string;
  icon?: React.ReactNode;
  shortcut?: string;
  action?: () => void;
  divider?: boolean;
  disabled?: boolean;
  checked?: boolean;
  submenu?: MenuItem[];
}

interface Menu {
  label: string;
  icon: React.ReactNode;
  items: MenuItem[];
}

export function MenuBar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const [openSubmenu, setOpenSubmenu] = useState<string | null>(null);
  const menuRef = useRef<HTMLDivElement>(null);
  const navigate = useNavigate();
  const { setAddDialogOpen, fontSize, toolbarSize, setFontSize, setToolbarSize } = useUIStore();
  const { downloads } = useDownloadStore();
  const { theme, setTheme } = useTheme();

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setOpenMenu(null);
        setOpenSubmenu(null);
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

  const handleThemeToggle = () => {
    setTheme(theme === 'dark' ? 'light' : 'dark');
    setOpenMenu(null);
  };

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
        { label: "", divider: true },
        {
          label: "Theme",
          icon: theme === 'dark' ? <Moon size={16} /> : <Sun size={16} />,
          submenu: [
            {
              label: "Light Mode",
              icon: <Sun size={14} />,
              checked: theme === 'light',
              action: () => {
                setTheme('light');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
            {
              label: "Dark Mode",
              icon: <Moon size={14} />,
              checked: theme === 'dark',
              action: () => {
                setTheme('dark');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
          ],
        },
        {
          label: "Font Size",
          icon: <Type size={16} />,
          submenu: [
            {
              label: "Small",
              icon: <span className="text-xs w-4">A</span>,
              checked: fontSize === 'small',
              action: () => {
                setFontSize('small');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
            {
              label: "Medium",
              icon: <span className="text-sm w-4">A</span>,
              checked: fontSize === 'medium',
              action: () => {
                setFontSize('medium');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
            {
              label: "Large",
              icon: <span className="text-base w-4">A</span>,
              checked: fontSize === 'large',
              action: () => {
                setFontSize('large');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
          ],
        },
        {
          label: "Toolbar Size",
          icon: <Square size={16} />,
          submenu: [
            {
              label: "Small Buttons",
              icon: <Square size={14} />,
              checked: toolbarSize === 'small',
              action: () => {
                setToolbarSize('small');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
            {
              label: "Large Buttons",
              icon: <Square size={14} />,
              checked: toolbarSize === 'large',
              action: () => {
                setToolbarSize('large');
                setOpenSubmenu(null);
                setOpenMenu(null);
              },
            },
          ],
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

  const handleSubmenuClick = (submenuLabel: string) => {
    setOpenSubmenu(openSubmenu === submenuLabel ? null : submenuLabel);
  };

  const renderMenuItem = (item: MenuItem, index: number, menuLabel: string) => {
    if (item.divider) {
      return <div key={index} className="my-1 border-t border-gray-700" />;
    }

    const hasSubmenu = item.submenu && item.submenu.length > 0;
    const submenuId = `${menuLabel}-${item.label}`;
    const isSubmenuOpen = openSubmenu === submenuId;

    return (
      <div key={index} className="relative">
        <button
          onClick={() => {
            if (hasSubmenu) {
              handleSubmenuClick(submenuId);
            } else {
              item.action?.();
              setOpenMenu(null);
            }
          }}
          disabled={item.disabled}
          className={`w-full flex items-center justify-between px-4 py-2.5 text-sm transition-colors ${
            item.disabled
              ? "text-gray-600 cursor-not-allowed"
              : "text-gray-300 hover:bg-gray-800 hover:text-white"
          }`}
        >
          <div className="flex items-center gap-3">
            {item.checked !== undefined && (
              <span className="w-4">
                {item.checked && <Check size={14} className="text-blue-400" />}
              </span>
            )}
            {item.icon}
            {item.label}
          </div>
          <div className="flex items-center gap-2">
            {item.shortcut && (
              <kbd className="ml-4 px-2 py-0.5 text-xs bg-gray-800 text-gray-500 rounded border border-gray-700">
                {item.shortcut}
              </kbd>
            )}
            {hasSubmenu && <span className="text-gray-500">▶</span>}
          </div>
        </button>

        {hasSubmenu && isSubmenuOpen && (
          <div className="absolute top-0 left-full ml-1 min-w-40 bg-gray-900 border border-gray-700 rounded-xl shadow-xl py-1 z-50 overflow-hidden">
            {item.submenu!.map((subItem, subIndex) => (
              <button
                key={subIndex}
                onClick={() => {
                  subItem.action?.();
                  setOpenSubmenu(null);
                  setOpenMenu(null);
                }}
                className={`w-full flex items-center gap-3 px-4 py-2 text-sm transition-colors ${
                  subItem.disabled
                    ? "text-gray-600 cursor-not-allowed"
                    : "text-gray-300 hover:bg-gray-800 hover:text-white"
                }`}
              >
                {subItem.checked !== undefined && (
                  <span className="w-4">
                    {subItem.checked && <Check size={14} className="text-blue-400" />}
                  </span>
                )}
                {subItem.icon}
                {subItem.label}
              </button>
            ))}
          </div>
        )}
      </div>
    );
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
              {menu.items.map((item, index) => renderMenuItem(item, index, menu.label))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

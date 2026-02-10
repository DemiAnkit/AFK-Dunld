// src/components/layout/Sidebar.tsx
import { NavLink } from "react-router-dom";
import { 
  Download, 
  CheckCircle, 
  AlertCircle, 
  Settings,
  Activity 
} from "lucide-react";
import { useUIStore } from "../../stores/uiStore";

export function Sidebar() {
  const { sidebarCollapsed } = useUIStore();

  const navItems = [
    { path: "/", icon: Download, label: "All Downloads" },
    { path: "/downloading", icon: Activity, label: "Downloading" },
    { path: "/completed", icon: CheckCircle, label: "Completed" },
    { path: "/failed", icon: AlertCircle, label: "Failed" },
    { path: "/settings", icon: Settings, label: "Settings" },
  ];

  return (
    <aside 
      className={`bg-gray-900 border-r border-gray-800 flex flex-col transition-all duration-300 ${
        sidebarCollapsed ? "w-16" : "w-64"
      }`}
    >
      {/* Logo */}
      <div className="p-4 border-b border-gray-800">
        <div className="flex items-center gap-3">
          <Download className="w-8 h-8 text-blue-500" />
          {!sidebarCollapsed && (
            <div>
              <h1 className="font-bold text-lg text-white">AFK Download</h1>
              <p className="text-xs text-gray-500">Manager</p>
            </div>
          )}
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-2">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2 rounded-lg mb-1 transition-colors ${
                isActive
                  ? "bg-blue-600 text-white"
                  : "text-gray-400 hover:bg-gray-800 hover:text-white"
              }`
            }
          >
            <item.icon className="w-5 h-5 flex-shrink-0" />
            {!sidebarCollapsed && <span>{item.label}</span>}
          </NavLink>
        ))}
      </nav>

      {/* Version */}
      {!sidebarCollapsed && (
        <div className="p-4 border-t border-gray-800 text-xs text-gray-500">
          v0.1.0
        </div>
      )}
    </aside>
  );
}

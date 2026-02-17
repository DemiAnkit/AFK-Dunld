// src/components/layout/TabNavigation.tsx
import { NavLink, useLocation } from 'react-router-dom';
import { 
  Download, 
  CheckCircle, 
  AlertCircle, 
  Play,
  Film,
  Music,
  Plus,
  Folder,
  Youtube,
  History
} from "lucide-react";
import { useUIStore } from "../../stores/uiStore";

export function TabNavigation() {
  const { customCategories, setAddCategoryDialogOpen } = useUIStore();
  
  const handleAddCategory = () => {
    setAddCategoryDialogOpen(true);
  };
  
  const tabs = [
    { path: "/", icon: Download, label: "All" },
    { path: "/missing", icon: AlertCircle, label: "Missing Files" },
    { path: "/downloading", icon: Play, label: "Active" },
    { path: "/completed", icon: CheckCircle, label: "Completed" },
    { path: "/youtube", icon: Youtube, label: "YouTube" },
    { path: "/torrent", icon: Download, label: "Torrent" },
    { path: "/video", icon: Film, label: "Video" },
    { path: "/music", icon: Music, label: "Music" },
    { path: "/history", icon: History, label: "History" },
  ];

  return (
    <div className="bg-gray-900 border-b border-gray-800">
      <div className="flex items-center px-4 overflow-x-auto scrollbar-thin">
        {tabs.map((tab) => (
          <NavLink
            key={tab.path}
            to={tab.path}
            className={({ isActive }) =>
              `flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors flex-shrink-0 ${
                isActive
                  ? "text-blue-400 border-blue-400"
                  : "text-gray-400 border-transparent hover:text-white hover:border-gray-700"
              }`
            }
          >
            <tab.icon className="w-4 h-4" />
            <span>{tab.label}</span>
          </NavLink>
        ))}
        
        {/* Custom Categories */}
        {customCategories.map((category) => (
          <NavLink
            key={category}
            to={`/category/${category.toLowerCase()}`}
            className={({ isActive }) =>
              `flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors flex-shrink-0 ${
                isActive
                  ? "text-blue-400 border-blue-400"
                  : "text-gray-400 border-transparent hover:text-white hover:border-gray-700"
              }`
            }
          >
            <Folder className="w-4 h-4" />
            <span>{category}</span>
          </NavLink>
        ))}
        
        <button 
          onClick={handleAddCategory}
          className="flex items-center gap-2 px-4 py-3 text-sm font-medium text-gray-400 border-b-2 border-transparent hover:text-white hover:border-gray-700 ml-auto transition-colors flex-shrink-0"
        >
          <Plus className="w-4 h-4" />
          <span>Add Category</span>
        </button>
      </div>
    </div>
  );
}

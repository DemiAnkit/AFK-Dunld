import { Plus, FolderOpen, Settings, MoreVertical, Search, Download, Pause, Play, Trash2 } from 'lucide-react';
import { useState } from 'react';
import { AddDownloadDialog } from '../downloads/AddDownloadDialog';

export const Toolbar = () => {
  const [showAddDialog, setShowAddDialog] = useState(false);

  return (
    <>
      <div className="h-14 bg-gray-900 border-b border-gray-800 flex items-center px-4 gap-3">
        {/* Left side - File operations */}
        <div className="flex items-center gap-2">
          <button
            onClick={() => setShowAddDialog(true)}
            className="flex items-center gap-2 px-3 py-2 bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
          >
            <Plus size={16} />
            <span className="text-sm font-medium">Add URL</span>
          </button>
          
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <FolderOpen size={16} />
          </button>
          
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <Download size={16} />
          </button>
          
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <Pause size={16} />
          </button>
          
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <Play size={16} />
          </button>
          
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <Trash2 size={16} />
          </button>
        </div>

        <div className="w-px h-6 bg-gray-700 mx-2" />

        {/* Search */}
        <div className="flex-1 max-w-md">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={16} />
            <input
              type="text"
              placeholder="Search downloads..."
              className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-md text-sm focus:outline-none focus:border-blue-500"
            />
          </div>
        </div>

        {/* Right side - Options */}
        <div className="flex items-center gap-2">
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <MoreVertical size={16} />
          </button>
          
          <button className="p-2 hover:bg-gray-800 rounded-md transition-colors">
            <Settings size={16} />
          </button>
        </div>
      </div>

      {showAddDialog && (
        <AddDownloadDialog onClose={() => setShowAddDialog(false)} />
      )}
    </>
  );
};
import { Plus, FolderOpen, Settings, MoreVertical, Search, Download, Pause, Play, StopCircle } from 'lucide-react';
import { useState } from 'react';
import { AddDownloadDialog } from '../downloads/AddDownloadDialog';
import { pauseAll, resumeAll, cancelAll } from '../../services/phase1Api';
import { toast } from 'react-hot-toast';

export const Toolbar = () => {
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);

  const handlePauseAll = async () => {
    try {
      setIsProcessing(true);
      const pausedIds = await pauseAll();
      toast.success(`Paused ${pausedIds.length} download(s)`);
    } catch (error) {
      toast.error(`Failed to pause downloads: ${error}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleResumeAll = async () => {
    try {
      setIsProcessing(true);
      const resumedIds = await resumeAll();
      toast.success(`Resumed ${resumedIds.length} download(s)`);
    } catch (error) {
      toast.error(`Failed to resume downloads: ${error}`);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleCancelAll = async () => {
    if (!confirm('Are you sure you want to cancel all downloads?')) return;
    
    try {
      setIsProcessing(true);
      const cancelledIds = await cancelAll();
      toast.success(`Cancelled ${cancelledIds.length} download(s)`);
    } catch (error) {
      toast.error(`Failed to cancel downloads: ${error}`);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <>
      <div className="h-14 bg-gray-900/80 backdrop-blur-sm border-b border-gray-800 flex items-center px-4 gap-3">
        {/* Left side - File operations */}
        <div className="flex items-center gap-2">
          <button
            onClick={() => setShowAddDialog(true)}
            className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-600 to-blue-700 
                     hover:from-blue-500 hover:to-blue-600 text-white rounded-xl 
                     transition-all duration-200 font-semibold shadow-lg shadow-blue-500/25 
                     hover:shadow-blue-500/40 hover:shadow-xl hover:scale-105 active:scale-95 
                     border border-blue-500/30 group"
          >
            <Plus size={16} className="group-hover:rotate-90 transition-transform duration-300" />
            <span className="text-sm">Add URL</span>
          </button>
          
          <button className="p-2 text-gray-400 hover:text-blue-400 hover:bg-blue-500/10 
                           rounded-xl transition-all duration-200 border border-transparent
                           hover:border-blue-500/30 hover:scale-110 active:scale-95"
                  title="Open Folder">
            <FolderOpen size={18} />
          </button>
          
          <button className="p-2 text-gray-400 hover:text-green-400 hover:bg-green-500/10 
                           rounded-xl transition-all duration-200 border border-transparent
                           hover:border-green-500/30 hover:scale-110 active:scale-95"
                  title="Download">
            <Download size={18} />
          </button>
          
          <button 
            onClick={handlePauseAll}
            disabled={isProcessing}
            className="p-2 text-gray-400 hover:text-orange-400 hover:bg-orange-500/10 
                     rounded-xl transition-all duration-200 border border-transparent
                     hover:border-orange-500/30 hover:scale-110 active:scale-95
                     disabled:opacity-50 disabled:cursor-not-allowed"
            title="Pause All Downloads">
            <Pause size={18} />
          </button>
          
          <button 
            onClick={handleResumeAll}
            disabled={isProcessing}
            className="p-2 text-gray-400 hover:text-green-400 hover:bg-green-500/10 
                     rounded-xl transition-all duration-200 border border-transparent
                     hover:border-green-500/30 hover:scale-110 active:scale-95
                     disabled:opacity-50 disabled:cursor-not-allowed"
            title="Resume All Downloads">
            <Play size={18} />
          </button>
          
          <button 
            onClick={handleCancelAll}
            disabled={isProcessing}
            className="p-2 text-gray-400 hover:text-red-400 hover:bg-red-500/10 
                     rounded-xl transition-all duration-200 border border-transparent
                     hover:border-red-500/30 hover:scale-110 active:scale-95
                     disabled:opacity-50 disabled:cursor-not-allowed"
            title="Cancel All Downloads">
            <StopCircle size={18} />
          </button>
        </div>

        <div className="w-px h-6 bg-gray-700/50 mx-2" />

        {/* Search */}
        <div className="flex-1 max-w-md">
          <div className="relative group">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-500 group-focus-within:text-blue-400 transition-colors" size={16} />
            <input
              type="text"
              placeholder="Search downloads..."
              className="w-full pl-10 pr-4 py-2 bg-gray-800/80 border border-gray-700/50 rounded-xl text-sm 
                       text-white placeholder-gray-500 focus:outline-none focus:border-blue-500/50 
                       focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 hover:border-gray-600"
            />
          </div>
        </div>

        {/* Right side - Options */}
        <div className="flex items-center gap-2">
          <button className="p-2 text-gray-400 hover:text-white hover:bg-gray-800/80 
                           rounded-xl transition-all duration-200 border border-transparent
                           hover:border-gray-500/30 hover:scale-110 active:scale-95"
                  title="More Options">
            <MoreVertical size={18} />
          </button>
          
          <button className="p-2 text-gray-400 hover:text-blue-400 hover:bg-blue-500/10 
                           rounded-xl transition-all duration-200 border border-transparent
                           hover:border-blue-500/30 hover:scale-110 active:scale-95"
                  title="Settings">
            <Settings size={18} className="hover:rotate-90 transition-transform duration-300" />
          </button>
        </div>
      </div>

      {showAddDialog && (
        <AddDownloadDialog onClose={() => setShowAddDialog(false)} />
      )}
    </>
  );
};
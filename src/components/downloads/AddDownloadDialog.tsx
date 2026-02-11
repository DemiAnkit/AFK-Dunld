import { useState, useEffect, useRef } from "react";
import { X, Plus, Link, FolderOpen, Download, Youtube } from "lucide-react";
import { useDownloadStore } from "../../stores/downloadStore";
import { open } from "@tauri-apps/plugin-dialog";
import toast from "react-hot-toast";

interface AddDownloadDialogProps {
  onClose: () => void;
}

export function AddDownloadDialog({ onClose }: AddDownloadDialogProps) {
  const [url, setUrl] = useState("");
  const [savePath, setSavePath] = useState("");
  const [fileName, setFileName] = useState("");
  const [loading, setLoading] = useState(false);
  const [isYouTube, setIsYouTube] = useState(false);
  const [youtubeQuality, setYoutubeQuality] = useState("best");
  const [youtubeFormat, setYoutubeFormat] = useState<"video" | "audio">("video");
  
  const { addDownload } = useDownloadStore();
  
  // Draggable dialog state
  const [isDragging, setIsDragging] = useState(false);
  const [position, setPosition] = useState({ x: 0, y: 0 });
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const dialogRef = useRef<HTMLDivElement>(null);

  // Detect YouTube URLs
  useEffect(() => {
    const youtubeRegex = /(?:youtube\.com\/(?:[^\/]+\/.+\/|(?:v|e(?:mbed)?)\/|.*[?&]v=)|youtu\.be\/)([^"&?\/\s]{11})/;
    setIsYouTube(youtubeRegex.test(url));
  }, [url]);

  // Draggable handlers
  const handleMouseDown = (e: React.MouseEvent) => {
    if ((e.target as HTMLElement).tagName === 'INPUT' || 
        (e.target as HTMLElement).tagName === 'BUTTON') {
      return;
    }
    setIsDragging(true);
    setDragStart({
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    });
  };

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;
      
      setPosition({
        x: e.clientX - dragStart.x,
        y: e.clientY - dragStart.y,
      });
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, dragStart]);

  const handleAddDownload = async () => {
    if (!url.trim()) {
      toast.error("Please enter a URL");
      return;
    }
    
    setLoading(true);
    try {
      const downloadOptions: any = {
        savePath: savePath || undefined,
        fileName: fileName || undefined,
      };

      if (isYouTube) {
        downloadOptions.category = "youtube";
        downloadOptions.youtubeFormat = youtubeFormat; // "video" or "audio"
        downloadOptions.youtubeQuality = youtubeFormat === "video" ? youtubeQuality : "best";
        
        toast.success(`YouTube ${youtubeFormat === "audio" ? "Audio" : "Video"} download started!`);
      }
      
      await addDownload(url.trim(), downloadOptions);
      
      setUrl("");
      setSavePath("");
      setFileName("");
      onClose();
      toast.success("Download added successfully!");
    } catch (error) {
      console.error("Failed to add download:", error);
      toast.error("Failed to add download");
    } finally {
      setLoading(false);
    }
  };

  const handlePaste = async () => {
    try {
      const text = await navigator.clipboard.readText();
      setUrl(text.trim());
    } catch (error) {
      console.error("Failed to read clipboard:", error);
      toast.error("Failed to paste from clipboard");
    }
  };

  const handleSelectFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Download Location",
      });
      
      if (selected && typeof selected === 'string') {
        setSavePath(selected);
      }
    } catch (error) {
      console.error("Failed to select folder:", error);
      toast.error("Failed to select folder");
    }
  };

  return (
    <div 
      className="fixed inset-0 flex items-center justify-center z-50 p-4"
      style={{ backgroundColor: 'rgba(0, 0, 0, 0.85)' }}
      onClick={onClose}
    >
      <div 
        ref={dialogRef}
        className="bg-gradient-to-br from-gray-900 via-gray-900 to-gray-800 rounded-xl shadow-2xl w-full max-w-2xl border border-gray-700"
        style={{ 
          transform: `translate(${position.x}px, ${position.y}px)`,
          cursor: isDragging ? 'grabbing' : 'grab',
          transition: isDragging ? 'none' : 'transform 0.1s ease-out',
        }}
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header - Draggable */}
        <div 
          className="flex items-center justify-between p-6 border-b border-gray-700 bg-gradient-to-r from-blue-900/20 to-purple-900/20"
          onMouseDown={handleMouseDown}
          style={{ cursor: isDragging ? 'grabbing' : 'grab' }}
        >
          <div className="flex items-center gap-3">
            <div className="p-2 bg-blue-600/30 rounded-lg">
              <Download className="w-6 h-6 text-blue-400" />
            </div>
            <div>
              <h2 className="text-xl font-semibold text-white">Add New Download</h2>
              <p className="text-sm text-gray-400">Drag to move • Enter URL or paste from clipboard</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-800 rounded-lg transition-colors group"
            onMouseDown={(e) => e.stopPropagation()}
          >
            <X size={20} className="text-gray-400 group-hover:text-white" />
          </button>
        </div>
        
        {/* Body */}
        <div className="p-6 space-y-5">
          {/* URL Input */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-gray-300 flex items-center gap-2">
              <Link size={16} />
              Download URL {isYouTube && <span className="flex items-center gap-1 text-xs text-red-400 bg-red-500/20 px-2 py-0.5 rounded"><Youtube size={12} /> YouTube</span>}
            </label>
            <div className="flex gap-2">
              <div className="flex-1 relative">
                <input
                  type="text"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  placeholder="https://example.com/file.zip or YouTube URL..."
                  className="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
                  onKeyPress={(e) => e.key === 'Enter' && handleAddDownload()}
                  autoFocus
                />
              </div>
              <button
                onClick={handlePaste}
                className="px-4 py-3 bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded-lg transition-colors group"
                title="Paste from clipboard"
              >
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" className="text-gray-400 group-hover:text-white">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                  <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
              </button>
            </div>
          </div>

          {/* YouTube Options */}
          {isYouTube && (
            <div className="space-y-4 p-4 bg-red-900/10 rounded-lg border border-red-500/30">
              <div className="flex items-center gap-2 text-red-400">
                <Youtube size={20} />
                <span className="font-semibold">YouTube Download Options</span>
              </div>
              
              {/* Format Type */}
              <div className="space-y-2">
                <label className="block text-sm font-medium text-gray-300">
                  Download Type
                </label>
                <div className="grid grid-cols-2 gap-2">
                  <button
                    type="button"
                    onClick={() => setYoutubeFormat("video")}
                    className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                      youtubeFormat === "video"
                        ? "bg-red-600 text-white shadow-lg shadow-red-500/30"
                        : "bg-gray-700 text-gray-300 hover:bg-gray-600"
                    }`}
                  >
                    📹 Video + Audio
                  </button>
                  <button
                    type="button"
                    onClick={() => setYoutubeFormat("audio")}
                    className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                      youtubeFormat === "audio"
                        ? "bg-red-600 text-white shadow-lg shadow-red-500/30"
                        : "bg-gray-700 text-gray-300 hover:bg-gray-600"
                    }`}
                  >
                    🎵 Audio Only (MP3)
                  </button>
                </div>
              </div>

              {/* Quality Selector - Only for Video */}
              {youtubeFormat === "video" && (
                <div className="space-y-2">
                  <label className="block text-sm font-medium text-gray-300">
                    Video Quality
                  </label>
                  <div className="grid grid-cols-5 gap-2">
                    {["2160p", "1440p", "1080p", "720p", "best"].map((quality) => (
                      <button
                        key={quality}
                        type="button"
                        onClick={() => setYoutubeQuality(quality)}
                        className={`px-3 py-2 rounded-lg text-sm font-medium transition-all ${
                          youtubeQuality === quality
                            ? "bg-red-600 text-white shadow-lg shadow-red-500/30"
                            : "bg-gray-700 text-gray-300 hover:bg-gray-600"
                        }`}
                      >
                        {quality}
                      </button>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {/* File Name */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-gray-300">
              File Name (Optional)
            </label>
            <input
              type="text"
              value={fileName}
              onChange={(e) => setFileName(e.target.value)}
              placeholder="Leave empty to use original name"
              className="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
            />
          </div>

          {/* Save Location */}
          <div className="space-y-2">
            <label className="block text-sm font-medium text-gray-300 flex items-center gap-2">
              <FolderOpen size={16} />
              Save Location
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={savePath}
                onChange={(e) => setSavePath(e.target.value)}
                placeholder="Default download folder"
                className="flex-1 px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all"
                readOnly
              />
              <button
                onClick={handleSelectFolder}
                className="px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors flex items-center gap-2 font-medium"
              >
                <FolderOpen size={18} />
                Browse
              </button>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 p-6 border-t border-gray-800 bg-gray-900/50">
          <button
            onClick={onClose}
            className="px-5 py-2.5 text-gray-300 hover:bg-gray-800 rounded-lg transition-colors font-medium"
          >
            Cancel
          </button>
          <button
            onClick={handleAddDownload}
            disabled={!url.trim() || loading}
            className="px-5 py-2.5 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded-lg transition-all flex items-center gap-2 font-medium shadow-lg shadow-blue-500/20 disabled:shadow-none"
          >
            {loading ? (
              <>
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                Adding...
              </>
            ) : (
              <>
                <Plus size={18} />
                Add Download
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
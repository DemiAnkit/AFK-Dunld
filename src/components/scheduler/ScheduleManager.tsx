import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Calendar, Clock, Plus, Trash2, Edit2, Play } from 'lucide-react';
import toast from 'react-hot-toast';
import { format } from 'date-fns';

interface ScheduledDownload {
  id: string;
  download_id: string;
  scheduled_time: string;
  recurrence: string | null;
  enabled: boolean;
  url: string;
  file_name: string;
}

export const ScheduleManager = () => {
  const [schedules, setSchedules] = useState<ScheduledDownload[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddDialog, setShowAddDialog] = useState(false);

  useEffect(() => {
    loadSchedules();
  }, []);

  const loadSchedules = async () => {
    try {
      setLoading(true);
      const data = await invoke<ScheduledDownload[]>('get_scheduled_downloads');
      setSchedules(data);
    } catch (error) {
      console.error('Failed to load schedules:', error);
      toast.error('Failed to load scheduled downloads');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Delete this scheduled download?')) return;

    try {
      await invoke('cancel_scheduled_download', { scheduleId: id });
      toast.success('Schedule deleted');
      loadSchedules();
    } catch (error) {
      console.error('Failed to delete schedule:', error);
      toast.error('Failed to delete schedule');
    }
  };

  const handleToggle = async (schedule: ScheduledDownload) => {
    try {
      await invoke('update_scheduled_download', {
        scheduleId: schedule.id,
        enabled: !schedule.enabled,
      });
      toast.success(schedule.enabled ? 'Schedule disabled' : 'Schedule enabled');
      loadSchedules();
    } catch (error) {
      console.error('Failed to toggle schedule:', error);
      toast.error('Failed to update schedule');
    }
  };

  return (
    <div className="flex flex-col h-full bg-gray-950 p-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 className="text-2xl font-bold text-white">Scheduled Downloads</h2>
          <p className="text-gray-400 mt-1">Manage timed and recurring downloads</p>
        </div>
        <button
          onClick={() => setShowAddDialog(true)}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors flex items-center gap-2"
        >
          <Plus className="w-4 h-4" />
          Add Schedule
        </button>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">
          <div className="text-gray-400">Loading schedules...</div>
        </div>
      ) : schedules.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-64 text-gray-400">
          <Calendar className="w-16 h-16 mb-4 opacity-50" />
          <p className="text-lg">No scheduled downloads</p>
          <p className="text-sm">Create a schedule to download files at specific times</p>
        </div>
      ) : (
        <div className="space-y-4">
          {schedules.map((schedule) => (
            <div
              key={schedule.id}
              className="bg-gray-900 rounded-lg p-4 border border-gray-800 hover:border-gray-700 transition-colors"
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <h3 className="font-medium text-white mb-1">{schedule.file_name}</h3>
                  <p className="text-sm text-gray-400 mb-3 truncate">{schedule.url}</p>
                  
                  <div className="flex items-center gap-4 text-sm">
                    <div className="flex items-center gap-2 text-gray-400">
                      <Clock className="w-4 h-4" />
                      {format(new Date(schedule.scheduled_time), 'MMM dd, yyyy HH:mm')}
                    </div>
                    
                    {schedule.recurrence && (
                      <div className="px-2 py-1 bg-purple-900/30 text-purple-300 rounded text-xs">
                        {schedule.recurrence}
                      </div>
                    )}
                    
                    <div className={`px-2 py-1 rounded text-xs ${
                      schedule.enabled 
                        ? 'bg-green-900/30 text-green-300' 
                        : 'bg-gray-800 text-gray-400'
                    }`}>
                      {schedule.enabled ? 'Enabled' : 'Disabled'}
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => handleToggle(schedule)}
                    className={`p-2 rounded-lg transition-colors ${
                      schedule.enabled
                        ? 'bg-green-600 hover:bg-green-700 text-white'
                        : 'bg-gray-800 hover:bg-gray-700 text-gray-400'
                    }`}
                    title={schedule.enabled ? 'Disable' : 'Enable'}
                  >
                    <Play className="w-4 h-4" />
                  </button>
                  
                  <button
                    onClick={() => handleDelete(schedule.id)}
                    className="p-2 bg-gray-800 hover:bg-red-600 text-gray-400 hover:text-white rounded-lg transition-colors"
                    title="Delete"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default ScheduleManager;

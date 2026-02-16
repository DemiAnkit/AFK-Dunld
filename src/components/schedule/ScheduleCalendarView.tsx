// Schedule Calendar View Component
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ScheduledTask {
  id: string;
  download_id: string;
  scheduled_time: string;
  repeat_interval: string | null;
  enabled: boolean;
}

interface CalendarDay {
  date: Date;
  isCurrentMonth: boolean;
  tasks: ScheduledTask[];
}

export function ScheduleCalendarView() {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [scheduledTasks, setScheduledTasks] = useState<ScheduledTask[]>([]);
  const [calendarDays, setCalendarDays] = useState<CalendarDay[]>([]);
  const [selectedTask, setSelectedTask] = useState<ScheduledTask | null>(null);

  useEffect(() => {
    loadScheduledTasks();
  }, []);

  useEffect(() => {
    generateCalendar();
  }, [currentDate, scheduledTasks]);

  const loadScheduledTasks = async () => {
    try {
      const tasks = await invoke<ScheduledTask[]>('get_scheduled_downloads');
      setScheduledTasks(tasks);
    } catch (error) {
      console.error('Failed to load scheduled tasks:', error);
    }
  };

  const generateCalendar = () => {
    const year = currentDate.getFullYear();
    const month = currentDate.getMonth();
    
    const firstDay = new Date(year, month, 1);
    const lastDay = new Date(year, month + 1, 0);
    const startDate = new Date(firstDay);
    startDate.setDate(startDate.getDate() - startDate.getDay());
    
    const days: CalendarDay[] = [];
    const current = new Date(startDate);
    
    for (let i = 0; i < 42; i++) {
      const isCurrentMonth = current.getMonth() === month;
      const dayTasks = scheduledTasks.filter(task => {
        const taskDate = new Date(task.scheduled_time);
        return (
          taskDate.getDate() === current.getDate() &&
          taskDate.getMonth() === current.getMonth() &&
          taskDate.getFullYear() === current.getFullYear()
        );
      });
      
      days.push({
        date: new Date(current),
        isCurrentMonth,
        tasks: dayTasks,
      });
      
      current.setDate(current.getDate() + 1);
    }
    
    setCalendarDays(days);
  };

  const previousMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() - 1, 1));
  };

  const nextMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 1));
  };

  const formatTime = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
  };

  const getRepeatLabel = (interval: string | null) => {
    if (!interval) return 'Once';
    if (interval === 'hourly') return 'Hourly';
    if (interval === 'daily') return 'Daily';
    if (interval === 'weekly') return 'Weekly';
    if (interval === 'monthly') return 'Monthly';
    if (interval.startsWith('custom:')) {
      const seconds = parseInt(interval.split(':')[1]);
      return `Every ${seconds / 3600}h`;
    }
    return interval;
  };

  const isToday = (date: Date) => {
    const today = new Date();
    return (
      date.getDate() === today.getDate() &&
      date.getMonth() === today.getMonth() &&
      date.getFullYear() === today.getFullYear()
    );
  };

  const toggleTaskEnabled = async (taskId: string, enabled: boolean) => {
    try {
      await invoke('update_scheduled_download', {
        taskId,
        enabled: !enabled,
      });
      await loadScheduledTasks();
    } catch (error) {
      console.error('Failed to toggle task:', error);
    }
  };

  const deleteTask = async (taskId: string) => {
    try {
      await invoke('cancel_scheduled_download', { taskId });
      await loadScheduledTasks();
      setSelectedTask(null);
    } catch (error) {
      console.error('Failed to delete task:', error);
    }
  };

  return (
    <div className="schedule-calendar-view p-4">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold">Schedule Calendar</h2>
        <div className="flex items-center gap-4">
          <button
            onClick={previousMonth}
            className="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
          >
            ←
          </button>
          <span className="text-lg font-semibold min-w-[150px] text-center">
            {currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
          </span>
          <button
            onClick={nextMonth}
            className="px-3 py-1 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
          >
            →
          </button>
        </div>
      </div>

      {/* Calendar Grid */}
      <div className="grid grid-cols-7 gap-2">
        {/* Day headers */}
        {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map(day => (
          <div key={day} className="text-center font-semibold p-2 text-sm text-gray-600 dark:text-gray-400">
            {day}
          </div>
        ))}
        
        {/* Calendar days */}
        {calendarDays.map((day, index) => (
          <div
            key={index}
            className={`
              min-h-[100px] p-2 border rounded-lg
              ${day.isCurrentMonth ? 'bg-white dark:bg-gray-800' : 'bg-gray-50 dark:bg-gray-900'}
              ${isToday(day.date) ? 'ring-2 ring-blue-500' : 'border-gray-200 dark:border-gray-700'}
              hover:shadow-md transition-shadow cursor-pointer
            `}
          >
            <div className={`text-sm font-semibold mb-1 ${isToday(day.date) ? 'text-blue-600 dark:text-blue-400' : ''}`}>
              {day.date.getDate()}
            </div>
            
            {/* Tasks for this day */}
            <div className="space-y-1">
              {day.tasks.slice(0, 3).map(task => (
                <div
                  key={task.id}
                  onClick={() => setSelectedTask(task)}
                  className={`
                    text-xs p-1 rounded truncate
                    ${task.enabled ? 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200' : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400'}
                    hover:shadow-sm transition-shadow
                  `}
                  title={`${formatTime(task.scheduled_time)} - ${getRepeatLabel(task.repeat_interval)}`}
                >
                  {formatTime(task.scheduled_time)}
                </div>
              ))}
              {day.tasks.length > 3 && (
                <div className="text-xs text-gray-500 dark:text-gray-400 text-center">
                  +{day.tasks.length - 3} more
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* Task Details Modal */}
      {selectedTask && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-xl font-bold mb-4">Scheduled Task Details</h3>
            
            <div className="space-y-3 mb-6">
              <div>
                <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Download ID:</label>
                <p className="text-sm font-mono">{selectedTask.download_id}</p>
              </div>
              
              <div>
                <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Scheduled Time:</label>
                <p>{new Date(selectedTask.scheduled_time).toLocaleString()}</p>
              </div>
              
              <div>
                <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Repeat:</label>
                <p>{getRepeatLabel(selectedTask.repeat_interval)}</p>
              </div>
              
              <div>
                <label className="text-sm font-semibold text-gray-600 dark:text-gray-400">Status:</label>
                <p className={selectedTask.enabled ? 'text-green-600 dark:text-green-400' : 'text-gray-600 dark:text-gray-400'}>
                  {selectedTask.enabled ? 'Enabled' : 'Disabled'}
                </p>
              </div>
            </div>
            
            <div className="flex gap-3">
              <button
                onClick={() => toggleTaskEnabled(selectedTask.id, selectedTask.enabled)}
                className="flex-1 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
              >
                {selectedTask.enabled ? 'Disable' : 'Enable'}
              </button>
              
              <button
                onClick={() => deleteTask(selectedTask.id)}
                className="flex-1 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
              >
                Delete
              </button>
              
              <button
                onClick={() => setSelectedTask(null)}
                className="flex-1 px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-800 dark:text-gray-200 rounded hover:bg-gray-400 dark:hover:bg-gray-500"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Legend */}
      <div className="mt-6 flex items-center gap-6 text-sm">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-blue-100 dark:bg-blue-900 rounded"></div>
          <span>Enabled</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-gray-200 dark:bg-gray-700 rounded"></div>
          <span>Disabled</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 border-2 border-blue-500 rounded"></div>
          <span>Today</span>
        </div>
      </div>
    </div>
  );
}

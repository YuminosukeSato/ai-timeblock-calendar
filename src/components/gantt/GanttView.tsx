import { useEffect, useRef } from 'react';

import Gantt from 'frappe-gantt';
/* frappe-gantt CSS loaded via style import in main.tsx */

import { toFrappeTasks } from '../../lib/ganttAdapter';
import { GanttTask } from '../../types';

interface GanttViewProps {
  tasks: GanttTask[];
  viewMode?: 'Day' | 'Week' | 'Month';
  onTaskDateChange?: (taskId: string, start: string, end: string) => void;
  onProgressChange?: (taskId: string, progress: number) => void;
  onTaskClick?: (taskId: string) => void;
}

export function GanttView({
  tasks,
  viewMode = 'Day',
  onTaskDateChange,
  onProgressChange,
  onTaskClick
}: GanttViewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const ganttRef = useRef<Gantt | null>(null);

  useEffect(() => {
    if (!containerRef.current || tasks.length === 0) {
      return;
    }

    const frappeTasks = toFrappeTasks(tasks);

    ganttRef.current = new Gantt(containerRef.current, frappeTasks, {
      view_mode: viewMode,
      bar_height: 30,
      padding: 18,
      column_width: viewMode === 'Week' ? 90 : viewMode === 'Month' ? 200 : 40,
      on_date_change: onTaskDateChange
        ? (task: Gantt.Task, start: Date, end: Date) => {
            onTaskDateChange(
              task.id ?? '',
              start.toISOString().slice(0, 10),
              end.toISOString().slice(0, 10)
            );
          }
        : undefined,
      on_progress_change: onProgressChange
        ? (task: Gantt.Task, progress: number) => {
            onProgressChange(task.id ?? '', progress);
          }
        : undefined,
      on_click: onTaskClick
        ? (task: Gantt.Task) => {
            onTaskClick(task.id ?? '');
          }
        : undefined
    });

    return () => {
      ganttRef.current = null;
    };
  }, [tasks, viewMode, onTaskDateChange, onProgressChange, onTaskClick]);

  if (tasks.length === 0) {
    return (
      <div data-testid="gantt-container">
        <p>No tasks to display</p>
      </div>
    );
  }

  return <div ref={containerRef} data-testid="gantt-container" />;
}

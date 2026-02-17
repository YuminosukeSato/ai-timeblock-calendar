import { GanttTask } from '../types';

export interface FrappeTask {
  id: string;
  name: string;
  start: string;
  end: string;
  progress: number;
  dependencies: string;
}

function isoToDate(iso: string): string {
  return iso.slice(0, 10);
}

export function toFrappeTask(task: GanttTask): FrappeTask {
  return {
    id: task.id,
    name: task.title,
    start: isoToDate(task.start),
    end: isoToDate(task.end),
    progress: task.progress,
    dependencies: task.dependsOn.join(', ')
  };
}

export function toFrappeTasks(tasks: GanttTask[]): FrappeTask[] {
  return tasks.map(toFrappeTask);
}

export function fromFrappeDateChange(
  taskId: string,
  start: Date,
  end: Date
): { id: string; start: string; end: string } {
  return {
    id: taskId,
    start: start.toISOString().slice(0, 10),
    end: end.toISOString().slice(0, 10)
  };
}

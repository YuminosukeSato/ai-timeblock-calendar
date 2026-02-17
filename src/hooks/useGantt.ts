import { GanttTask } from '../types';

export interface UpdateProgressResult {
  tasks?: GanttTask[];
  warning?: string;
  error?: string;
}

export function getGanttData(tasks: GanttTask[]): GanttTask[] {
  const taskMap = new Map(tasks.map((task) => [task.id, task]));
  const visited = new Set<string>();
  const visiting = new Set<string>();
  const order: GanttTask[] = [];

  const visit = (id: string) => {
    if (visiting.has(id)) {
      throw new Error('circular dependency detected');
    }
    if (visited.has(id)) {
      return;
    }

    const task = taskMap.get(id);
    if (!task) {
      return;
    }

    visiting.add(id);
    task.dependsOn.forEach(visit);
    visiting.delete(id);
    visited.add(id);
    order.push(task);
  };

  tasks.forEach((task) => visit(task.id));
  return order;
}

export function updateProgress(
  tasks: GanttTask[],
  taskId: string,
  progress: number
): UpdateProgressResult {
  if (progress < 0 || progress > 100) {
    return { error: 'progress must be between 0 and 100' };
  }

  const task = tasks.find((item) => item.id === taskId);
  if (!task) {
    return { error: 'task not found' };
  }

  const unresolvedDep = task.dependsOn.find((depId) => {
    const dep = tasks.find((item) => item.id === depId);
    return dep ? dep.progress < 100 : false;
  });

  const updated = tasks.map((item) =>
    item.id === taskId ? { ...item, progress } : item
  );

  return {
    tasks: updated,
    warning: unresolvedDep ? 'dependency task is not completed' : undefined
  };
}

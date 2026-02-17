import { create } from 'zustand';

import { GanttTask } from '../types';

interface GanttStore {
  tasks: GanttTask[];
  setTasks: (tasks: GanttTask[]) => void;
}

export const useGanttStore = create<GanttStore>((set) => ({
  tasks: [],
  setTasks: (tasks) => set({ tasks })
}));

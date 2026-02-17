import { create } from 'zustand';

import { CalendarView } from '../hooks/useCalendarView';

interface CalendarStore {
  view: CalendarView;
  setView: (view: CalendarView) => void;
}

export const useCalendarStore = create<CalendarStore>((set) => ({
  view: 'week',
  setView: (view) => set({ view })
}));

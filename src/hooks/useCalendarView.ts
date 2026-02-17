export type CalendarView = 'day' | 'week' | 'month' | 'gantt';

export function nextCalendarView(view: CalendarView): CalendarView {
  const sequence: CalendarView[] = ['day', 'week', 'month', 'gantt'];
  const idx = sequence.indexOf(view);
  return sequence[(idx + 1) % sequence.length];
}

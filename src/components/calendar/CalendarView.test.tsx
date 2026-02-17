import 'temporal-polyfill/global';

import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { TimeBlock } from '../../types';

vi.mock('@schedule-x/react', () => ({
  useCalendarApp: vi.fn(() => ({})),
  ScheduleXCalendar: ({ calendarApp }: { calendarApp: unknown }) => (
    <div
      data-testid="sx-calendar-mock"
      data-app={calendarApp ? 'loaded' : 'empty'}
    />
  )
}));

vi.mock('@schedule-x/calendar', () => ({
  createViewDay: vi.fn(() => ({ name: 'day' })),
  createViewWeek: vi.fn(() => ({ name: 'week' })),
  createViewMonthGrid: vi.fn(() => ({ name: 'month-grid' }))
}));

vi.mock('@schedule-x/drag-and-drop', () => ({
  createDragAndDropPlugin: vi.fn(() => ({}))
}));

vi.mock('@schedule-x/event-modal', () => ({
  createEventModalPlugin: vi.fn(() => ({}))
}));

vi.mock('@schedule-x/resize', () => ({
  createResizePlugin: vi.fn(() => ({}))
}));

vi.mock('@schedule-x/events-service', () => ({
  createEventsServicePlugin: vi.fn(() => ({
    getAll: vi.fn(() => []),
    add: vi.fn(),
    update: vi.fn(),
    remove: vi.fn()
  }))
}));

vi.mock('@schedule-x/theme-default/dist/index.css', () => ({}));

import { CalendarView } from './CalendarView';

const blocks: TimeBlock[] = [
  {
    id: 'tb-1',
    title: 'Design Review',
    startTime: '2026-02-17T09:00:00Z',
    endTime: '2026-02-17T10:00:00Z',
    categoryId: 'work',
    progress: 80,
    syncStatus: 'synced'
  },
  {
    id: 'tb-2',
    title: 'Lunch',
    startTime: '2026-02-17T12:00:00Z',
    endTime: '2026-02-17T13:00:00Z',
    categoryId: 'personal',
    progress: 0,
    syncStatus: 'local'
  }
];

describe('CalendarView', () => {
  it('renders the calendar container', () => {
    render(<CalendarView blocks={blocks} />);
    expect(screen.getByTestId('schedule-x-calendar')).toBeInTheDocument();
  });

  it('renders the mocked ScheduleXCalendar component', () => {
    render(<CalendarView blocks={blocks} />);
    expect(screen.getByTestId('sx-calendar-mock')).toBeInTheDocument();
  });

  it('passes calendarApp to ScheduleXCalendar', () => {
    render(<CalendarView blocks={blocks} />);
    const mock = screen.getByTestId('sx-calendar-mock');
    expect(mock.getAttribute('data-app')).toBe('loaded');
  });

  it('renders with empty blocks without errors', () => {
    render(<CalendarView blocks={[]} />);
    expect(screen.getByTestId('schedule-x-calendar')).toBeInTheDocument();
  });

  it('calls useCalendarApp with correct config', async () => {
    const { useCalendarApp } = await import('@schedule-x/react');
    render(<CalendarView blocks={blocks} view="day" />);
    expect(useCalendarApp).toHaveBeenCalled();
    const config = vi.mocked(useCalendarApp).mock.calls[0][0];
    expect(config.views).toHaveLength(3);
    expect(config.events).toHaveLength(2);
    expect(config.calendars).toBeDefined();
    expect(config.plugins).toBeDefined();
  });

  it('converts blocks to schedule-x Temporal events in config', async () => {
    const { useCalendarApp } = await import('@schedule-x/react');
    render(<CalendarView blocks={blocks} />);
    const config = vi.mocked(useCalendarApp).mock.calls[0][0];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const events = config.events as any as Array<{
      id: string;
      start: Temporal.ZonedDateTime;
      end: Temporal.ZonedDateTime;
      calendarId: string;
    }>;
    expect(events[0].id).toBe('tb-1');
    expect(events[0].start).toBeInstanceOf(Temporal.ZonedDateTime);
    expect(events[0].start.toString()).toContain('2026-02-17T09:00:00');
    expect(events[0].end.toString()).toContain('2026-02-17T10:00:00');
    expect(events[0].calendarId).toBe('work');
    expect(events[1].id).toBe('tb-2');
    expect(events[1].calendarId).toBe('personal');
  });

  it('includes onEventUpdate callback when prop is provided', async () => {
    const { useCalendarApp } = await import('@schedule-x/react');
    const onUpdate = vi.fn();
    render(<CalendarView blocks={blocks} onEventUpdate={onUpdate} />);
    const config = vi.mocked(useCalendarApp).mock.calls[0][0];
    expect(config.callbacks?.onEventUpdate).toBeDefined();
  });

  it('includes onEventClick callback when prop is provided', async () => {
    const { useCalendarApp } = await import('@schedule-x/react');
    const onClick = vi.fn();
    render(<CalendarView blocks={blocks} onEventClick={onClick} />);
    const config = vi.mocked(useCalendarApp).mock.calls[0][0];
    expect(config.callbacks?.onEventClick).toBeDefined();
  });
});

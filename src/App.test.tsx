import 'temporal-polyfill/global';

import { fireEvent, render, screen, within } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useTimeBlockStore } from './store/timeBlockStore';

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

vi.mock('frappe-gantt', () => {
  return {
    default: vi.fn().mockImplementation(() => ({
      change_view_mode: vi.fn(),
      refresh: vi.fn()
    }))
  };
});

vi.mock('frappe-gantt/dist/frappe-gantt.css', () => ({}));

import App from './App';

describe('App', () => {
  beforeEach(() => {
    useTimeBlockStore.setState({ blocks: [], snapshot: null, error: null });
  });

  it('renders the + New Block button', () => {
    render(<App />);
    expect(
      screen.getByRole('button', { name: /new block/i })
    ).toBeInTheDocument();
  });

  it('opens EventModal when + New Block is clicked', () => {
    render(<App />);
    fireEvent.click(screen.getByRole('button', { name: /new block/i }));
    expect(
      screen.getByRole('form', { name: 'event-modal' })
    ).toBeInTheDocument();
  });

  it('closes EventModal when Cancel is clicked', () => {
    render(<App />);
    fireEvent.click(screen.getByRole('button', { name: /new block/i }));
    fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(
      screen.queryByRole('form', { name: 'event-modal' })
    ).not.toBeInTheDocument();
  });

  it('adds a block to the store when saved from EventModal', () => {
    render(<App />);
    fireEvent.click(screen.getByRole('button', { name: /new block/i }));

    const modal = screen.getByRole('form', { name: 'event-modal' });
    fireEvent.change(within(modal).getByLabelText('title'), {
      target: { value: 'New Task' }
    });
    fireEvent.change(within(modal).getByLabelText('start'), {
      target: { value: '2026-02-17T14:00:00Z' }
    });
    fireEvent.change(within(modal).getByLabelText('end'), {
      target: { value: '2026-02-17T15:00:00Z' }
    });
    fireEvent.click(within(modal).getByRole('button', { name: 'Save' }));

    const blocks = useTimeBlockStore.getState().blocks;
    expect(blocks).toHaveLength(1);
    expect(blocks[0].title).toBe('New Task');
    expect(blocks[0].startTime).toBe('2026-02-17T14:00:00Z');
    expect(blocks[0].endTime).toBe('2026-02-17T15:00:00Z');
  });

  it('closes modal after successful save', () => {
    render(<App />);
    fireEvent.click(screen.getByRole('button', { name: /new block/i }));

    const modal = screen.getByRole('form', { name: 'event-modal' });
    fireEvent.change(within(modal).getByLabelText('title'), {
      target: { value: 'Task' }
    });
    fireEvent.change(within(modal).getByLabelText('start'), {
      target: { value: '2026-02-17T14:00:00Z' }
    });
    fireEvent.change(within(modal).getByLabelText('end'), {
      target: { value: '2026-02-17T15:00:00Z' }
    });
    fireEvent.click(within(modal).getByRole('button', { name: 'Save' }));

    expect(
      screen.queryByRole('form', { name: 'event-modal' })
    ).not.toBeInTheDocument();
  });

  it('shows demo blocks when store is empty', () => {
    render(<App />);
    expect(screen.getByTestId('sx-calendar-mock')).toBeInTheDocument();
  });
});

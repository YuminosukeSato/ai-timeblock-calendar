import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach } from 'vitest';

import { GanttTask } from '../../types';

const mockRefresh = vi.fn();

vi.mock('frappe-gantt', () => {
  return {
    default: vi
      .fn()
      .mockImplementation((_el: HTMLElement, tasks: unknown[]) => ({
        tasks,
        refresh: mockRefresh,
        change_view_mode: vi.fn()
      }))
  };
});

import Gantt from 'frappe-gantt';
import { GanttView } from './GanttView';

const tasks: GanttTask[] = [
  {
    id: 'a',
    title: 'design',
    start: '2026-02-17T09:00:00Z',
    end: '2026-02-18T09:00:00Z',
    progress: 50,
    dependsOn: []
  },
  {
    id: 'b',
    title: 'impl',
    start: '2026-02-18T09:00:00Z',
    end: '2026-02-20T09:00:00Z',
    progress: 20,
    dependsOn: ['a']
  }
];

describe('GanttView', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the gantt container element', () => {
    render(<GanttView tasks={tasks} />);
    expect(screen.getByTestId('gantt-container')).toBeInTheDocument();
  });

  it('initializes Gantt with converted tasks', () => {
    render(<GanttView tasks={tasks} />);
    expect(Gantt).toHaveBeenCalledTimes(1);
    const callArgs = vi.mocked(Gantt).mock.calls[0];
    const passedTasks = callArgs[1] as Array<{
      id: string;
      name: string;
      dependencies: string;
    }>;
    expect(passedTasks).toHaveLength(2);
    expect(passedTasks[0].id).toBe('a');
    expect(passedTasks[0].name).toBe('design');
    expect(passedTasks[0].dependencies).toBe('');
    expect(passedTasks[1].id).toBe('b');
    expect(passedTasks[1].name).toBe('impl');
    expect(passedTasks[1].dependencies).toBe('a');
  });

  it('passes correct date format from tasks', () => {
    render(<GanttView tasks={tasks} />);
    const callArgs = vi.mocked(Gantt).mock.calls[0];
    const passedTasks = callArgs[1] as Array<{ start: string; end: string }>;
    expect(passedTasks[0].start).toBe('2026-02-17');
    expect(passedTasks[0].end).toBe('2026-02-18');
  });

  it('passes progress values correctly', () => {
    render(<GanttView tasks={tasks} />);
    const callArgs = vi.mocked(Gantt).mock.calls[0];
    const passedTasks = callArgs[1] as Array<{ progress: number }>;
    expect(passedTasks[0].progress).toBe(50);
    expect(passedTasks[1].progress).toBe(20);
  });

  it('configures on_date_change callback when onTaskDateChange is provided', () => {
    const onDateChange = vi.fn();
    render(<GanttView tasks={tasks} onTaskDateChange={onDateChange} />);
    const callArgs = vi.mocked(Gantt).mock.calls[0];
    const options = callArgs[2] as { on_date_change: unknown };
    expect(options.on_date_change).toBeDefined();
  });

  it('configures on_progress_change callback when onProgressChange is provided', () => {
    const onProgress = vi.fn();
    render(<GanttView tasks={tasks} onProgressChange={onProgress} />);
    const callArgs = vi.mocked(Gantt).mock.calls[0];
    const options = callArgs[2] as { on_progress_change: unknown };
    expect(options.on_progress_change).toBeDefined();
  });

  it('does not crash with empty tasks', () => {
    render(<GanttView tasks={[]} />);
    expect(screen.getByTestId('gantt-container')).toBeInTheDocument();
  });

  it('renders empty state message when tasks are empty', () => {
    render(<GanttView tasks={[]} />);
    expect(screen.getByText('No tasks to display')).toBeInTheDocument();
  });
});

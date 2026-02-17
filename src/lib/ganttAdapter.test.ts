import { describe, expect, it } from 'vitest';

import { GanttTask } from '../types';

import {
  toFrappeTask,
  toFrappeTasks,
  fromFrappeDateChange
} from './ganttAdapter';

describe('toFrappeTask', () => {
  const task: GanttTask = {
    id: 'g-1',
    title: 'Design Phase',
    start: '2026-02-17T09:00:00Z',
    end: '2026-02-20T17:00:00Z',
    progress: 60,
    dependsOn: ['g-0']
  };

  it('maps id directly', () => {
    expect(toFrappeTask(task).id).toBe('g-1');
  });

  it('maps title to name', () => {
    expect(toFrappeTask(task).name).toBe('Design Phase');
  });

  it('extracts date portion from ISO start/end', () => {
    const result = toFrappeTask(task);
    expect(result.start).toBe('2026-02-17');
    expect(result.end).toBe('2026-02-20');
  });

  it('maps progress directly', () => {
    expect(toFrappeTask(task).progress).toBe(60);
  });

  it('joins dependsOn array into comma-separated dependencies string', () => {
    expect(toFrappeTask(task).dependencies).toBe('g-0');
  });

  it('handles multiple dependencies', () => {
    const multi: GanttTask = { ...task, dependsOn: ['a', 'b', 'c'] };
    expect(toFrappeTask(multi).dependencies).toBe('a, b, c');
  });

  it('handles empty dependsOn', () => {
    const noDeps: GanttTask = { ...task, dependsOn: [] };
    expect(toFrappeTask(noDeps).dependencies).toBe('');
  });

  it('handles zero progress', () => {
    const zero: GanttTask = { ...task, progress: 0 };
    expect(toFrappeTask(zero).progress).toBe(0);
  });

  it('handles 100% progress', () => {
    const full: GanttTask = { ...task, progress: 100 };
    expect(toFrappeTask(full).progress).toBe(100);
  });
});

describe('toFrappeTasks', () => {
  it('converts empty array', () => {
    expect(toFrappeTasks([])).toEqual([]);
  });

  it('preserves order of tasks', () => {
    const tasks: GanttTask[] = [
      {
        id: 'a',
        title: 'A',
        start: '2026-02-17T00:00:00Z',
        end: '2026-02-18T00:00:00Z',
        progress: 0,
        dependsOn: []
      },
      {
        id: 'b',
        title: 'B',
        start: '2026-02-19T00:00:00Z',
        end: '2026-02-20T00:00:00Z',
        progress: 50,
        dependsOn: ['a']
      }
    ];
    const result = toFrappeTasks(tasks);
    expect(result).toHaveLength(2);
    expect(result[0].id).toBe('a');
    expect(result[1].id).toBe('b');
    expect(result[1].dependencies).toBe('a');
  });
});

describe('fromFrappeDateChange', () => {
  it('converts Date objects to ISO date strings', () => {
    const start = new Date('2026-02-18T00:00:00Z');
    const end = new Date('2026-02-22T00:00:00Z');
    const result = fromFrappeDateChange('g-1', start, end);
    expect(result.id).toBe('g-1');
    expect(result.start).toBe('2026-02-18');
    expect(result.end).toBe('2026-02-22');
  });
});

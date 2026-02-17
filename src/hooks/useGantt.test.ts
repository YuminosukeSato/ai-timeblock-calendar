import { describe, expect, it } from 'vitest';

import { getGanttData, updateProgress } from './useGantt';

describe('useGantt', () => {
  it('getGanttData returns tasks in dependency order', () => {
    const tasks = getGanttData([
      {
        id: 'b',
        title: 'impl',
        start: '2026-02-18T09:00:00Z',
        end: '2026-02-18T10:00:00Z',
        progress: 0,
        dependsOn: ['a']
      },
      {
        id: 'a',
        title: 'design',
        start: '2026-02-17T09:00:00Z',
        end: '2026-02-17T10:00:00Z',
        progress: 100,
        dependsOn: []
      }
    ]);

    expect(tasks[0].id).toBe('a');
    expect(tasks[1].id).toBe('b');
  });

  it('getGanttData throws error for circular dependencies', () => {
    expect(() =>
      getGanttData([
        {
          id: 'a',
          title: 'A',
          start: '2026-02-17T09:00:00Z',
          end: '2026-02-17T10:00:00Z',
          progress: 0,
          dependsOn: ['b']
        },
        {
          id: 'b',
          title: 'B',
          start: '2026-02-17T10:00:00Z',
          end: '2026-02-17T11:00:00Z',
          progress: 0,
          dependsOn: ['a']
        }
      ])
    ).toThrow(/circular/);
  });

  it('updateProgress returns validation error for out of range progress', () => {
    const result = updateProgress(
      [
        {
          id: 'a',
          title: 'A',
          start: '2026-02-17T09:00:00Z',
          end: '2026-02-17T10:00:00Z',
          progress: 0,
          dependsOn: []
        }
      ],
      'a',
      120
    );

    expect(result.error).toContain('between 0 and 100');
  });

  it('updateProgress returns warning when dependency incomplete', () => {
    const result = updateProgress(
      [
        {
          id: 'a',
          title: 'A',
          start: '2026-02-17T09:00:00Z',
          end: '2026-02-17T10:00:00Z',
          progress: 50,
          dependsOn: []
        },
        {
          id: 'b',
          title: 'B',
          start: '2026-02-17T10:00:00Z',
          end: '2026-02-17T11:00:00Z',
          progress: 0,
          dependsOn: ['a']
        }
      ],
      'b',
      10
    );

    expect(result.warning).toContain('dependency');
  });
});

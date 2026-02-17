import { describe, expect, it } from 'vitest';

import { createTimeBlocksManager } from './useTimeBlocks';

describe('useTimeBlocks manager', () => {
  it('fetchBlocks returns blocks inside range', () => {
    const manager = createTimeBlocksManager([
      {
        id: '1',
        title: 'task',
        startTime: '2026-02-17T09:00:00Z',
        endTime: '2026-02-17T10:00:00Z',
        progress: 0,
        syncStatus: 'local'
      }
    ]);

    const blocks = manager.fetchBlocks(
      '2026-02-17T00:00:00Z',
      '2026-02-18T00:00:00Z'
    );

    expect(blocks).toHaveLength(1);
  });

  it('fetchBlocks returns empty array for empty period', () => {
    const manager = createTimeBlocksManager([]);

    const blocks = manager.fetchBlocks(
      '2026-02-17T00:00:00Z',
      '2026-02-18T00:00:00Z'
    );

    expect(blocks).toEqual([]);
  });

  it('createBlock returns validation error when end before start', () => {
    const manager = createTimeBlocksManager([]);

    const result = manager.createBlock({
      title: 'task',
      startTime: '2026-02-17T10:00:00Z',
      endTime: '2026-02-17T09:00:00Z'
    });

    expect(result.error).toContain('end time');
  });

  it('createBlock returns overlap warning', () => {
    const manager = createTimeBlocksManager([
      {
        id: '1',
        title: 'seed',
        startTime: '2026-02-17T09:00:00Z',
        endTime: '2026-02-17T10:00:00Z',
        progress: 0,
        syncStatus: 'local'
      }
    ]);

    const result = manager.createBlock({
      title: 'new',
      startTime: '2026-02-17T09:30:00Z',
      endTime: '2026-02-17T10:30:00Z'
    });

    expect(result.warning).toContain('overlap');
  });

  it('deleteBlock returns error for missing id', () => {
    const manager = createTimeBlocksManager([]);

    const result = manager.deleteBlock('missing');

    expect(result.error).toContain('not found');
  });

  it('resolveConflict supports local and remote paths', () => {
    const manager = createTimeBlocksManager([]);
    const local = {
      id: '1',
      title: 'local',
      startTime: '2026-02-17T09:00:00Z',
      endTime: '2026-02-17T10:00:00Z',
      progress: 0,
      syncStatus: 'local' as const
    };
    const remote = {
      ...local,
      title: 'remote'
    };

    expect(manager.resolveConflict('local', local, remote).title).toBe('local');
    expect(manager.resolveConflict('remote', local, remote).title).toBe(
      'remote'
    );
  });
});

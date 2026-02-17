import { beforeEach, describe, expect, it } from 'vitest';

import { useTimeBlockStore } from './timeBlockStore';

describe('timeBlockStore', () => {
  beforeEach(() => {
    useTimeBlockStore.setState({ blocks: [], snapshot: null, error: null });
  });

  it('initial state is empty', () => {
    expect(useTimeBlockStore.getState().blocks).toEqual([]);
  });

  it('createBlock performs optimistic update', () => {
    useTimeBlockStore.getState().createBlockOptimistic({
      id: '1',
      title: 'task',
      startTime: '2026-02-17T09:00:00Z',
      endTime: '2026-02-17T10:00:00Z',
      progress: 0,
      syncStatus: 'local'
    });

    expect(useTimeBlockStore.getState().blocks).toHaveLength(1);
  });

  it('rollback restores previous snapshot after failure', () => {
    const state = useTimeBlockStore.getState();
    state.createBlockOptimistic({
      id: '1',
      title: 'task',
      startTime: '2026-02-17T09:00:00Z',
      endTime: '2026-02-17T10:00:00Z',
      progress: 0,
      syncStatus: 'local'
    });

    useTimeBlockStore.getState().rollback();

    expect(useTimeBlockStore.getState().blocks).toEqual([]);
  });
});

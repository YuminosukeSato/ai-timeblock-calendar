import { create } from 'zustand';

import { TimeBlock } from '../types';

interface TimeBlockState {
  blocks: TimeBlock[];
  snapshot: TimeBlock[] | null;
  error: string | null;
  setBlocks: (blocks: TimeBlock[]) => void;
  createBlockOptimistic: (block: TimeBlock) => void;
  updateBlockOptimistic: (id: string, patch: Partial<TimeBlock>) => void;
  rollback: () => void;
  commit: () => void;
  deleteBlockOptimistic: (id: string) => void;
}

export const useTimeBlockStore = create<TimeBlockState>((set, get) => ({
  blocks: [],
  snapshot: null,
  error: null,
  setBlocks: (blocks) => set({ blocks }),
  createBlockOptimistic: (block) =>
    set((state) => ({
      snapshot: state.blocks,
      blocks: [...state.blocks, block],
      error: null
    })),
  rollback: () => {
    const snapshot = get().snapshot;
    if (snapshot) {
      set({ blocks: snapshot, snapshot: null });
    }
  },
  updateBlockOptimistic: (id, patch) => {
    const state = get();
    const index = state.blocks.findIndex((block) => block.id === id);
    if (index === -1) {
      throw new Error('time block not found');
    }
    const updated = { ...state.blocks[index], ...patch, id };
    const blocks = [...state.blocks];
    blocks[index] = updated;
    set({ snapshot: state.blocks, blocks, error: null });
  },
  commit: () => set({ snapshot: null }),
  deleteBlockOptimistic: (id) => {
    const state = get();
    const exists = state.blocks.some((block) => block.id === id);
    if (!exists) {
      throw new Error('time block not found');
    }
    set({
      snapshot: state.blocks,
      blocks: state.blocks.filter((block) => block.id !== id),
      error: null
    });
  }
}));

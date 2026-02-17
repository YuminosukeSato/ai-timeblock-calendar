import { create } from 'zustand';

import { TimeBlock } from '../types';

interface TimeBlockState {
  blocks: TimeBlock[];
  snapshot: TimeBlock[] | null;
  error: string | null;
  setBlocks: (blocks: TimeBlock[]) => void;
  createBlockOptimistic: (block: TimeBlock) => void;
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

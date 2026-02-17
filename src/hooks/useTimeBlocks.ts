import { ConflictResolution, TimeBlock, TimeBlockInput } from '../types';

export interface CreateBlockResult {
  block?: TimeBlock;
  error?: string;
  warning?: string;
}

export interface TimeBlocksManager {
  getAll: () => TimeBlock[];
  fetchBlocks: (start: string, end: string) => TimeBlock[];
  createBlock: (input: TimeBlockInput) => CreateBlockResult;
  deleteBlock: (id: string) => { error?: string };
  resolveConflict: (
    mode: ConflictResolution,
    local: TimeBlock,
    remote: TimeBlock
  ) => TimeBlock;
}

export function createTimeBlocksManager(
  initial: TimeBlock[] = []
): TimeBlocksManager {
  let blocks = [...initial];
  const generateId = () => {
    if (
      typeof globalThis.crypto !== 'undefined' &&
      typeof globalThis.crypto.randomUUID === 'function'
    ) {
      return globalThis.crypto.randomUUID();
    }
    return `block-${Date.now()}-${Math.random().toString(16).slice(2)}`;
  };

  return {
    getAll: () => blocks,
    fetchBlocks: (start, end) => {
      const startDate = new Date(start);
      const endDate = new Date(end);
      return blocks.filter((block) => {
        const blockStart = new Date(block.startTime);
        const blockEnd = new Date(block.endTime);
        return blockEnd > startDate && blockStart < endDate;
      });
    },
    createBlock: (input) => {
      const start = new Date(input.startTime);
      const end = new Date(input.endTime);
      if (end <= start) {
        return { error: 'end time must be after start time' };
      }

      const overlap = blocks.some((block) => {
        const blockStart = new Date(block.startTime);
        const blockEnd = new Date(block.endTime);
        return end > blockStart && start < blockEnd;
      });

      const created: TimeBlock = {
        id: generateId(),
        title: input.title,
        startTime: input.startTime,
        endTime: input.endTime,
        categoryId: input.categoryId,
        projectId: input.projectId,
        progress: input.progress ?? 0,
        syncStatus: 'local'
      };

      blocks = [...blocks, created];
      return {
        block: created,
        warning: overlap ? 'time overlap detected' : undefined
      };
    },
    deleteBlock: (id) => {
      const exists = blocks.some((block) => block.id === id);
      if (!exists) {
        return { error: 'time block not found' };
      }
      blocks = blocks.filter((block) => block.id !== id);
      return {};
    },
    resolveConflict: (mode, local, remote) => {
      if (mode === 'local') {
        return local;
      }
      return remote;
    }
  };
}

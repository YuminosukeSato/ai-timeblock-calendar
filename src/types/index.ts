export type SyncStatus =
  | 'local'
  | 'synced'
  | 'pending_push'
  | 'pending_delete'
  | 'conflict';

export interface TimeBlock {
  id: string;
  title: string;
  startTime: string;
  endTime: string;
  categoryId?: string;
  projectId?: string;
  progress: number;
  syncStatus: SyncStatus;
}

export interface TimeBlockInput {
  title: string;
  startTime: string;
  endTime: string;
  categoryId?: string;
  projectId?: string;
  progress?: number;
}

export type ConflictResolution = 'local' | 'remote';

export interface GanttTask {
  id: string;
  title: string;
  start: string;
  end: string;
  progress: number;
  dependsOn: string[];
}

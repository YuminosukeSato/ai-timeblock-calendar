export interface SyncStatusState {
  syncing: boolean;
  lastSyncedAt?: string;
}

export function useSync(): SyncStatusState {
  return { syncing: false };
}

import { ConflictResolution } from '../../types';

interface ConflictModalProps {
  isOpen: boolean;
  onResolve: (mode: ConflictResolution) => void;
}

export function ConflictModal({ isOpen, onResolve }: ConflictModalProps) {
  if (!isOpen) {
    return null;
  }

  return (
    <div role="dialog" aria-label="conflict-modal">
      <h2>Resolve Conflict</h2>
      <button onClick={() => onResolve('local')}>Use Local</button>
      <button onClick={() => onResolve('remote')}>Use Remote</button>
    </div>
  );
}

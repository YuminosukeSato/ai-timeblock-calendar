import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { ConflictModal } from './ConflictModal';

describe('ConflictModal', () => {
  it("calls resolveConflict('local') when local selected", () => {
    const onResolve = vi.fn();
    render(<ConflictModal isOpen onResolve={onResolve} />);

    fireEvent.click(screen.getByRole('button', { name: 'Use Local' }));

    expect(onResolve).toHaveBeenCalledWith('local');
  });

  it("calls resolveConflict('remote') when remote selected", () => {
    const onResolve = vi.fn();
    render(<ConflictModal isOpen onResolve={onResolve} />);

    fireEvent.click(screen.getByRole('button', { name: 'Use Remote' }));

    expect(onResolve).toHaveBeenCalledWith('remote');
  });
});

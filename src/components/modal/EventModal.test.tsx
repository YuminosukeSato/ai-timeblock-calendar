import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import { EventModal } from './EventModal';

describe('EventModal', () => {
  it('disables submit when required fields are missing', () => {
    render(<EventModal isOpen onSave={vi.fn()} onCancel={vi.fn()} />);

    const button = screen.getByRole('button', { name: 'Save' });
    expect(button).toBeDisabled();
  });

  it('shows validation error when end is before start', () => {
    render(<EventModal isOpen onSave={vi.fn()} onCancel={vi.fn()} />);

    fireEvent.change(screen.getByLabelText('title'), {
      target: { value: 'task' }
    });
    fireEvent.change(screen.getByLabelText('start'), {
      target: { value: '2026-02-17T10:00:00Z' }
    });
    fireEvent.change(screen.getByLabelText('end'), {
      target: { value: '2026-02-17T09:00:00Z' }
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save' }));

    expect(screen.getByRole('alert')).toHaveTextContent(
      'end time must be after start time'
    );
  });

  it('calls onSave with valid values', () => {
    const onSave = vi.fn();
    render(<EventModal isOpen onSave={onSave} onCancel={vi.fn()} />);

    fireEvent.change(screen.getByLabelText('title'), {
      target: { value: 'task' }
    });
    fireEvent.change(screen.getByLabelText('start'), {
      target: { value: '2026-02-17T09:00:00Z' }
    });
    fireEvent.change(screen.getByLabelText('end'), {
      target: { value: '2026-02-17T10:00:00Z' }
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save' }));

    expect(onSave).toHaveBeenCalledWith(
      expect.objectContaining({
        title: 'task',
        startTime: '2026-02-17T09:00:00Z',
        endTime: '2026-02-17T10:00:00Z'
      })
    );
  });

  it('shows existing values in edit mode', () => {
    render(
      <EventModal
        isOpen
        initialValues={{
          title: 'existing',
          startTime: '2026-02-17T09:00:00Z',
          endTime: '2026-02-17T10:00:00Z',
          progress: 50
        }}
        onSave={vi.fn()}
        onCancel={vi.fn()}
      />
    );

    expect(screen.getByLabelText('title')).toHaveValue('existing');
  });

  it('sets project_id when project is selected', () => {
    const onSave = vi.fn();
    render(
      <EventModal
        isOpen
        projects={[{ id: 'project-1', name: 'App Development' }]}
        onSave={onSave}
        onCancel={vi.fn()}
      />
    );

    fireEvent.change(screen.getByLabelText('title'), {
      target: { value: 'task' }
    });
    fireEvent.change(screen.getByLabelText('start'), {
      target: { value: '2026-02-17T09:00:00Z' }
    });
    fireEvent.change(screen.getByLabelText('end'), {
      target: { value: '2026-02-17T10:00:00Z' }
    });
    fireEvent.change(screen.getByLabelText('project'), {
      target: { value: 'project-1' }
    });
    fireEvent.click(screen.getByRole('button', { name: 'Save' }));

    expect(onSave).toHaveBeenCalledWith(
      expect.objectContaining({
        projectId: 'project-1'
      })
    );
  });
});

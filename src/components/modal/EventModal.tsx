import { FormEvent, useMemo, useState } from 'react';

export interface EventModalValues {
  title: string;
  startTime: string;
  endTime: string;
  categoryId?: string;
  projectId?: string;
  dependsOnId?: string;
  progress: number;
  description?: string;
}

interface Option {
  id: string;
  name: string;
}

interface EventModalProps {
  isOpen: boolean;
  initialValues?: EventModalValues;
  projects?: Option[];
  onSave: (values: EventModalValues) => void;
  onCancel: () => void;
}

const defaultValues: EventModalValues = {
  title: '',
  startTime: '',
  endTime: '',
  progress: 0,
  description: ''
};

export function EventModal({
  isOpen,
  initialValues,
  projects = [],
  onSave,
  onCancel
}: EventModalProps) {
  const [values, setValues] = useState<EventModalValues>(
    initialValues ?? defaultValues
  );
  const [error, setError] = useState<string | null>(null);

  const disabled = useMemo(() => {
    return (
      values.title.trim() === '' ||
      values.startTime === '' ||
      values.endTime === ''
    );
  }, [values]);

  if (!isOpen) {
    return null;
  }

  const submit = (event: FormEvent) => {
    event.preventDefault();
    if (new Date(values.endTime) <= new Date(values.startTime)) {
      setError('end time must be after start time');
      return;
    }
    setError(null);
    onSave(values);
  };

  return (
    <form aria-label="event-modal" onSubmit={submit}>
      <h2>Add Time Block</h2>
      <label>
        Title
        <input
          aria-label="title"
          value={values.title}
          onChange={(event) =>
            setValues({ ...values, title: event.target.value })
          }
        />
      </label>
      <label>
        Start
        <input
          aria-label="start"
          value={values.startTime}
          onChange={(event) =>
            setValues({ ...values, startTime: event.target.value })
          }
        />
      </label>
      <label>
        End
        <input
          aria-label="end"
          value={values.endTime}
          onChange={(event) =>
            setValues({ ...values, endTime: event.target.value })
          }
        />
      </label>
      <label>
        Project
        <select
          aria-label="project"
          value={values.projectId ?? ''}
          onChange={(event) =>
            setValues({
              ...values,
              projectId: event.target.value || undefined
            })
          }
        >
          <option value="">None</option>
          {projects.map((project) => (
            <option key={project.id} value={project.id}>
              {project.name}
            </option>
          ))}
        </select>
      </label>
      <button type="button" onClick={onCancel}>
        Cancel
      </button>
      <button type="submit" disabled={disabled}>
        Save
      </button>
      {error ? <p role="alert">{error}</p> : null}
    </form>
  );
}

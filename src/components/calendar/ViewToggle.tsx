interface ViewToggleProps {
  view: 'day' | 'week' | 'month' | 'gantt';
  onChange: (view: 'day' | 'week' | 'month' | 'gantt') => void;
}

export function ViewToggle({ view, onChange }: ViewToggleProps) {
  const labels: Record<'day' | 'week' | 'month' | 'gantt', string> = {
    day: 'Day',
    week: 'Week',
    month: 'Month',
    gantt: 'Gantt'
  };

  return (
    <div className="view-toggle">
      {(['day', 'week', 'month', 'gantt'] as const).map((mode) => (
        <button
          key={mode}
          className={view === mode ? 'is-active' : ''}
          onClick={() => onChange(mode)}
        >
          {labels[mode]}
        </button>
      ))}
    </div>
  );
}

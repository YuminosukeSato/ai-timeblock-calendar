import { useCallback, useMemo, useState } from 'react';

import { CalendarView } from './components/calendar/CalendarView';
import { ViewToggle } from './components/calendar/ViewToggle';
import { GanttView } from './components/gantt/GanttView';
import { EventModal, EventModalValues } from './components/modal/EventModal';
import { GanttTask, TimeBlock } from './types';
import { useTimeBlockStore } from './store/timeBlockStore';

const demoBlocks: TimeBlock[] = [
  {
    id: 'demo-1',
    title: 'Design Review',
    startTime: '2026-02-17T08:30:00Z',
    endTime: '2026-02-17T09:30:00Z',
    progress: 100,
    syncStatus: 'synced',
    categoryId: 'work'
  },
  {
    id: 'demo-2',
    title: 'Sprint Planning',
    startTime: '2026-02-17T10:00:00Z',
    endTime: '2026-02-17T11:00:00Z',
    progress: 60,
    syncStatus: 'synced',
    categoryId: 'work'
  },
  {
    id: 'demo-3',
    title: 'Implementation Block',
    startTime: '2026-02-18T12:30:00Z',
    endTime: '2026-02-18T15:00:00Z',
    progress: 35,
    syncStatus: 'local',
    categoryId: 'work'
  },
  {
    id: 'demo-4',
    title: 'Deep Work',
    startTime: '2026-02-20T04:00:00Z',
    endTime: '2026-02-20T06:00:00Z',
    progress: 0,
    syncStatus: 'local',
    categoryId: 'learning'
  },
  {
    id: 'demo-5',
    title: 'Lunch',
    startTime: '2026-02-19T03:30:00Z',
    endTime: '2026-02-19T04:00:00Z',
    progress: 0,
    syncStatus: 'synced',
    categoryId: 'personal'
  }
];

const demoGanttTasks: GanttTask[] = [
  {
    id: 'gantt-1',
    title: 'Design Phase',
    start: '2026-02-16',
    end: '2026-02-20',
    progress: 100,
    dependsOn: []
  },
  {
    id: 'gantt-2',
    title: 'Implementation',
    start: '2026-02-20',
    end: '2026-02-28',
    progress: 35,
    dependsOn: ['gantt-1']
  },
  {
    id: 'gantt-3',
    title: 'Testing',
    start: '2026-02-26',
    end: '2026-03-05',
    progress: 0,
    dependsOn: ['gantt-2']
  },
  {
    id: 'gantt-4',
    title: 'Documentation',
    start: '2026-03-01',
    end: '2026-03-07',
    progress: 0,
    dependsOn: ['gantt-2']
  },
  {
    id: 'gantt-5',
    title: 'Deploy',
    start: '2026-03-07',
    end: '2026-03-10',
    progress: 0,
    dependsOn: ['gantt-3', 'gantt-4']
  }
];

export default function App() {
  const [view, setView] = useState<'day' | 'week' | 'month' | 'gantt'>('week');
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingBlock, setEditingBlock] = useState<TimeBlock | null>(null);
  const blocks = useTimeBlockStore((state) => state.blocks);
  const createBlockOptimistic = useTimeBlockStore(
    (state) => state.createBlockOptimistic
  );
  const updateBlockOptimistic = useTimeBlockStore(
    (state) => state.updateBlockOptimistic
  );
  const displayBlocks = blocks.length > 0 ? blocks : demoBlocks;

  const ganttTasks = useMemo(() => demoGanttTasks, []);

  const handleNewBlock = useCallback(() => {
    setEditingBlock(null);
    setIsModalOpen(true);
  }, []);

  const handleSave = useCallback(
    (values: EventModalValues) => {
      if (editingBlock) {
        updateBlockOptimistic(editingBlock.id, {
          title: values.title,
          startTime: values.startTime,
          endTime: values.endTime,
          categoryId: values.categoryId,
          projectId: values.projectId,
          progress: values.progress
        });
      } else {
        const id = crypto.randomUUID();
        createBlockOptimistic({
          id,
          title: values.title,
          startTime: values.startTime,
          endTime: values.endTime,
          categoryId: values.categoryId,
          projectId: values.projectId,
          progress: values.progress,
          syncStatus: 'local'
        });
      }
      setIsModalOpen(false);
      setEditingBlock(null);
    },
    [editingBlock, createBlockOptimistic, updateBlockOptimistic]
  );

  const handleCancel = useCallback(() => {
    setIsModalOpen(false);
    setEditingBlock(null);
  }, []);

  return (
    <div className="notion-shell">
      <aside className="left-rail">
        <div className="brand-block">
          <p className="brand-kicker">AI Planner</p>
          <h1>Time Block Calendar</h1>
        </div>

        <section className="mini-month">
          <p className="section-title">February 2026</p>
          <div className="mini-grid">
            {['M', 'T', 'W', 'T', 'F', 'S', 'S'].map((label, i) => (
              <span key={i} className="mini-head">
                {label}
              </span>
            ))}
            {Array.from({ length: 28 }).map((_, idx) => (
              <span
                key={idx}
                className={idx === 16 ? 'mini-day is-today' : 'mini-day'}
              >
                {idx + 1}
              </span>
            ))}
          </div>
        </section>

        <section>
          <p className="section-title">Categories</p>
          <ul className="category-list">
            <li>
              <span className="dot work" />
              Work
            </li>
            <li>
              <span className="dot personal" />
              Personal
            </li>
            <li>
              <span className="dot learning" />
              Learning
            </li>
          </ul>
        </section>

        <section className="sync-card">
          <p className="section-title">Sync</p>
          <p>Google Connected</p>
          <p className="muted">last sync: 3 min ago</p>
        </section>
      </aside>

      <main className="center-panel">
        <header className="top-bar">
          <div>
            <p className="muted">Week View</p>
            <h2>Week 3 · February 2026</h2>
          </div>
          <div className="top-actions">
            <button className="ghost-btn">{'<'}</button>
            <button className="ghost-btn">{'>'}</button>
            <ViewToggle view={view} onChange={setView} />
            <button className="primary-btn" onClick={handleNewBlock}>
              + New Block
            </button>
          </div>
        </header>
        <section className="calendar-surface">
          {view === 'gantt' ? (
            <GanttView tasks={ganttTasks} viewMode="Week" />
          ) : (
            <CalendarView blocks={displayBlocks} view={view} />
          )}
        </section>
      </main>

      <aside className="right-rail">
        <section className="insight-card">
          <p className="section-title">Today</p>
          <h3>5 blocks</h3>
          <p className="muted">Focus time: 4h 30m</p>
        </section>
        <section className="insight-card">
          <p className="section-title">Projects</p>
          <div className="progress-row">
            <span>App Development</span>
            <strong>42%</strong>
          </div>
          <div className="progress-track">
            <span style={{ width: '42%' }} />
          </div>
          <div className="progress-row">
            <span>Paper Writing</span>
            <strong>18%</strong>
          </div>
          <div className="progress-track">
            <span style={{ width: '18%' }} />
          </div>
        </section>
        <section className="insight-card">
          <p className="section-title">Upcoming</p>
          <ul className="upcoming-list">
            <li>
              <span className="pill">09:00</span>
              Spec Review
            </li>
            <li>
              <span className="pill">13:00</span>
              Implementation Sprint
            </li>
            <li>
              <span className="pill">16:00</span>
              Sync Check
            </li>
          </ul>
        </section>
      </aside>

      <EventModal
        isOpen={isModalOpen}
        initialValues={
          editingBlock
            ? {
                title: editingBlock.title,
                startTime: editingBlock.startTime,
                endTime: editingBlock.endTime,
                categoryId: editingBlock.categoryId,
                projectId: editingBlock.projectId,
                progress: editingBlock.progress
              }
            : undefined
        }
        onSave={handleSave}
        onCancel={handleCancel}
      />
    </div>
  );
}

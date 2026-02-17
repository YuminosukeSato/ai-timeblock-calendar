import { useEffect, useState } from 'react';

import {
  createViewDay,
  createViewMonthGrid,
  createViewWeek
} from '@schedule-x/calendar';
import { createDragAndDropPlugin } from '@schedule-x/drag-and-drop';
import { createEventModalPlugin } from '@schedule-x/event-modal';
import { createEventsServicePlugin } from '@schedule-x/events-service';
import { ScheduleXCalendar, useCalendarApp } from '@schedule-x/react';
import { createResizePlugin } from '@schedule-x/resize';
import 'temporal-polyfill/global';
import '@schedule-x/theme-default/dist/index.css';

import {
  fromScheduleXEvent,
  toScheduleXEventsTemporal
} from '../../lib/calendarAdapter';
import { TimeBlock } from '../../types';

interface CalendarViewProps {
  blocks: TimeBlock[];
  view?: 'day' | 'week' | 'month';
  onEventUpdate?: (update: {
    id: string;
    title: string;
    startTime: string;
    endTime: string;
  }) => void;
  onEventClick?: (blockId: string) => void;
}

const CALENDARS = {
  work: {
    colorName: 'work',
    lightColors: {
      main: '#d8f3dc',
      container: '#d8f3dc',
      onContainer: '#1b4332'
    },
    darkColors: {
      main: '#2d6a4f',
      container: '#2d6a4f',
      onContainer: '#d8f3dc'
    }
  },
  personal: {
    colorName: 'personal',
    lightColors: {
      main: '#faedcd',
      container: '#faedcd',
      onContainer: '#7c2d12'
    },
    darkColors: {
      main: '#b45309',
      container: '#b45309',
      onContainer: '#faedcd'
    }
  },
  learning: {
    colorName: 'learning',
    lightColors: {
      main: '#dbeafe',
      container: '#dbeafe',
      onContainer: '#1e3a5f'
    },
    darkColors: {
      main: '#1d4ed8',
      container: '#1d4ed8',
      onContainer: '#dbeafe'
    }
  },
  default: {
    colorName: 'default',
    lightColors: {
      main: '#e5e7eb',
      container: '#e5e7eb',
      onContainer: '#374151'
    },
    darkColors: {
      main: '#4b5563',
      container: '#4b5563',
      onContainer: '#e5e7eb'
    }
  }
};

export function CalendarView({
  blocks,
  view = 'week',
  onEventUpdate,
  onEventClick
}: CalendarViewProps) {
  const [eventsService] = useState(() => createEventsServicePlugin());

  const calendar = useCalendarApp({
    views: [createViewDay(), createViewWeek(), createViewMonthGrid()],
    defaultView:
      view === 'day'
        ? createViewDay().name
        : view === 'month'
          ? createViewMonthGrid().name
          : createViewWeek().name,
    events: toScheduleXEventsTemporal(blocks),
    calendars: CALENDARS,
    plugins: [
      eventsService,
      createDragAndDropPlugin(),
      createResizePlugin(),
      createEventModalPlugin()
    ],
    callbacks: {
      onEventUpdate(updatedEvent) {
        if (onEventUpdate) {
          onEventUpdate(
            fromScheduleXEvent({
              id: updatedEvent.id,
              title: updatedEvent.title ?? '',
              start: String(updatedEvent.start),
              end: String(updatedEvent.end)
            })
          );
        }
      },
      onEventClick(event) {
        if (onEventClick) {
          onEventClick(String(event.id));
        }
      }
    }
  });

  useEffect(() => {
    const currentEvents = eventsService.getAll();
    const currentIds = new Set(currentEvents.map((e) => String(e.id)));
    const newIds = new Set(blocks.map((b) => b.id));

    for (const ce of currentEvents) {
      if (!newIds.has(String(ce.id))) {
        eventsService.remove(String(ce.id));
      }
    }

    const sxEvents = toScheduleXEventsTemporal(blocks);
    for (const sxe of sxEvents) {
      if (currentIds.has(sxe.id)) {
        eventsService.update(sxe);
      } else {
        eventsService.add(sxe);
      }
    }
  }, [blocks, eventsService]);

  return (
    <div className="calendar-view" data-testid="schedule-x-calendar">
      <ScheduleXCalendar calendarApp={calendar} />
    </div>
  );
}

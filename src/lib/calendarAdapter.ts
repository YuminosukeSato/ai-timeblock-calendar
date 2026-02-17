import 'temporal-polyfill/global';

import { TimeBlock } from '../types';

export interface ScheduleXEvent {
  id: string;
  title: string;
  start: string;
  end: string;
  calendarId: string;
  _customFields: {
    progress: number;
    projectId?: string;
    syncStatus: string;
  };
}

export function rfc3339ToScheduleXString(rfc: string): string {
  const d = new Date(rfc);
  const year = d.getUTCFullYear();
  const month = String(d.getUTCMonth() + 1).padStart(2, '0');
  const day = String(d.getUTCDate()).padStart(2, '0');
  const hours = String(d.getUTCHours()).padStart(2, '0');
  const minutes = String(d.getUTCMinutes()).padStart(2, '0');
  return `${year}-${month}-${day} ${hours}:${minutes}`;
}

function scheduleXToRfc3339(sx: string): string {
  const [datePart, timePart] = sx.split(' ');
  return `${datePart}T${timePart}:00Z`;
}

export function rfc3339ToTemporal(rfc: string): Temporal.ZonedDateTime {
  const d = new Date(rfc);
  return Temporal.Instant.fromEpochMilliseconds(d.getTime()).toZonedDateTimeISO(
    'UTC'
  );
}

export function toScheduleXEvent(block: TimeBlock): ScheduleXEvent {
  return {
    id: block.id,
    title: block.title,
    start: rfc3339ToScheduleXString(block.startTime),
    end: rfc3339ToScheduleXString(block.endTime),
    calendarId: block.categoryId ?? 'default',
    _customFields: {
      progress: block.progress,
      projectId: block.projectId,
      syncStatus: block.syncStatus
    }
  };
}

export interface ScheduleXEventTemporal {
  id: string;
  title: string;
  start: Temporal.ZonedDateTime;
  end: Temporal.ZonedDateTime;
  calendarId: string;
  _customFields: {
    progress: number;
    projectId?: string;
    syncStatus: string;
  };
}

export function toScheduleXEventTemporal(
  block: TimeBlock
): ScheduleXEventTemporal {
  return {
    id: block.id,
    title: block.title,
    start: rfc3339ToTemporal(block.startTime),
    end: rfc3339ToTemporal(block.endTime),
    calendarId: block.categoryId ?? 'default',
    _customFields: {
      progress: block.progress,
      projectId: block.projectId,
      syncStatus: block.syncStatus
    }
  };
}

export function toScheduleXEventsTemporal(
  blocks: TimeBlock[]
): ScheduleXEventTemporal[] {
  return blocks.map(toScheduleXEventTemporal);
}

export function toScheduleXEvents(blocks: TimeBlock[]): ScheduleXEvent[] {
  return blocks.map(toScheduleXEvent);
}

export function fromScheduleXEvent(event: {
  id: string | number;
  title: string;
  start: string;
  end: string;
}): { id: string; title: string; startTime: string; endTime: string } {
  return {
    id: String(event.id),
    title: event.title,
    startTime: scheduleXToRfc3339(event.start),
    endTime: scheduleXToRfc3339(event.end)
  };
}

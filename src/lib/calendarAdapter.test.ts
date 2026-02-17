import 'temporal-polyfill/global';

import { describe, expect, it } from 'vitest';

import { TimeBlock } from '../types';

import {
  toScheduleXEvent,
  toScheduleXEvents,
  toScheduleXEventTemporal,
  toScheduleXEventsTemporal,
  fromScheduleXEvent,
  rfc3339ToTemporal
} from './calendarAdapter';

describe('toScheduleXEvent', () => {
  const block: TimeBlock = {
    id: 'tb-1',
    title: 'Design Review',
    startTime: '2026-02-17T09:00:00Z',
    endTime: '2026-02-17T10:30:00Z',
    categoryId: 'work',
    projectId: 'proj-1',
    progress: 75,
    syncStatus: 'synced'
  };

  it('converts id and title directly', () => {
    const event = toScheduleXEvent(block);
    expect(event.id).toBe('tb-1');
    expect(event.title).toBe('Design Review');
  });

  it('converts RFC3339 start/end to YYYY-MM-DD HH:mm format', () => {
    const event = toScheduleXEvent(block);
    expect(event.start).toBe('2026-02-17 09:00');
    expect(event.end).toBe('2026-02-17 10:30');
  });

  it('maps categoryId to calendarId', () => {
    const event = toScheduleXEvent(block);
    expect(event.calendarId).toBe('work');
  });

  it('sets calendarId to "default" when categoryId is undefined', () => {
    const noCat: TimeBlock = { ...block, categoryId: undefined };
    const event = toScheduleXEvent(noCat);
    expect(event.calendarId).toBe('default');
  });

  it('preserves progress in _customFields', () => {
    const event = toScheduleXEvent(block);
    expect(event._customFields).toEqual({
      progress: 75,
      projectId: 'proj-1',
      syncStatus: 'synced'
    });
  });

  it('handles midnight boundary (00:00)', () => {
    const midnight: TimeBlock = {
      ...block,
      startTime: '2026-02-17T00:00:00Z',
      endTime: '2026-02-17T01:00:00Z'
    };
    const event = toScheduleXEvent(midnight);
    expect(event.start).toBe('2026-02-17 00:00');
    expect(event.end).toBe('2026-02-17 01:00');
  });

  it('handles end-of-day time (23:59)', () => {
    const lateNight: TimeBlock = {
      ...block,
      startTime: '2026-02-17T23:00:00Z',
      endTime: '2026-02-17T23:59:00Z'
    };
    const event = toScheduleXEvent(lateNight);
    expect(event.start).toBe('2026-02-17 23:00');
    expect(event.end).toBe('2026-02-17 23:59');
  });
});

describe('toScheduleXEvents', () => {
  it('converts empty array to empty array', () => {
    expect(toScheduleXEvents([])).toEqual([]);
  });

  it('converts multiple blocks preserving order', () => {
    const blocks: TimeBlock[] = [
      {
        id: 'a',
        title: 'A',
        startTime: '2026-02-17T09:00:00Z',
        endTime: '2026-02-17T10:00:00Z',
        progress: 0,
        syncStatus: 'local'
      },
      {
        id: 'b',
        title: 'B',
        startTime: '2026-02-18T14:00:00Z',
        endTime: '2026-02-18T15:00:00Z',
        progress: 50,
        syncStatus: 'synced'
      }
    ];
    const events = toScheduleXEvents(blocks);
    expect(events).toHaveLength(2);
    expect(events[0].id).toBe('a');
    expect(events[1].id).toBe('b');
  });
});

describe('fromScheduleXEvent', () => {
  it('converts schedule-x event back to TimeBlock partial', () => {
    const sxEvent = {
      id: 'tb-1',
      title: 'Updated Title',
      start: '2026-02-17 11:00',
      end: '2026-02-17 12:30'
    };
    const result = fromScheduleXEvent(sxEvent);
    expect(result.id).toBe('tb-1');
    expect(result.title).toBe('Updated Title');
    expect(result.startTime).toBe('2026-02-17T11:00:00Z');
    expect(result.endTime).toBe('2026-02-17T12:30:00Z');
  });

  it('handles single-digit hours', () => {
    const sxEvent = {
      id: 'x',
      title: 'Early',
      start: '2026-02-17 08:00',
      end: '2026-02-17 09:00'
    };
    const result = fromScheduleXEvent(sxEvent);
    expect(result.startTime).toBe('2026-02-17T08:00:00Z');
  });
});

describe('rfc3339ToTemporal', () => {
  it('converts RFC3339 to Temporal.ZonedDateTime', () => {
    const result = rfc3339ToTemporal('2026-02-17T09:00:00Z');
    expect(result).toBeInstanceOf(Temporal.ZonedDateTime);
    expect(result.hour).toBe(9);
    expect(result.minute).toBe(0);
    expect(result.year).toBe(2026);
    expect(result.month).toBe(2);
    expect(result.day).toBe(17);
  });

  it('handles midnight', () => {
    const result = rfc3339ToTemporal('2026-02-17T00:00:00Z');
    expect(result.hour).toBe(0);
    expect(result.minute).toBe(0);
  });
});

describe('toScheduleXEventTemporal', () => {
  const block: TimeBlock = {
    id: 'tb-1',
    title: 'Design Review',
    startTime: '2026-02-17T09:00:00Z',
    endTime: '2026-02-17T10:30:00Z',
    categoryId: 'work',
    projectId: 'proj-1',
    progress: 75,
    syncStatus: 'synced'
  };

  it('returns Temporal.ZonedDateTime for start and end', () => {
    const event = toScheduleXEventTemporal(block);
    expect(event.start).toBeInstanceOf(Temporal.ZonedDateTime);
    expect(event.end).toBeInstanceOf(Temporal.ZonedDateTime);
  });

  it('preserves time values in Temporal objects', () => {
    const event = toScheduleXEventTemporal(block);
    expect(event.start.hour).toBe(9);
    expect(event.start.minute).toBe(0);
    expect(event.end.hour).toBe(10);
    expect(event.end.minute).toBe(30);
  });

  it('maps id, title, calendarId correctly', () => {
    const event = toScheduleXEventTemporal(block);
    expect(event.id).toBe('tb-1');
    expect(event.title).toBe('Design Review');
    expect(event.calendarId).toBe('work');
  });
});

describe('toScheduleXEventsTemporal', () => {
  it('converts multiple blocks to Temporal events', () => {
    const blocks: TimeBlock[] = [
      {
        id: 'a',
        title: 'A',
        startTime: '2026-02-17T09:00:00Z',
        endTime: '2026-02-17T10:00:00Z',
        progress: 0,
        syncStatus: 'local'
      },
      {
        id: 'b',
        title: 'B',
        startTime: '2026-02-18T14:00:00Z',
        endTime: '2026-02-18T15:00:00Z',
        progress: 50,
        syncStatus: 'synced'
      }
    ];
    const events = toScheduleXEventsTemporal(blocks);
    expect(events).toHaveLength(2);
    expect(events[0].start).toBeInstanceOf(Temporal.ZonedDateTime);
    expect(events[1].start).toBeInstanceOf(Temporal.ZonedDateTime);
  });
});

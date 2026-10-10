import { z } from 'zod';
import type { Task } from '../today/model';
export type CalendarView = 'day' | 'week' | 'month' | 'agenda';
export const eventTypes = [
  'study',
  'class',
  'competition',
  'exam',
  'deadline',
  'paper',
  'personal',
] as const;
export interface EventInput {
  title: string;
  event_type: (typeof eventTypes)[number];
  event_date: string | null;
  starts_at: string | null;
  ends_at: string | null;
  task_id: string | null;
}
export interface CalendarEvent extends EventInput {
  id: string;
  revision: number;
}
export interface ActualInterval {
  id: string;
  session_id: string;
  task_id: string | null;
  title: string;
  local_date: string;
  starts_at: string;
  ends_at: string;
  elapsed_ms: number;
  state: string;
  provenance: string;
}
export interface CalendarSnapshot {
  from: string;
  until: string;
  events: CalendarEvent[];
  actual: ActualInterval[];
  tasks: Task[];
  task_options: Task[];
  trash: CalendarEvent[];
}
export const eventSchema = z
  .object({
    title: z.string().trim().min(1).max(250),
    event_type: z.enum(eventTypes),
    event_date: z.iso.date().nullable(),
    starts_at: z.iso.datetime().nullable(),
    ends_at: z.iso.datetime().nullable(),
    task_id: z.uuid().nullable(),
  })
  .refine((input) =>
    input.starts_at && input.ends_at
      ? input.event_date === null &&
        Date.parse(input.ends_at) > Date.parse(input.starts_at) &&
        Date.parse(input.ends_at) - Date.parse(input.starts_at) <= 7 * 86400000
      : input.starts_at === null && input.ends_at === null && input.event_date !== null,
  );
export function dateString(date: Date) {
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, '0'),
    String(date.getDate()).padStart(2, '0'),
  ].join('-');
}
export function addDays(day: string, count: number) {
  const d = new Date(day + 'T12:00:00');
  d.setDate(d.getDate() + count);
  return dateString(d);
}
export function range(day: string, view: CalendarView) {
  const d = new Date(day + 'T12:00:00');
  let from = day;
  let count = 1;
  if (view === 'month') {
    d.setDate(1);
    from = addDays(dateString(d), -((d.getDay() + 6) % 7));
    count = 42;
  }
  if (view === 'week') {
    from = addDays(day, -((d.getDay() + 6) % 7));
    count = 7;
  }
  if (view === 'agenda') count = 14;
  return {
    from,
    until: addDays(from, count),
    days: Array.from({ length: count }, (_, i) => addDays(from, i)),
  };
}
export function shift(day: string, view: CalendarView, step: number) {
  if (view !== 'month') return addDays(day, step * (view === 'day' ? 1 : view === 'week' ? 7 : 14));
  const d = new Date(day + 'T12:00:00');
  const target = d.getMonth() + step;
  d.setDate(1);
  d.setMonth(target);
  return dateString(d);
}
export function localDateTime(value: string) {
  const d = new Date(value);
  return (
    dateString(d) +
    'T' +
    String(d.getHours()).padStart(2, '0') +
    ':' +
    String(d.getMinutes()).padStart(2, '0')
  );
}
export function parseLocalDateTime(value: string) {
  if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/.test(value)) throw new RangeError('INVALID_LOCAL_TIME');
  const utc = new Date(value).toISOString();
  // Date normalizes impossible dates and skipped DST times; reject that change.
  if (localDateTime(utc) !== value) throw new RangeError('INVALID_LOCAL_TIME');
  return utc;
}
export function onDay(event: CalendarEvent, day: string) {
  return (
    event.event_date === day ||
    !!(
      event.starts_at &&
      event.ends_at &&
      Date.parse(event.starts_at) < new Date(addDays(day, 1) + 'T00:00:00').getTime() &&
      Date.parse(event.ends_at) > new Date(day + 'T00:00:00').getTime()
    )
  );
}
export function timePosition(start: string, end: string, day: string) {
  const first = new Date(day + 'T00:00:00').getTime(),
    last = new Date(addDays(day, 1) + 'T00:00:00').getTime();
  const a = Math.max(first, Date.parse(start)),
    b = Math.min(last, Date.parse(end));
  return {
    top: (100 * (a - first)) / (last - first),
    height: (100 * Math.max(0, b - a)) / (last - first),
  };
}

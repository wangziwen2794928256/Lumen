import { describe, it, expect } from 'vitest';
import {
  addDays,
  range,
  shift,
  eventSchema,
  onDay,
  timePosition,
  parseLocalDateTime,
} from '../../src/features/calendar/model';
describe('calendar local dates and intervals', () => {
  it('rejects normalized impossible dates and retains the entered local minute', () => {
    expect(() => parseLocalDateTime('2026-02-30T10:00')).toThrow();
    expect(() => parseLocalDateTime('2026-10-03')).toThrow();
    expect(new Date(parseLocalDateTime('2026-10-03T09:30')).getHours()).toBe(9);
  });
  it('keeps leap days, month boundaries and Monday weeks', () => {
    expect(addDays('2028-02-28', 1)).toBe('2028-02-29');
    expect(addDays('2026-12-31', 1)).toBe('2027-01-01');
    expect(range('2026-10-03', 'week').from).toBe('2026-09-28');
    expect(range('2026-10-03', 'month').days).toHaveLength(42);
    expect(shift('2026-01-31', 'month', 1)).toBe('2026-02-01');
  });
  it('requires exactly one valid date model and increasing bounded times', () => {
    const input = {
      title: '课程',
      event_type: 'class',
      event_date: null,
      starts_at: '2026-10-03T01:00:00.000Z',
      ends_at: '2026-10-03T02:00:00.000Z',
      task_id: null,
    };
    expect(eventSchema.safeParse(input).success).toBe(true);
    expect(eventSchema.safeParse({ ...input, ends_at: input.starts_at }).success).toBe(false);
    expect(eventSchema.safeParse({ ...input, event_date: '2026-10-03' }).success).toBe(false);
    expect(
      eventSchema.safeParse({ ...input, starts_at: null, ends_at: null, event_date: '2026-02-30' })
        .success,
    ).toBe(false);
  });
  it('clips timed overlap to the selected local day', () => {
    const start = new Date('2026-10-03T23:00:00').toISOString(),
      end = new Date('2026-10-04T01:00:00').toISOString();
    const event = {
      id: 'fixture',
      revision: 0,
      title: '跨日',
      event_type: 'study' as const,
      event_date: null,
      starts_at: start,
      ends_at: end,
      task_id: null,
    };
    expect(onDay(event, '2026-10-03')).toBe(true);
    expect(onDay(event, '2026-10-04')).toBe(true);
    expect(onDay(event, '2026-10-05')).toBe(false);
    expect(timePosition(start, end, '2026-10-04').top).toBe(0);
    expect(timePosition(start, end, '2026-10-04').height).toBeCloseTo(100 / 24);
  });
});

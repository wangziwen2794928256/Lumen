import { act, cleanup, render, screen } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import { MemoryRouter } from 'react-router-dom';
import { FortuneCard } from '../../src/features/fortune/FortuneCard';
import { dailyIndex, localDay } from '../../src/features/fortune/model';
import { fortuneText } from '../../src/features/fortune/text';
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});
it('uses a valid local calendar date with a stable finite sign, including leap days', () => {
  expect(localDay(new Date(2026, 9, 11, 23, 59))).toBe('2026-10-11');
  expect(dailyIndex('2024-02-29', 12)).toBe(dailyIndex('2024-02-29', 12));
  expect((dailyIndex('2026-10-11', 12) + 1) % 12).toBe(dailyIndex('2026-10-12', 12));
  for (const day of ['2026-02-29', '2026-02-30', '2026-13-01', 'invalid'])
    expect(() => dailyIndex(day, 12)).toThrow();
  expect(() => dailyIndex('2026-10-11', 0)).toThrow();
});
it('refreshes after local midnight and on returning from sleep without an external service', () => {
  vi.useFakeTimers();
  vi.setSystemTime(new Date(2026, 9, 11, 23, 59, 59));
  render(
    <MemoryRouter>
      <FortuneCard expanded />
    </MemoryRouter>,
  );
  expect(screen.getByText('2026-10-11')).toBeInTheDocument();
  act(() => {
    vi.advanceTimersByTime(1200);
  });
  expect(screen.getByText('2026-10-12')).toBeInTheDocument();
  vi.setSystemTime(new Date(2026, 9, 15, 10));
  act(() => {
    window.dispatchEvent(new Event('focus'));
  });
  expect(screen.getByText('2026-10-15')).toBeInTheDocument();
  expect(screen.getByText(fortuneText.signs[dailyIndex('2026-10-15', 12)][0])).toBeInTheDocument();
});

import { it, expect, vi } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { EntityForm } from '../../src/features/roadmap/EntityForm';
import {
  defaults,
  entitySchema,
  weekStart,
  taskOptionsSchema,
} from '../../src/features/roadmap/model';
it('validates allocation bounds, calendar dates and local week boundaries', () => {
  expect(
    entitySchema.safeParse({
      title: '目标',
      journey_id: null,
      data: { ...defaults, minimum_minutes: 80, preferred_minutes: 60 },
    }).success,
  ).toBe(false);
  expect(
    entitySchema.safeParse({
      title: '目标',
      journey_id: null,
      data: { ...defaults, start_date: '2026-02-30' },
    }).success,
  ).toBe(false);
  expect(
    taskOptionsSchema.safeParse({
      earliest_start: null,
      minimum_session: 30,
      preferred_session: 15,
      splittable: true,
      energy_level: 'normal',
      fixed_time: false,
    }).success,
  ).toBe(false);
  expect(weekStart('2026-10-04')).toBe('2026-09-28');
});
it('preserves a failed journey draft and request identity for retry', async () => {
  const act = vi.fn().mockResolvedValueOnce(false).mockResolvedValueOnce(true);
  render(<EntityForm kind="journey" entities={[]} busy={false} act={act} />);
  fireEvent.click(screen.getByRole('button', { name: '新建长期目标' }));
  fireEvent.change(screen.getByLabelText('名称'), { target: { value: '自己的长期目标' } });
  fireEvent.click(screen.getByRole('button', { name: '保存' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('草稿仍保留');
  expect(screen.getByLabelText('名称')).toHaveValue('自己的长期目标');
  fireEvent.click(screen.getByRole('button', { name: '保存' }));
  await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
  expect(act.mock.calls[0][0]).toEqual(act.mock.calls[1][0]);
});

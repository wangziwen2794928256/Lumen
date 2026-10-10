import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { it, expect, vi } from 'vitest';
import { EventForm } from '../../src/features/calendar/EventForm';
it('keeps a failed calendar draft and retries the same create request', async () => {
  const act = vi.fn().mockResolvedValueOnce(false).mockResolvedValueOnce(true);
  render(<EventForm day="2026-10-03" tasks={[]} disabled={false} busy={false} act={act} />);
  fireEvent.click(screen.getByRole('button', { name: '新建日历事件' }));
  fireEvent.change(screen.getByLabelText('事件标题'), { target: { value: '保留事件草稿' } });
  fireEvent.click(screen.getByLabelText('全天', { exact: true }));
  fireEvent.click(screen.getByRole('button', { name: '保存事件' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('未保存的输入已保留');
  expect(screen.getByLabelText('事件标题')).toHaveValue('保留事件草稿');
  fireEvent.click(screen.getByRole('button', { name: '保存事件' }));
  await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
  expect(act.mock.calls[0][0]).toEqual(act.mock.calls[1][0]);
});

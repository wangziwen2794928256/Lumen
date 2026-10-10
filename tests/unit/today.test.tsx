import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { TaskForm } from '../../src/features/today/TaskForm';
import { taskInputSchema } from '../../src/features/today/model';
describe('Today input and retry boundary', () => {
  it('rejects impossible dates, fractional durations and empty titles', () => {
    const input = {
      title: '任务',
      body: '',
      due_date: '2026-10-03',
      estimated_minutes: 25,
      priority: 1,
    };
    expect(taskInputSchema.safeParse(input).success).toBe(true);
    for (const change of [
      { title: ' ' },
      { due_date: '2026-02-30' },
      { estimated_minutes: 0 },
      { estimated_minutes: 1.5 },
      { priority: 4 },
    ])
      expect(taskInputSchema.safeParse({ ...input, ...change }).success).toBe(false);
  });
  it('preserves a failed draft and retries with the same request id', async () => {
    const act = vi.fn().mockResolvedValueOnce(false).mockResolvedValueOnce(true);
    render(<TaskForm disabled={false} busy={false} act={act} />);
    fireEvent.click(screen.getByRole('button', { name: '新建任务' }));
    fireEvent.change(screen.getByLabelText('任务标题'), { target: { value: '保留草稿' } });
    fireEvent.click(screen.getByRole('button', { name: '保存任务' }));
    expect(await screen.findByRole('alert')).toHaveTextContent('未保存的输入已保留');
    expect(screen.getByLabelText('任务标题')).toHaveValue('保留草稿');
    fireEvent.click(screen.getByRole('button', { name: '保存任务' }));
    await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
    expect(act).toHaveBeenCalledTimes(2);
    expect(act.mock.calls[0][0]).toEqual(act.mock.calls[1][0]);
  });
});

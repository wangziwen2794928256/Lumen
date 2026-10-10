import { it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { InsightsPage } from '../../src/pages/insights/InsightsPage';
import { ratio, recordHref } from '../../src/features/insights/model';
import { insightFacts } from '../fixtures/insights';
const { snapshot, review, action } = vi.hoisted(() => ({
  snapshot: vi.fn(),
  review: vi.fn(),
  action: vi.fn(),
}));
vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => true }));
vi.mock('../../src/features/insights/api', () => ({ insightsApi: { snapshot, review, action } }));
beforeEach(() => {
  vi.clearAllMocks();
  localStorage.clear();
  snapshot.mockResolvedValue(insightFacts());
  review.mockResolvedValue({
    week: '2026-09-28',
    facts: insightFacts(),
    revision: 0,
    body: '',
    next_focus: '',
  });
  action.mockResolvedValue(undefined);
});
it('Insights failed reflection retains exact retry request and restores pending draft', async () => {
  action.mockRejectedValueOnce(new Error('fixture failure'));
  const mount = () =>
    render(
      <MemoryRouter>
        <InsightsPage />
      </MemoryRouter>,
    );
  const first = mount();
  await screen.findByText('Journey 计划任务进度');
  fireEvent.click(screen.getByRole('button', { name: '周回顾' }));
  const field = await screen.findByLabelText('My Reflection');
  fireEvent.change(field, { target: { value: '我的真实心得' } });
  fireEvent.click(screen.getByRole('button', { name: '保存我的心得' }));
  await screen.findByRole('alert');
  const initial = action.mock.calls[0][0];
  expect(initial.body).toBe('我的真实心得');
  expect(screen.getByLabelText('My Reflection')).toHaveValue('我的真实心得');
  first.unmount();
  mount();
  await screen.findByText('Journey 计划任务进度');
  fireEvent.click(screen.getByRole('button', { name: '周回顾' }));
  await waitFor(() => expect(screen.getByLabelText('My Reflection')).toHaveValue('我的真实心得'));
  fireEvent.click(screen.getByRole('button', { name: '保存我的心得' }));
  await waitFor(() => expect(action).toHaveBeenCalledTimes(2));
  expect(action.mock.calls[1][0].request_id).toBe(initial.request_id);
});
it('Words link to exact journal record and music uses separate cloud', async () => {
  render(
    <MemoryRouter>
      <InsightsPage />
    </MemoryRouter>,
  );
  await screen.findByText('Journey 计划任务进度');
  fireEvent.click(screen.getByRole('button', { name: '本周词语' }));
  fireEvent.click(screen.getByRole('button', { name: '数学，2 次，查看来源' }));
  expect(screen.getByRole('link', { name: /隔离数学记录/ })).toHaveAttribute(
    'href',
    '/journal?entry=00000000-0000-4000-8000-000000000001',
  );
  fireEvent.click(screen.getByRole('button', { name: '音乐云' }));
  expect(screen.getByText('隔离歌曲')).toBeInTheDocument();
  expect(screen.queryByRole('button', { name: '数学，2 次，查看来源' })).not.toBeInTheDocument();
});
it('zero ratios and source destinations are explicit', () => {
  expect(ratio(0, 0)).toBe('暂无样本');
  expect(ratio(1, 2)).toBe('50%');
  expect(
    recordHref({ kind: 'review', date: '2026-09-28', id: 'fixture', title: 'fixture' }),
  ).toContain('week=2026-09-28');
});

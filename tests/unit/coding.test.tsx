import { it, expect, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { Editor } from '../../src/features/coding/Editor';
import { Stats } from '../../src/features/coding/Stats';
import {
  fresh,
  readDraft,
  firstPass,
  safeLink,
  draftKey,
  type Snapshot,
} from '../../src/features/coding/model';
const { action } = vi.hoisted(() => ({ action: vi.fn() }));
vi.mock('../../src/features/coding/api', () => ({ codingApi: { action } }));
const data: Snapshot = {
  problems: [],
  total: 0,
  attempts: [],
  contests: [],
  journeys: [],
  tasks: [],
  focus: [],
  analytics: {
    from: '2026-09-28',
    until: '2026-10-04',
    problems: 0,
    solved: 0,
    attempts: 0,
    first_pass_numerator: 0,
    first_pass_denominator: 0,
    manual_seconds: 0,
    focus_seconds: 0,
    contests: 0,
    difficulty: [],
    topics: [],
  },
};
it('retains duplicate input and request identity across restart and keeps other drafts', async () => {
  localStorage.clear();
  action.mockRejectedValue({ code: 'CODING_DUPLICATE' });
  const initial = fresh('problem');
  const other = fresh('contest');
  localStorage.setItem(draftKey(other), JSON.stringify(other));
  const view = render(
    <Editor initial={initial} data={data} onBack={() => {}} onSaved={() => {}} />,
  );
  fireEvent.change(screen.getByLabelText('名称'), { target: { value: '保留输入' } });
  fireEvent.change(screen.getByLabelText('平台题号'), { target: { value: '1' } });
  fireEvent.click(screen.getByRole('button', { name: '保存' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('平台与题号已存在');
  const sent = action.mock.calls[0][0];
  const stored = JSON.parse(localStorage.getItem(draftKey(initial))!);
  expect(stored.request_id).toBe(sent.request_id);
  expect(stored.input.title).toBe('保留输入');
  expect(localStorage.getItem(draftKey(other))).not.toBeNull();
  view.unmount();
  render(<Editor initial={stored} data={data} onBack={() => {}} onSaved={() => {}} />);
  action.mockResolvedValue(undefined);
  fireEvent.click(screen.getByRole('button', { name: '保存' }));
  await waitFor(() => expect(localStorage.getItem(draftKey(initial))).toBeNull());
  expect(action.mock.calls[1][0].request_id).toBe(sent.request_id);
  expect(readDraft()?.id).toBe(other.id);
});
it('shows zero denominators explicitly and blocks unsafe links', () => {
  render(<Stats value={data.analytics} />);
  expect(screen.getAllByText('暂无样本').length).toBeGreaterThan(1);
  expect(firstPass({ first_pass_numerator: 1, first_pass_denominator: 3 })).toBe('33.3% (1/3)');
  expect(safeLink('javascript:alert(1)')).toBeNull();
  expect(safeLink('https://user:pass@example.com')).toBeNull();
  expect(safeLink('https://example.com/problem')).toBe('https://example.com/problem');
});

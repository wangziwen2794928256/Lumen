import { it, expect, vi } from 'vitest';
import { act, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { Editor } from '../../src/features/journal/Editor';
import { fresh, pendingKey, readPending, type Entry } from '../../src/features/journal/model';
const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock('../../src/features/journal/api', () => ({ journalInvoke: invoke }));
it('keeps a failed save on disk, preserves markdown and blocks active HTML', async () => {
  localStorage.clear();
  invoke.mockRejectedValue({ code: 'JOURNAL_CONFLICT' });
  const initial = fresh();
  render(
    <Editor
      initial={initial}
      onBack={() => {}}
      onSaved={() => {}}
      onRecover={() => {}}
      onReload={() => {}}
    />,
  );
  fireEvent.change(screen.getByLabelText('Markdown 正文'), {
    target: { value: '# 日记\n<script>alert(1)</script>\n[危险](javascript:alert(1))' },
  });
  fireEvent.click(screen.getByRole('button', { name: '保存记录' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('草稿已保留');
  expect(readPending(false)?.body).toContain('# 日记');
  fireEvent.click(screen.getByRole('button', { name: '预览' }));
  expect(screen.getByRole('heading', { name: '日记' })).toBeInTheDocument();
  expect(document.querySelector('.journal-markdown script')).toBeNull();
  expect(screen.getByText('危险')).not.toHaveAttribute('href', 'javascript:alert(1)');
  expect(invoke.mock.calls[0][1].input.id).toBe(initial.id);
});
it('serializes a save and preserves newer edits until their own acknowledgement', async () => {
  localStorage.clear();
  let resolve: (value: Entry) => void = () => {};
  invoke
    .mockImplementationOnce(
      () =>
        new Promise<Entry>((r) => {
          resolve = r;
        }),
    )
    .mockImplementation(async (_cmd, args) =>
      _cmd === 'journal_attachments'
        ? []
        : {
            ...args.input,
            is_draft: false,
            revision: 2,
            deleted_at: null,
            updated_at: '',
          },
    );
  const initial = fresh();
  render(
    <Editor
      initial={initial}
      onBack={() => {}}
      onSaved={() => {}}
      onRecover={() => {}}
      onReload={() => {}}
    />,
  );
  fireEvent.change(screen.getByLabelText('Markdown 正文'), { target: { value: '第一版' } });
  fireEvent.click(screen.getByRole('button', { name: '保存记录' }));
  await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent('正在保存'));
  fireEvent.change(screen.getByLabelText('Markdown 正文'), {
    target: { value: '保存过程中继续写' },
  });
  await act(async () => {
    resolve({
      ...initial,
      body: '第一版',
      is_draft: false,
      revision: 1,
      deleted_at: null,
      updated_at: '',
    });
  });
  await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent('已自动保存到本机'));
  expect(screen.getByLabelText('Markdown 正文')).toHaveValue('保存过程中继续写');
  expect(localStorage.getItem(pendingKey(false, initial.client_id))).toBeNull();
  const last = invoke.mock.calls.filter((x) => x[0] === 'journal_save').at(-1);
  expect(last?.[1].input.body).toBe('保存过程中继续写');
  expect(last?.[1].input.revision).toBe(1);
});

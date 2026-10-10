import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { DatabaseDiagnostics } from '../../src/features/database/DatabaseDiagnostics';
import { verifyDatabase } from '../../src/lib/desktop';
vi.mock('../../src/lib/desktop', () => ({ verifyDatabase: vi.fn() }));
describe('database diagnostic', () => {
  it('displays actual successful schema status without private rows', async () => {
    vi.mocked(verifyDatabase).mockResolvedValue({
      schema_version: 1,
      integrity: 'ok',
      storage: 'AppData/lumen.db',
      task_count: 0,
      journal_count: 0,
    });
    render(<DatabaseDiagnostics />);
    fireEvent.click(screen.getByRole('button', { name: '检查本地数据库' }));
    await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent('数据库已就绪'));
    expect(screen.getByText(/结构版本 1/)).toBeVisible();
  });
  it('does not expose raw storage errors or personal content', async () => {
    vi.mocked(verifyDatabase).mockRejectedValue(new Error('private fixture SQL text'));
    render(<DatabaseDiagnostics />);
    fireEvent.click(screen.getByRole('button', { name: '检查本地数据库' }));
    await waitFor(() => expect(screen.getByRole('status')).toHaveTextContent('数据库暂不可用'));
    expect(screen.queryByText(/private fixture/)).toBeNull();
  });
});

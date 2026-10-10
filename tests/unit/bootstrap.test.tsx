import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { MemoryRouter } from 'react-router-dom';
import { AppRoutes } from '../../src/app/router';
import { verifyDesktop } from '../../src/lib/desktop';
vi.mock('../../src/lib/desktop', () => ({ verifyDesktop: vi.fn() }));
function renderApp() {
  return render(
    <MemoryRouter initialEntries={['/bootstrap']}>
      <AppRoutes />
    </MemoryRouter>,
  );
}
describe('bootstrap boundary', () => {
  it('routes to about without losing navigation', () => {
    renderApp();
    fireEvent.click(screen.getByRole('link', { name: '关于工程' }));
    expect(screen.getByRole('heading', { name: '本地优先，长期可读' })).toBeInTheDocument();
  });
  it('does not claim native connection in a browser', async () => {
    vi.mocked(verifyDesktop).mockResolvedValue(null);
    renderApp();
    fireEvent.click(screen.getByRole('button', { name: '验证桌面连接' }));
    expect(await screen.findByText(/当前为浏览器预览/)).toBeInTheDocument();
  });
  it('reports IPC failure without crashing', async () => {
    vi.mocked(verifyDesktop).mockRejectedValue(new Error('IPC unavailable'));
    renderApp();
    fireEvent.click(screen.getByRole('button', { name: '验证桌面连接' }));
    expect(await screen.findByText(/桌面连接失败/)).toBeInTheDocument();
  });
});

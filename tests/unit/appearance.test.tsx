import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { readAppearance, appearanceKey, defaults, saveAppearance } from '../../src/lib/appearance';
import { MemoryRouter } from 'react-router-dom';
import { AppRoutes } from '../../src/app/router';
describe('appearance boundary', () => {
  it('recovers invalid or corrupt preferences', () => {
    localStorage.setItem(appearanceKey, '{broken');
    expect(readAppearance()).toEqual(defaults);
    localStorage.setItem(
      appearanceKey,
      JSON.stringify({ mode: 'neon', accent: 'red', collapsed: 'yes' }),
    );
    expect(readAppearance()).toEqual(defaults);
  });
  it('keeps UI usable when storage throws', () => {
    const spy = vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
      throw new Error('QuotaExceeded');
    });
    expect(() => saveAppearance(defaults)).not.toThrow();
    spy.mockRestore();
  });
  it('persists selection and collapsed navigation keeps accessible names', () => {
    render(
      <MemoryRouter initialEntries={['/settings']}>
        <AppRoutes />
      </MemoryRouter>,
    );
    fireEvent.click(screen.getByRole('button', { name: '深色' }));
    expect(document.documentElement.dataset.theme).toBe('dark');
    fireEvent.click(screen.getByRole('button', { name: '折叠侧栏' }));
    expect(screen.getByRole('link', { name: '日历' })).toBeInTheDocument();
    expect(JSON.parse(localStorage.getItem(appearanceKey)!)).toMatchObject({
      mode: 'dark',
      collapsed: true,
    });
  });
});

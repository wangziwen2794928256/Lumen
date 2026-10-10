import { useEffect, useState, useSyncExternalStore, type ReactNode } from 'react';
import { readAppearance, saveAppearance, type Appearance } from '../../lib/appearance';
import { AppearanceContext } from './appearance-context';
function subscribe(callback: () => void) {
  const query = window.matchMedia('(prefers-color-scheme: dark)');
  query.addEventListener('change', callback);
  return () => query.removeEventListener('change', callback);
}
function getSystemDark() {
  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}
export function AppearanceProvider({ children }: { children: ReactNode }) {
  const [value, setValue] = useState(readAppearance);
  const systemDark = useSyncExternalStore(subscribe, getSystemDark, () => false);
  const resolved = value.mode === 'system' ? (systemDark ? 'dark' : 'light') : value.mode;
  useEffect(() => {
    document.documentElement.dataset.theme = resolved;
    document.documentElement.dataset.accent = value.accent;
    saveAppearance(value);
  }, [value, resolved]);
  return (
    <AppearanceContext.Provider
      value={{
        value,
        update: (next: Partial<Appearance>) => setValue((previous) => ({ ...previous, ...next })),
      }}
    >
      {children}
    </AppearanceContext.Provider>
  );
}

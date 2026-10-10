import { createContext, useContext } from 'react';
import type { Appearance } from '../../lib/appearance';
export const AppearanceContext = createContext<{
  value: Appearance;
  update: (next: Partial<Appearance>) => void;
} | null>(null);
export function useAppearance() {
  const context = useContext(AppearanceContext);
  if (!context) throw new Error('AppearanceProvider required');
  return context;
}

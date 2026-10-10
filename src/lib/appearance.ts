export type ThemeMode = 'light' | 'dark' | 'system';
export type Accent = 'sage' | 'blue' | 'lavender' | 'gray';
export interface Appearance {
  mode: ThemeMode;
  accent: Accent;
  collapsed: boolean;
}
export const appearanceKey = 'lumen.appearance.v1';
export const defaults: Appearance = { mode: 'system', accent: 'sage', collapsed: false };
export function readAppearance(): Appearance {
  try {
    const value = JSON.parse(localStorage.getItem(appearanceKey) || '{}');
    return {
      mode: ['light', 'dark', 'system'].includes(value?.mode) ? value.mode : defaults.mode,
      accent: ['sage', 'blue', 'lavender', 'gray'].includes(value?.accent)
        ? value.accent
        : defaults.accent,
      collapsed: typeof value?.collapsed === 'boolean' ? value.collapsed : false,
    };
  } catch {
    return { ...defaults };
  }
}
export function saveAppearance(value: Appearance) {
  try {
    localStorage.setItem(appearanceKey, JSON.stringify(value));
  } catch {
    /* Preferences may be unavailable; UI remains usable. */
  }
}

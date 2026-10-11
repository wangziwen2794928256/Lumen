import english from '../locales/en/messages.json' with { type: 'json' };

export type Locale = 'zh-CN' | 'en';
export const LOCALE_KEY = 'lumen.locale.v1';
export function readLocale(): Locale {
  try {
    return localStorage.getItem(LOCALE_KEY) === 'en' ? 'en' : 'zh-CN';
  } catch {
    return 'zh-CN';
  }
}
export const locale = readLocale();
export function translate(source: string, language: Locale = locale): string {
  return language === 'en' ? ((english as Record<string, string>)[source] ?? source) : source;
}
export function persistLocale(language: Locale): boolean {
  try {
    localStorage.setItem(LOCALE_KEY, language);
    return true;
  } catch {
    return false;
  }
}

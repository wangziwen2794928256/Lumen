import { afterEach, expect, test } from 'vitest';
import { LOCALE_KEY, persistLocale, readLocale, translate } from '../../src/lib/locale';
import english from '../../src/locales/en/messages.json' with { type: 'json' };
afterEach(() => localStorage.clear());
test('invalid or missing language preserves Chinese default; English persists', () => {
  expect(readLocale()).toBe('zh-CN');
  localStorage.setItem(LOCALE_KEY, 'unsupported');
  expect(readLocale()).toBe('zh-CN');
  expect(persistLocale('en')).toBe(true);
  expect(readLocale()).toBe('en');
});
test('English translates interface text, retaining unknown user-authored content', () => {
  expect(translate('新建任务', 'en')).toBe('New task');
  expect(translate('新建任务', 'zh-CN')).toBe('新建任务');
  expect(translate('我的私人研究目标', 'en')).toBe('我的私人研究目标');
  for (const value of Object.values(english)) {
    expect(value.trim().length).toBeGreaterThan(0);
    expect(value).not.toMatch(/\p{Script=Han}/u);
  }
});

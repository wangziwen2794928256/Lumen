import { test, expect } from '@playwright/test';
test('language selector applies English and persists after reload, then returns to Chinese', async ({
  page,
}) => {
  await page.goto('/#/settings');
  await page.getByLabel('界面语言').selectOption('en');
  await page.getByRole('button', { name: '应用语言并重新载入' }).click();
  await expect(page.locator('html')).toHaveAttribute('lang', 'en');
  await expect(page.getByRole('heading', { name: 'Module management', exact: true })).toBeVisible();
  await page.reload();
  await expect(page.getByRole('navigation', { name: 'Main navigation' })).toBeVisible();
  await page.getByLabel('Interface language').selectOption('zh-CN');
  await page.getByRole('button', { name: 'Apply language and reload' }).click();
  await expect(page.locator('html')).toHaveAttribute('lang', 'zh-CN');
  await expect(page.getByRole('heading', { name: '模块管理', exact: true })).toBeVisible();
});
test('English pages and accessible controls fit in light and dark themes', async ({
  page,
}, info) => {
  await page.addInitScript(() => localStorage.setItem('lumen.locale.v1', 'en'));
  for (const mode of ['Light', 'Dark']) {
    await page.goto('/#/settings');
    await page.getByRole('button', { name: mode, exact: true }).click();
    for (const route of [
      '/',
      '/focus',
      '/calendar',
      '/roadmap',
      '/journal',
      '/coding',
      '/insights',
      '/music',
      '/vault',
      '/profile',
      '/settings',
      '/design',
      '/about',
    ]) {
      await page.goto('/#' + route);
      await expect(page.locator('main')).toBeVisible();
      await expect
        .poll(() => page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))
        .toBe(true);
      const chinese = await page.locator('main').evaluate((el) => {
        const copy = el.cloneNode(true) as HTMLElement;
        copy.querySelectorAll('option').forEach((o) => o.remove());
        return /\p{Script=Han}/u.test(copy.textContent ?? '');
      });
      expect(chinese, route).toBe(false);
      await page.screenshot({
        path: `tests/screenshots/phase18-en-${route.replaceAll('/', '') || 'today'}-${mode}-${info.project.name}.png`,
        fullPage: true,
      });
    }
  }
});

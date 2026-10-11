import { test, expect } from '@playwright/test';
const routes = [
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
  '/fortune',
  '/settings',
  '/design',
  '/about',
];
test('Chinese full-page layout audit, both themes and persistent shell navigation', async ({
  page,
}, info) => {
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(e.message));
  for (const mode of ['浅色', '深色']) {
    await page.goto('/#/settings');
    await page.getByRole('button', { name: mode, exact: true }).click();
    for (const route of routes) {
      await page.goto('/#' + route);
      await expect(page.locator('main h1')).toBeVisible();
      await expect(page.locator('.route-loading')).toHaveCount(0);
      const overflow = await page.locator('main').evaluate((el) =>
        [...el.querySelectorAll('button,input,textarea,select,a')]
          .filter((e) => {
            const r = e.getBoundingClientRect();
            return r.width > 0 && (r.left < -1 || r.right > innerWidth + 1);
          })
          .map(
            (e) =>
              e.tagName + ':' + (e.getAttribute('aria-label') ?? e.textContent ?? '').slice(0, 50),
          ),
      );
      expect(overflow, `${mode} ${route}: horizontal control clipping`).toEqual([]);
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(
        true,
      );
      await page.screenshot({
        path: `tests/screenshots/phase22-zh-${route.replaceAll('/', '') || 'today'}-${mode}-${info.project.name}.png`,
        fullPage: true,
      });
    }
    const nav = page.getByRole('navigation', { name: '主要导航' });
    const shell = await page.locator('.sidebar').elementHandle();
    for (const route of ['/fortune', '/journal', '/music', '/']) {
      await nav.locator(`a[href="#${route}"]`).click();
      await expect(page.locator('main h1')).toBeVisible();
      expect(await shell!.evaluate((e) => e.isConnected)).toBe(true);
    }
    // Settings remains reachable in the short 720px window after adding a module.
    await page.getByRole('link', { name: '设置', exact: true }).click();
    await expect(page.locator('main h1')).toHaveText('设置');
  }
  expect(errors).toEqual([]);
});

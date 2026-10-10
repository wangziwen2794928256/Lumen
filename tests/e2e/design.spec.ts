import { test, expect } from '@playwright/test';
test('themes, sidebar persistence, dialog keyboard and screenshots', async ({ page }, info) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: '跳至主要内容' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.getByRole('main')).toBeFocused();
  expect(new URL(page.url()).hash).not.toContain('main-content');
  await expect(
    page.getByText('浏览器仅展示空状态。请在 Lumen 桌面应用中创建和保存任务。', { exact: true }),
  ).toBeVisible();
  for (const [mode, label] of [
    ['light', '浅色'],
    ['dark', '深色'],
  ]) {
    await page.getByRole('link', { name: '设置', exact: true }).click();
    await page.getByRole('button', { name: label, exact: true }).click();
    await expect(page.locator('html')).toHaveAttribute('data-theme', mode);
    for (const accent of ['鼠尾草', '雾蓝', '薰衣草', '暖灰']) {
      await page.getByRole('button', { name: accent, exact: true }).click();
      const ratios = await page.evaluate(() => {
        const cs = getComputedStyle(document.documentElement);
        const luminance = (hex: string) => {
          const colors = hex
            .trim()
            .slice(1)
            .match(/.{2}/g)!
            .map((c) => parseInt(c, 16) / 255)
            .map((c) => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4));
          return colors[0] * 0.2126 + colors[1] * 0.7152 + colors[2] * 0.0722;
        };
        const ratio = (a: string, b: string) => {
          const x = luminance(cs.getPropertyValue(a)),
            y = luminance(cs.getPropertyValue(b));
          return (Math.max(x, y) + 0.05) / (Math.min(x, y) + 0.05);
        };
        return ['--text-primary', '--text-secondary', '--text-muted']
          .map((token) => ratio(token, '--background'))
          .concat(ratio('--on-accent', '--accent-primary'), ratio('--on-accent', '--danger'));
      });
      for (const ratio of ratios) expect(ratio).toBeGreaterThanOrEqual(4.5);
    }
    await page.getByRole('button', { name: '鼠尾草', exact: true }).click();
    await page.getByRole('link', { name: '今日', exact: true }).click();
    await expect
      .poll(() => page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))
      .toBe(true);
    await page.screenshot({
      path: 'tests/screenshots/phase10-today-' + mode + '-' + info.project.name + '.png',
      fullPage: true,
    });
    await page.getByRole('link', { name: '组件展台', exact: true }).click();
    await expect(page.getByRole('button', { name: '鼠尾草', exact: true })).toHaveCount(0);
    await expect(page.getByRole('link', { name: '前往外观设置' })).toBeVisible();
    await page.screenshot({
      path: 'tests/screenshots/phase10-library-' + mode + '-' + info.project.name + '.png',
      fullPage: true,
    });
    const trigger = page.getByRole('button', { name: '打开对话框示例' });
    await trigger.focus();
    await page.keyboard.press('Enter');
    const modal = page.getByRole('dialog');
    await expect(modal).toBeVisible();
    await expect(modal.getByLabel('标题', { exact: true })).toBeFocused();
    for (let i = 0; i < 7; i++) {
      await page.keyboard.press('Tab');
      expect(await modal.evaluate((node) => node.contains(document.activeElement))).toBe(true);
    }
    await page.screenshot({
      path: 'tests/screenshots/phase10-dialog-' + mode + '-' + info.project.name + '.png',
    });
    await page.keyboard.press('Escape');
    await expect(modal).not.toBeVisible();
    await expect(trigger).toBeFocused();
    await page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));
    await page.getByRole('link', { name: '今日', exact: true }).click();
    await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(0);
  }
  await page.getByRole('button', { name: '折叠侧栏' }).click();
  await page.reload();
  await expect(page.getByRole('button', { name: '展开侧栏' })).toBeVisible();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await page.getByRole('link', { name: '设置', exact: true }).click();
  await page.getByRole('button', { name: '跟随系统' }).click();
  await page.emulateMedia({ colorScheme: 'light', reducedMotion: 'reduce' });
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  await page.emulateMedia({ colorScheme: 'dark' });
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  const transition = await page
    .getByRole('button', { name: '浅色', exact: true })
    .evaluate((el) => getComputedStyle(el).transitionDuration);
  expect(transition).toBe('0s');
});

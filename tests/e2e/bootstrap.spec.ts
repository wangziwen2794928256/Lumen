import { test, expect } from '@playwright/test';
test('bootstrap navigation and honest browser status', async ({ page }, info) => {
  await page.goto('/#/bootstrap');
  await expect(page.getByRole('heading', { name: '为长期成长，留一盏灯。' })).toBeVisible();
  await page.getByRole('button', { name: '验证桌面连接' }).click();
  await expect(page.getByRole('status', { name: '桌面连接状态' })).toContainText('浏览器预览');
  await expect
    .poll(() => page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))
    .toBe(true);
  await page.screenshot({
    path: 'tests/screenshots/phase10-bootstrap-' + info.project.name + '.png',
    fullPage: true,
  });
  await page.getByRole('link', { name: '关于工程' }).click();
  await expect(page.getByRole('heading', { name: '本地优先，长期可读' })).toBeVisible();
  await page.getByRole('link', { name: '基础窗口' }).click();
  await expect(page.getByRole('heading', { name: '基础工程已就绪' })).toBeVisible();
});

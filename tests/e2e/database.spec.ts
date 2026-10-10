import { test, expect } from '@playwright/test';
test('database diagnostic is honest in browser and appearance has one editing entry', async ({
  page,
}, info) => {
  await page.goto('/#/bootstrap');
  await page.getByRole('button', { name: '检查本地数据库' }).click();
  await expect(page.getByText('仅桌面环境可读取本地数据库。')).toBeVisible();
  await page.screenshot({
    path: 'tests/screenshots/phase10-diagnostics-' + info.project.name + '.png',
    fullPage: true,
  });
  await page.getByRole('link', { name: '设置', exact: true }).click();
  await page.getByRole('button', { name: '雾蓝', exact: true }).click();
  await page.getByRole('link', { name: '组件展台', exact: true }).click();
  await expect(page.locator('html')).toHaveAttribute('data-accent', 'blue');
  await expect(page.getByRole('button', { name: '雾蓝', exact: true })).toHaveCount(0);
  await page.getByRole('link', { name: '前往外观设置' }).click();
  await expect(page.getByRole('button', { name: '雾蓝', exact: true })).toHaveAttribute(
    'aria-pressed',
    'true',
  );
});

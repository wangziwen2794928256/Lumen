import { test, expect } from '@playwright/test';
test('fortune is discoverable, stable and optional; hidden differs from disabled', async ({
  page,
}) => {
  await page.goto('/#/');
  await expect(page.getByRole('heading', { name: '每日运势', exact: true })).toBeVisible();
  await page.getByRole('link', { name: '看看今日签 →' }).click();
  await expect(page.getByRole('heading', { name: '每日运势', level: 1 })).toBeVisible();
  const sign = await page.locator('.fortune-sign').innerText();
  await page.reload();
  await expect(page.locator('.fortune-sign')).toHaveText(sign);
  await page.goto('/#/settings');
  await page.getByRole('checkbox', { name: '每日运势 隐藏' }).check();
  await page.getByRole('button', { name: '保存模块配置' }).click();
  await expect(
    page.getByRole('navigation').getByRole('link', { name: '每日运势', exact: true }),
  ).toHaveCount(0);
  await page.goto('/#/');
  await expect(page.locator('.fortune-card')).toBeVisible();
  await page.goto('/#/settings');
  await page.getByRole('checkbox', { name: '每日运势 启用' }).uncheck();
  await page.getByRole('button', { name: '保存模块配置' }).click();
  await page.goto('/#/');
  await expect(page.locator('.fortune-card')).toHaveCount(0);
  await page.goto('/#/fortune');
  await expect(page.getByRole('status')).toContainText('已停用');
  await page.reload();
  await expect(page.getByRole('status')).toContainText('已停用');
});
test('module action buttons have a usable gap', async ({ page }) => {
  await page.goto('/#/settings');
  const save = await page.getByRole('button', { name: '保存模块配置' }).boundingBox();
  const reload = await page
    .getByRole('button', { name: '重新载入模块配置', exact: true })
    .boundingBox();
  expect(save).toBeTruthy();
  expect(reload).toBeTruthy();
  const gap =
    reload!.y >= save!.y + save!.height
      ? reload!.y - save!.y - save!.height
      : reload!.x - save!.x - save!.width;
  expect(gap).toBeGreaterThanOrEqual(12);
});

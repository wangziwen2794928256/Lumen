import { test, expect } from '@playwright/test';
test('module enabled hidden pinned and drag ordering persist without losing official records', async ({
  page,
}, info) => {
  await page.goto('/#/settings');
  await expect(page.getByRole('heading', { name: '模块管理' })).toBeVisible();
  await expect(page.getByRole('checkbox', { name: '今日 启用' })).toBeDisabled();
  await page.getByRole('checkbox', { name: '洞察 置顶' }).check();
  await page.getByRole('checkbox', { name: '个人档案 隐藏' }).check();
  await page.getByRole('button', { name: '保存模块配置' }).click();
  const nav = page.getByRole('navigation', { name: '主要导航' });
  await expect(nav.getByRole('link').first()).toHaveAttribute('href', '#/insights');
  await expect(nav.getByRole('link', { name: '个人档案', exact: true })).toHaveCount(0);
  await page.getByRole('checkbox', { name: '洞察 启用' }).uncheck();
  await page.getByRole('button', { name: '保存模块配置' }).click();
  await page.goto('/#/insights');
  await expect(page.getByRole('status')).toContainText('已停用');
  await page.reload();
  await expect(page.getByRole('status')).toContainText('已停用');
  await page.goto('/#/settings');
  await page.getByRole('checkbox', { name: '洞察 启用' }).check();
  await page.getByRole('checkbox', { name: '洞察 置顶' }).uncheck();
  await page.getByRole('button', { name: '保存模块配置' }).click();
  await nav
    .getByRole('link', { name: '洞察', exact: true })
    .dragTo(nav.getByRole('link', { name: '专注', exact: true }));
  await expect(nav.getByRole('link').nth(1)).toHaveText('洞察');
  await page.reload();
  await expect(nav.getByRole('link').nth(1)).toHaveText('洞察');
  await page.getByRole('button', { name: '洞察 下移' }).focus();
  await page.keyboard.press('Enter');
  await page.getByRole('button', { name: '保存模块配置' }).click();
  await expect(nav.getByRole('link').nth(1)).toHaveText('专注');
  for (const mode of ['浅色', '深色']) {
    await page.getByRole('button', { name: mode, exact: true }).click();
    await expect
      .poll(() => page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))
      .toBe(true);
    await page.getByRole('heading', { name: '模块管理' }).scrollIntoViewIfNeeded();
    await page.screenshot({
      path: `tests/screenshots/phase10-modules-${mode}-${info.project.name}.png`,
      fullPage: true,
    });
  }
});

import { test, expect, type Page } from '@playwright/test';
import { installModuleRegistryFixture } from '../fixtures/module-registry';
async function fixture(page: Page, onboarding = false) {
  await page.addInitScript(
    ({ onboarding }) => {
      Object.assign(window, { isTauri: true });
      const preferences = {
        close_to_tray: false,
        notifications_enabled: false,
        focus_finished: true,
        upcoming_event: true,
        weekly_review: true,
        backup_failure: true,
        revision: 0,
      };
      const host = window as unknown as {
        __TAURI_INTERNALS__: {
          invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;
        };
        desktopCalls: string[];
      };
      host.desktopCalls = [];
      host.__TAURI_INTERNALS__ = {
        invoke: async (cmd, args) => {
          host.desktopCalls.push(cmd);
          if (cmd === 'deep_link_initial') return null;
          if (cmd === 'desktop_status')
            return {
              preferences: JSON.parse(
                localStorage.getItem('desktop.preferences') ?? JSON.stringify(preferences),
              ),
              autostart: false,
              notification_permission: 'Granted',
              runtime: { tray_ready: true, error_code: null },
              onboarding: onboarding && !localStorage.getItem('onboarding'),
            };
          if (cmd === 'desktop_preferences_save') {
            const input = args?.preferences as typeof preferences;
            const current = JSON.parse(
              localStorage.getItem('desktop.preferences') ?? JSON.stringify(preferences),
            );
            if (current.revision !== input.revision) throw Error('STATE_CONFLICT');
            const saved = { ...input, revision: input.revision + 1 };
            localStorage.setItem('desktop.preferences', JSON.stringify(saved));
            return saved;
          }
          if (cmd === 'desktop_autostart') throw Error('AUTOSTART_UNAVAILABLE');
          if (cmd === 'desktop_test_notification') throw Error('NOTIFICATION_DENIED');
          if (cmd === 'onboarding_complete') {
            localStorage.setItem('onboarding', JSON.stringify(args?.input));
            return null;
          }
          if (cmd === 'onboarding_skip') {
            localStorage.setItem('onboarding', 'skipped');
            return null;
          }
          if (cmd === 'capture_shortcut') throw Error('SHORTCUT_CONFLICT');
          throw Error('Unsupported isolated command');
        },
      };
    },
    { onboarding },
  );
  await installModuleRegistryFixture(page);
}
test('Desktop settings persist opt-in values and tolerate OS failures', async ({ page }, info) => {
  await fixture(page);
  await page.goto('/#/settings');
  const card = page.locator('.desktop-settings');
  await expect(card.getByLabel('启用系统通知', { exact: true })).not.toBeChecked();
  await expect(card.getByLabel('开机时启动 Lumen')).not.toBeChecked();
  expect(
    await page.evaluate(() =>
      (window as unknown as { desktopCalls: string[] }).desktopCalls.filter(
        (c) => c === 'desktop_autostart' || c === 'desktop_test_notification',
      ),
    ),
  ).toEqual([]);
  await card.getByLabel('关闭主窗口时留在托盘').check();
  await card.getByRole('button', { name: '保存桌面设置' }).click();
  await expect(card.getByRole('status')).toContainText('已保存');
  await page.reload();
  await expect(card.getByLabel('关闭主窗口时留在托盘')).toBeChecked();
  await card.getByLabel('开机时启动 Lumen').click();
  await expect(card.getByRole('alert')).toBeVisible();
  await expect(card.getByLabel('开机时启动 Lumen')).not.toBeChecked();
  await card.getByLabel('启用系统通知', { exact: true }).check();
  await card.getByRole('button', { name: '保存桌面设置' }).click();
  await card.getByRole('button', { name: '发送通用测试通知' }).click();
  await expect(card.getByRole('alert')).toBeVisible();
  const nav = page.getByRole('navigation', { name: '设置分组' });
  await expect(nav.getByRole('button')).toHaveCount(13);
  for (const theme of ['light', 'dark']) {
    await page.evaluate((t) => (document.documentElement.dataset.theme = t), theme);
    await nav.getByRole('button', { name: '通用', exact: true }).click();
    await expect(page.getByRole('heading', { name: '通用 / General', exact: true })).toBeFocused();
    expect(
      await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
    ).toBeTruthy();
    await page.screenshot({
      path: `tests/screenshots/phase14-settings-${theme}-${info.project.name}.png`,
      fullPage: true,
    });
  }
});
test('Five-step onboarding saves only manual input and survives shortcut conflict', async ({
  page,
}, info) => {
  await fixture(page, true);
  await page.goto('/#/settings');
  const dialog = page.getByRole('dialog', { name: '欢迎使用 Lumen' });
  await expect(dialog).toBeVisible();
  await dialog.getByLabel('你的主要目标（可留空）').fill('Isolated chosen journey');
  await dialog.getByRole('button', { name: '下一步' }).click();
  await dialog.getByLabel('工作日容量（分钟）').fill('1441');
  await expect(dialog.getByRole('button', { name: '下一步' })).toBeDisabled();
  await dialog.getByLabel('工作日容量（分钟）').fill('120');
  await dialog.getByRole('button', { name: '下一步' }).click();
  await dialog.getByLabel('科目（每行一个，可留空）').fill('Chosen subject');
  await dialog.getByRole('button', { name: '下一步' }).click();
  await dialog.getByLabel('快捷键（可选，留空保留默认）').fill('Ctrl+Shift+Space');
  await dialog.getByRole('button', { name: '下一步' }).click();
  await expect(dialog).toContainText('步骤 5 / 5');
  await page.screenshot({ path: `tests/screenshots/phase14-onboarding-${info.project.name}.png` });
  await dialog.getByRole('button', { name: '确认并进入 Lumen' }).click();
  await expect(dialog.getByRole('alert')).toContainText('快捷键未更新');
  expect(await page.evaluate(() => JSON.parse(localStorage.getItem('onboarding')!))).toEqual({
    journey: 'Isolated chosen journey',
    subjects: ['Chosen subject'],
    weekday_minutes: 120,
    weekend_minutes: 210,
  });
  await dialog.getByRole('button', { name: '确认并进入 Lumen' }).click();
  await expect(dialog).not.toBeVisible();
  expect(
    await page.evaluate(
      () =>
        (window as unknown as { desktopCalls: string[] }).desktopCalls.filter(
          (c) => c === 'onboarding_complete',
        ).length,
    ),
  ).toBe(1);
  await page.reload();
  await expect(dialog).not.toBeVisible();
});
test('Onboarding skip is persistent and does not submit a plan', async ({ page }) => {
  await fixture(page, true);
  await page.goto('/#/settings');
  await page.getByRole('button', { name: '跳过引导' }).click();
  await expect(page.getByRole('dialog')).not.toBeVisible();
  await page.reload();
  await expect(page.getByRole('dialog')).not.toBeVisible();
  expect(
    await page.evaluate(() =>
      (window as unknown as { desktopCalls: string[] }).desktopCalls.includes(
        'onboarding_complete',
      ),
    ),
  ).toBeFalsy();
});

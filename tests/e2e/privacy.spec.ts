import { test, expect } from '@playwright/test';
import { installModuleRegistryFixture } from '../fixtures/module-registry';
test('Backup policy bounds persistence manual snapshot and themes', async ({ page }, info) => {
  await page.addInitScript(() => {
    Object.assign(window, { isTauri: true });
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string, args: { policy: unknown }) => {
        if (command === 'deep_link_initial') return null;
        if (command === 'backup_status')
          return {
            policy: JSON.parse(
              localStorage.getItem('fixture.backup') ??
                '{"enabled":true,"daily":7,"weekly":4,"monthly":6}',
            ),
            scheduler: { last_success: null, error_code: null },
          };
        if (command === 'backup_policy_save') {
          localStorage.setItem('fixture.backup', JSON.stringify(args.policy));
          return null;
        }
        if (command === 'backup_create') return 'manual-isolated-fixture';
        throw Error('Unsupported isolated command');
      },
    };
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/settings');
  const backup = page.locator('.backup-settings:not(.privacy-center)');
  await expect(backup.getByRole('checkbox')).toBeChecked();
  const save = backup.getByRole('button', { name: '保存备份策略' });
  await backup.getByLabel('每日保留份数（1–90）').fill('0');
  await expect(save).toBeDisabled();
  await backup.getByLabel('每日保留份数（1–90）').fill('8');
  await backup.getByRole('checkbox').uncheck();
  await save.click();
  await expect(backup.getByRole('status')).toContainText('已保存');
  await page.reload();
  await expect(backup.getByRole('checkbox')).not.toBeChecked();
  await expect(backup.getByLabel('每日保留份数（1–90）')).toHaveValue('8');
  await backup.getByRole('button', { name: '立即创建完整备份' }).click();
  await expect(backup.getByRole('status')).toContainText('manual-isolated-fixture');
  for (const theme of ['light', 'dark'] as const) {
    await page.evaluate((t) => (document.documentElement.dataset.theme = t), theme);
    await backup.scrollIntoViewIfNeeded();
    await expect(backup.getByRole('button', { name: '保存备份策略' })).toBeVisible();
    expect(
      await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
    ).toBeTruthy();
    await page.screenshot({
      path: `tests/screenshots/phase13-backup-${theme}-${info.project.name}.png`,
      fullPage: true,
    });
  }
});

test('Privacy Center explicit exports import review restore and disconnect', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    Object.assign(window, { isTauri: true });
    let enabled = true;
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string) => {
        if (command === 'deep_link_initial') return null;
        if (command === 'privacy_status')
          return {
            data_directory: 'isolated-fixture/AppData',
            activity_tracking: false,
            windows_media_enabled: enabled,
            integrations: 0,
            credential_references: 0,
            network_upload: false,
          };
        if (command === 'privacy_disable') {
          enabled = false;
          return null;
        }
        if (command === 'privacy_open_folder') return null;
        if (command === 'private_export') return 'isolated-fixture/private-export';
        if (command === 'import_review')
          return {
            token: 'isolated',
            review: {
              add: 2,
              update: 1,
              conflicts: 0,
              invalid: 0,
              unchanged: 0,
              fingerprint: 'fixture',
            },
          };
        if (command === 'import_confirm') return {};
        if (command === 'restore_rehearse')
          return {
            candidate: 'isolated-fixture/restore',
            schema: 10,
            integrity: 'ok',
            sample_rows: 3,
            tables: 45,
            replaces_live_database: false,
          };
        throw Error('Unsupported isolated command');
      },
    };
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/settings');
  const center = page.locator('.privacy-center');
  await expect(center.getByRole('heading', { name: '隐私中心', exact: true })).toBeVisible();
  const remove = center.getByRole('button', { name: '移入回收站：集成采集的音乐记录' });
  await expect(remove).toBeDisabled();
  await center.getByRole('button', { name: '关闭监听并断开集成' }).click();
  await expect(center.getByRole('status')).toContainText('监听已关闭');
  for (const name of ['导出 JSON 数据备份', '导出 CSV', '导出 Markdown']) {
    await center.getByRole('button', { name, exact: true }).click();
    await expect(center.getByRole('status')).toContainText('private-export');
  }
  await center.getByRole('button', { name: '选择 JSON 并演练' }).click();
  await expect(center).toContainText('新增：2');
  const confirm = center.getByRole('button', { name: '确认导入', exact: true });
  await expect(confirm).toBeDisabled();
  await center
    .getByRole('checkbox', { name: '我已审阅数量，确认写入这些记录（写入前会创建完整备份）' })
    .check();
  await confirm.click();
  await expect(center.getByRole('status')).toContainText('导入完成');
  await center.getByRole('button', { name: '选择完整备份并演练' }).click();
  await expect(center).toContainText('isolated-fixture/restore');
  for (const theme of ['light', 'dark'] as const) {
    await page.evaluate((t) => (document.documentElement.dataset.theme = t), theme);
    await center.scrollIntoViewIfNeeded();
    expect(
      await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth),
    ).toBeTruthy();
    await page.screenshot({
      path: `tests/screenshots/phase13-privacy-${theme}-${info.project.name}.png`,
      fullPage: true,
    });
  }
});

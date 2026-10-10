import { test, expect } from '@playwright/test';
import { installModuleRegistryFixture } from '../fixtures/module-registry';
test('Vault CRUD archive restore private preview and controlled attachment feedback', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    Object.assign(window, { isTauri: true });
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string, args: Record<string, unknown>) => {
        const records = JSON.parse(localStorage.getItem('fixture.vault') ?? '[]');
        if (command === 'vault_snapshot')
          return {
            artifacts: records,
            journeys: [{ id: '00000000-0000-4000-8000-000000000001', title: '隔离Journey' }],
            competitions: [{ id: '00000000-0000-4000-8000-000000000002', title: '隔离比赛' }],
            limit: 1000,
          };
        if (command === 'vault_files')
          return [
            {
              id: 'file-fixture',
              name: 'fixture.pdf',
              size_bytes: 32,
              sha256: 'a'.repeat(64),
              mime_type: 'application/pdf',
            },
          ];
        if (command === 'vault_verify') return 'missing';
        if (command === 'vault_action') {
          const a = args.action as {
            type: string;
            artifact: { id: string; revision: number };
            id: string;
            state: string;
          };
          if (a.type === 'save') {
            const i = records.findIndex((x: { id: string }) => x.id === a.artifact.id);
            const value = { ...a.artifact, revision: a.artifact.revision + 1 };
            if (i < 0) records.push(value);
            else records[i] = value;
          } else {
            const record = records.find((x: { id: string }) => x.id === a.id);
            record.revision++;
            if (a.state === 'archive') record.archived_at = 'fixture';
            if (a.state === 'trash') record.deleted_at = 'fixture';
            if (a.state === 'restore') {
              record.archived_at = null;
              record.deleted_at = null;
            }
          }
          localStorage.setItem('fixture.vault', JSON.stringify(records));
          return null;
        }
        throw Error('Unexpected fixture command');
      },
    };
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/vault');
  await page.getByRole('button', { name: '新建作品', exact: true }).click();
  await page.getByLabel('作品标题', { exact: true }).fill('隔离真实作品');
  await page.getByLabel('作品描述', { exact: true }).fill('仅用户原文');
  await page.getByLabel('技能（逗号分隔）').fill('Rust, Python');
  await page.getByLabel('关联 Journey').selectOption('00000000-0000-4000-8000-000000000001');
  await page.getByLabel('关联比赛').selectOption('00000000-0000-4000-8000-000000000002');
  await page.getByRole('button', { name: '保存作品', exact: true }).click();
  await expect(page.locator('.vault-list')).toContainText('仅用户原文');
  await page.getByRole('button', { name: '校验文件' }).click();
  await expect(page.getByText('文件缺失', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: '作品集预览', exact: true }).click();
  await expect(page.locator('.vault-preview')).not.toContainText('隔离真实作品');
  await page.getByRole('button', { name: '当前作品', exact: true }).click();
  await page.getByRole('button', { name: '编辑', exact: true }).click();
  await page.getByLabel('可见性').selectOption('public');
  await page.getByRole('button', { name: '保存作品', exact: true }).click();
  await page.getByRole('button', { name: '作品集预览', exact: true }).click();
  await expect(page.locator('.vault-preview')).toContainText('仅用户原文');
  await expect(page.locator('.vault-preview')).not.toContainText('fixture.pdf');
  await page.screenshot({
    path: `tests/screenshots/phase11-portfolio-${info.project.name}.png`,
    fullPage: true,
  });
  await page.getByRole('button', { name: '当前作品', exact: true }).click();
  await page.getByRole('button', { name: '归档', exact: true }).click();
  await expect(page.locator('.vault-list article')).toHaveCount(0);
  await page.getByRole('button', { name: '已归档', exact: true }).click();
  await page.getByRole('button', { name: '恢复', exact: true }).click();
  await page.getByRole('button', { name: '当前作品', exact: true }).click();
  await page.getByRole('button', { name: '移至回收站', exact: true }).click();
  await page.getByRole('button', { name: '回收站', exact: true }).click();
  await page.getByRole('button', { name: '恢复', exact: true }).click();
  await page.getByRole('button', { name: '当前作品', exact: true }).click();
  await page.reload();
  await expect(page.locator('.vault-list')).toContainText('隔离真实作品');
  for (const dark of [false, true]) {
    await page.evaluate(
      (v) => (document.documentElement.dataset.theme = v ? 'dark' : 'light'),
      dark,
    );
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(
      true,
    );
    await page.screenshot({
      path: `tests/screenshots/phase11-vault-${dark ? 'dark' : 'light'}-${info.project.name}.png`,
      fullPage: true,
    });
  }
  await page.getByRole('button', { name: '编辑', exact: true }).click();
  await page.getByLabel('作品标题', { exact: true }).focus();
  await expect(page.getByLabel('作品标题', { exact: true })).toBeFocused();
  await page.screenshot({
    path: `tests/screenshots/phase11-vault-editor-${info.project.name}.png`,
    fullPage: true,
  });
});

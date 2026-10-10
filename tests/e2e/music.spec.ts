import { test, expect } from '@playwright/test';
import { installModuleRegistryFixture } from '../fixtures/module-registry';
test('Music manual editing clouds private history and safe media preferences', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    Object.assign(window, { isTauri: true });
    const read = () =>
      JSON.parse(
        localStorage.getItem('fixture.music') ??
          '{"items":[],"preferences":{"enabled":false,"threshold_seconds":30,"allowed_app":"","revision":0}}',
      );
    const write = (s: unknown) => localStorage.setItem('fixture.music', JSON.stringify(s));
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string, args: Record<string, unknown>) => {
        const s = read();
        if (command === 'music_status')
          return {
            state: 'manual-fallback',
            source_app: 'fixture.player',
            metadata_available: false,
            error: null,
          };
        if (command === 'music_snapshot') {
          const items = s.items.filter((p: { deleted: boolean }) => p.deleted === args.trash);
          const live = s.items.filter((p: { deleted: boolean }) => !p.deleted);
          return {
            items,
            cloud: live.map(
              (p: { title: string; artist: string; album: string; duration_seconds: number }) => ({
                ...p,
                count: 1,
                seconds: p.duration_seconds,
              }),
            ),
            totals: {
              count: live.length,
              seconds: live.reduce(
                (v: number, p: { duration_seconds: number }) => v + p.duration_seconds,
                0,
              ),
            },
            preferences: s.preferences,
            journals: [{ id: '00000000-0000-4000-8000-000000000010', title: '隔离音乐日记' }],
            visibility: 'private',
          };
        }
        if (command === 'music_action') {
          const a = args.action as Record<string, unknown>;
          if (a.type === 'save') {
            if (a.id) {
              const p = s.items.find((p: { id: string }) => p.id === a.id);
              if (p.revision !== a.revision) throw Error('MUSIC_CONFLICT');
              Object.assign(p, a, { revision: p.revision + 1 });
            } else
              s.items.push({
                ...a,
                id: crypto.randomUUID(),
                revision: 1,
                source: 'manual',
                source_app: '',
                journal_title: a.journal_id ? '隔离音乐日记' : null,
                deleted: false,
              });
          } else if (a.type === 'trash') {
            const p = s.items.find((p: { id: string }) => p.id === a.id);
            p.deleted = !a.restore;
            p.revision++;
          } else if (a.type === 'preferences') {
            s.preferences = { ...(a.preferences as object), revision: s.preferences.revision + 1 };
          }
          write(s);
          return null;
        }
        throw Error('UNEXPECTED_COMMAND');
      },
    };
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/music');
  await expect(page.getByRole('heading', { name: '音乐', exact: true })).toBeVisible();
  await page.getByLabel('歌曲', { exact: true }).fill('隔离测试歌曲');
  await page.getByLabel('艺术家', { exact: true }).fill('测试艺术家');
  await page.getByLabel('专辑', { exact: true }).fill('测试专辑');
  await page.getByLabel('平台', { exact: true }).fill('手动');
  await page
    .getByLabel('关联日记', { exact: true })
    .selectOption('00000000-0000-4000-8000-000000000010');
  await page.getByRole('button', { name: '保存音乐记录', exact: true }).click();
  await expect(page.getByText('音乐记录已保存', { exact: true })).toBeVisible();
  await expect(page.getByRole('link', { name: '关联日记：隔离音乐日记' })).toBeVisible();
  await page.getByLabel('云类型').selectOption('album');
  await expect(page.locator('.music-cloud')).toContainText('测试专辑');
  await page.getByRole('button', { name: '编辑音乐记录', exact: true }).click();
  await page.getByLabel('实际聆听秒数').fill('240');
  await page.getByRole('button', { name: '保存音乐记录', exact: true }).click();
  await expect(page.locator('.music-history')).toContainText('240 秒');
  await page.reload();
  await expect(page.locator('.music-history')).toContainText('240 秒');
  await page.getByRole('button', { name: '移入音乐回收站' }).click();
  await expect(page.locator('.music-history li')).toHaveCount(0);
  await page.getByRole('checkbox', { name: '音乐回收站' }).check();
  await page.getByRole('button', { name: '恢复音乐记录' }).click();
  await page.getByRole('checkbox', { name: '音乐回收站' }).uncheck();
  await expect(page.locator('.music-history li')).toHaveCount(1);
  await page.getByRole('checkbox', { name: '启用媒体监听' }).check();
  await page.getByLabel('允许的来源应用 ID').fill('fixture.player');
  await page.getByLabel('计入阈值（秒）').fill('10');
  await page.getByRole('button', { name: '保存监听设置' }).click();
  await page.reload();
  await expect(page.getByRole('checkbox', { name: '启用媒体监听' })).toBeChecked();
  await expect(page.getByLabel('计入阈值（秒）')).toHaveValue('10');
  for (const dark of [false, true]) {
    await page.evaluate(
      (value) => (document.documentElement.dataset.theme = value ? 'dark' : 'light'),
      dark,
    );
    await expect
      .poll(() => page.evaluate(() => document.documentElement.scrollWidth <= innerWidth))
      .toBe(true);
    await page.screenshot({
      path: `tests/screenshots/phase10-music-${dark ? 'dark' : 'light'}-${info.project.name}.png`,
      fullPage: true,
    });
  }
});
test('module switching keeps the application shell and forms have usable spacing', async ({
  page,
}, info) => {
  await page.goto('/#/settings');
  await page.locator('aside').evaluate((el) => el.setAttribute('data-shell-check', 'persistent'));
  for (const name of ['音乐', '洞察', '日记', '编程练习']) {
    const start = Date.now();
    await page
      .getByRole('navigation', { name: '主要导航' })
      .getByRole('link', { name, exact: true })
      .click();
    await expect(page.locator('aside[data-shell-check="persistent"]')).toBeVisible();
    await expect(page.locator('.route-content')).not.toContainText('正在载入模块');
    expect(Date.now() - start).toBeLessThan(3000);
  }
  await page.getByRole('link', { name: '设置', exact: true }).click();
  const gap = await page
    .locator('.journal-settings')
    .evaluate((el) => Number.parseFloat(getComputedStyle(el).rowGap));
  expect(gap).toBeGreaterThanOrEqual(12);
  await page.goto('/#/');
  const field = await page.locator('#daily-capacity').boundingBox(),
    button = await page.getByRole('button', { name: '保存容量' }).boundingBox();
  expect(field).toBeTruthy();
  expect(button).toBeTruthy();
  expect(button!.y - (field!.y + field!.height)).toBeGreaterThanOrEqual(11);
  await page.screenshot({
    path: `tests/screenshots/phase10-form-spacing-${info.project.name}.png`,
    fullPage: true,
  });
});

import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { test, expect } from '@playwright/test';
import type { Entry, Filter, Save, Attachment } from '../../src/features/journal/model';
test('Journal autosave, restart recovery, filters, markdown, trash and quick capture', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    const read = (): Entry[] => JSON.parse(localStorage.getItem('fixture.journal.entries') ?? '[]');
    const write = (entries: Entry[]) =>
      localStorage.setItem('fixture.journal.entries', JSON.stringify(entries));
    let attachments: Attachment[] = [];
    Object.assign(window, {
      isTauri: true,
      __TAURI_INTERNALS__: {
        invoke: async (
          command: string,
          args: {
            input?: Save;
            filter?: Filter;
            id?: string;
            revision?: number;
            restore?: boolean;
            url?: string;
          },
        ) => {
          const entries = read();
          if (command === 'journal_save') {
            const s = args.input!,
              old = entries.find((e) => e.id === s.id);
            if (old && old.revision !== s.revision) throw { code: 'JOURNAL_CONFLICT' };
            const e: Entry = {
              ...s,
              title: s.title || s.body.slice(0, 60),
              is_draft: !(s.publish || old?.is_draft === false),
              revision: (old?.revision ?? 0) + 1,
              deleted_at: null,
              updated_at: new Date().toISOString(),
            };
            write(old ? entries.map((x) => (x.id === e.id ? e : x)) : [...entries, e]);
            return e;
          }
          if (command === 'journal_get') return entries.find((e) => e.id === args.id);
          if (command === 'journal_trash') {
            const e = entries.find((e) => e.id === args.id)!;
            if (e.revision !== args.revision) throw { code: 'JOURNAL_CONFLICT' };
            e.deleted_at = args.restore ? null : new Date().toISOString();
            e.revision++;
            write(entries);
            return e;
          }
          if (command === 'journal_page') {
            const f = args.filter!;
            const selected = entries.filter(
              (e) =>
                (f.mode === 'trash'
                  ? !!e.deleted_at
                  : !e.deleted_at && e.is_draft === (f.mode === 'drafts')) &&
                (!f.query || e.title.includes(f.query) || e.body.includes(f.query)) &&
                (!f.entry_type || e.entry_type === f.entry_type) &&
                (!f.tag || e.tags.includes(f.tag)) &&
                (!f.from || e.date >= f.from) &&
                (!f.until || e.date <= f.until),
            );
            return {
              entries: selected.slice(f.page * 30, (f.page + 1) * 30),
              total: selected.length,
              page: f.page,
              recoveries: [],
              journeys: [],
              tasks: [],
              music: [],
            };
          }
          if (command === 'journal_attachments') return attachments;
          if (command === 'journal_link') {
            attachments = [
              ...attachments,
              {
                id: crypto.randomUUID(),
                name: args.url!,
                url: args.url!,
                size_bytes: null,
                sha256: null,
              },
            ];
            return attachments;
          }
          if (command === 'capture_open' || command === 'capture_close') return null;
          throw Error('Unsupported Journal fixture');
        },
      },
    });
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/journal');
  await page.getByRole('button', { name: '新建记录', exact: true }).click();
  await page.getByLabel('标题', { exact: true }).fill('独立学习记录');
  await page.getByLabel('记录类型', { exact: true }).selectOption('study_note');
  await page.getByLabel('标签（逗号分隔）').fill('学习, 复盘');
  await page
    .getByLabel('Markdown 正文')
    .fill(
      '# 真实思考\n\n自动保存必须保留新内容。\n\n<script>window.unsafe=1</script>\n[危险](javascript:alert(1))',
    );
  await expect(page.getByRole('status')).toHaveText('已自动保存到本机');
  await page.reload();
  await page.getByRole('button', { name: '草稿', exact: true }).click();
  await expect(page.getByRole('heading', { name: '独立学习记录' })).toBeVisible();
  await page.getByRole('button', { name: '编辑', exact: true }).click();
  await expect(page.getByLabel('Markdown 正文')).toContainText('自动保存必须保留新内容');
  await page.getByRole('button', { name: '预览', exact: true }).click();
  await expect(page.getByRole('heading', { name: '真实思考' })).toBeVisible();
  await expect(page.locator('.journal-markdown script')).toHaveCount(0);
  await page.getByRole('button', { name: '保存记录', exact: true }).click();
  await expect(page.getByRole('status')).toHaveText('已自动保存到本机');
  await page.getByLabel('链接地址').fill('https://example.com/study');
  await page.getByRole('button', { name: '添加链接', exact: true }).click();
  await expect(page.getByRole('link', { name: 'https://example.com/study' })).toBeVisible();
  for (const theme of ['light', 'dark']) {
    await page.evaluate((theme) => {
      document.documentElement.dataset.theme = theme;
    }, theme);
    await page.screenshot({
      path: `tests/screenshots/phase10-journal-editor-${theme}-${info.project.name}.png`,
      fullPage: true,
    });
  }
  await page.getByRole('button', { name: '返回列表', exact: true }).click();
  await page.getByRole('button', { name: '已保存', exact: true }).click();
  await page.getByLabel('搜索正文或标题').fill('自动保存');
  await page.getByLabel('标签筛选').fill('复盘');
  await expect(page.getByRole('heading', { name: '独立学习记录' })).toBeVisible();
  await page.getByRole('button', { name: '移入回收站', exact: true }).click();
  await expect(page.getByRole('heading', { name: '独立学习记录' })).toHaveCount(0);
  await page.getByRole('button', { name: '撤销移入回收站', exact: true }).click();
  await expect(page.getByRole('heading', { name: '独立学习记录' })).toBeVisible();
  await page.screenshot({
    path: `tests/screenshots/phase10-journal-list-${info.project.name}.png`,
    fullPage: true,
  });
  await page.goto('/#/capture');
  const bodyBox = await page.getByLabel('Markdown 正文').boundingBox();
  const saveBox = await page.getByRole('button', { name: '保存记录', exact: true }).boundingBox();
  expect(bodyBox).not.toBeNull();
  expect(saveBox).not.toBeNull();
  expect(saveBox!.y - (bodyBox!.y + bodyBox!.height)).toBeGreaterThanOrEqual(12);
  await page.getByLabel('Markdown 正文').fill('快速记录必须完整落库');
  await page.getByLabel('标签（逗号分隔）').fill('随手记');
  await page.getByRole('button', { name: '保存记录', exact: true }).click();
  await expect(page.getByText('记录已保存。可以继续写下一条。')).toBeVisible();
  await page.screenshot({
    path: `tests/screenshots/phase10-quick-capture-${info.project.name}.png`,
    fullPage: true,
  });
  await page.goto('/#/journal');
  await expect(page.getByRole('heading', { name: '快速记录必须完整落库' })).toBeVisible();
  expect(
    await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth),
  ).toBeTruthy();
});

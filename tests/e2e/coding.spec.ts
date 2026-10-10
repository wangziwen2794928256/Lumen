import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { test, expect } from '@playwright/test';
import type {
  Action,
  Filter,
  Problem,
  Attempt,
  Contest,
  Snapshot,
} from '../../src/features/coding/model';
test('Coding manual CRUD, attempts, analytics, persistence, trash and restore', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    const read = (): {
      problems: Problem[];
      attempts: Attempt[];
      contests: Contest[];
      receipts: Record<string, string>;
    } =>
      JSON.parse(
        localStorage.getItem('fixture.coding') ??
          '{"problems":[],"attempts":[],"contests":[],"receipts":{}}',
      );
    Object.assign(window, {
      isTauri: true,
      __TAURI_INTERNALS__: {
        invoke: async (command: string, args: { action?: Action; filter?: Filter }) => {
          const db = read();
          if (command === 'coding_action') {
            const a = args.action!;
            const body = JSON.stringify(a);
            if (db.receipts[a.request_id]) {
              if (db.receipts[a.request_id] !== body) throw { code: 'CODING_CONFLICT' };
              return;
            }
            const rows =
              a.type === 'problem'
                ? db.problems
                : a.type === 'attempt'
                  ? db.attempts
                  : a.type === 'contest'
                    ? db.contests
                    : a.kind === 'problem'
                      ? db.problems
                      : a.kind === 'attempt'
                        ? db.attempts
                        : db.contests;
            const old = rows.find((r) => r.id === a.id);
            if (a.type === 'trash') {
              if (!old || old.revision !== a.revision) throw { code: 'CODING_CONFLICT' };
              old.deleted_at = a.restore ? null : new Date().toISOString();
              old.revision++;
            } else {
              if (old && old.revision !== a.revision) throw { code: 'CODING_CONFLICT' };
              if (
                a.type === 'problem' &&
                db.problems.some(
                  (p) =>
                    p.id !== a.id &&
                    !p.deleted_at &&
                    p.platform === a.input.platform &&
                    p.external_id === a.input.external_id,
                )
              )
                throw { code: 'CODING_DUPLICATE' };
              if ('revision' in a.input || 'id' in a.input) throw { code: 'INVALID_INPUT' };
              const saved = {
                ...a.input,
                id: a.id,
                revision: (old?.revision ?? 0) + 1,
                deleted_at: null,
              };
              if (old) Object.assign(old, saved);
              else rows.push(saved as Problem & Attempt & Contest);
            }
            db.receipts[a.request_id] = body;
            localStorage.setItem('fixture.coding', JSON.stringify(db));
            return;
          }
          if (command === 'coding_snapshot') {
            const f = args.filter!,
              live = db.attempts.filter(
                (a) =>
                  !a.deleted_at && db.problems.some((p) => p.id === a.problem_id && !p.deleted_at),
              );
            const enriched = db.problems.map((p) => {
              const a = live.filter((a) => a.problem_id === p.id);
              const accepted = a.find((a) => a.outcome === 'accepted');
              return {
                ...p,
                attempt_count: a.length,
                solved: !!accepted,
                first_attempt: a[0]?.started_at ?? null,
                solved_at: accepted?.started_at ?? null,
                language: a.at(-1)?.language ?? null,
                manual_seconds: a.reduce((n, a) => n + a.manual_seconds, 0),
                focus_seconds: 0,
              };
            });
            const filtered = enriched.filter(
              (p) =>
                !!p.deleted_at === f.trash &&
                (!f.query || p.title.includes(f.query) || p.external_id.includes(f.query)) &&
                (!f.tag || p.tags.includes(f.tag)) &&
                (!f.platform || p.platform === f.platform) &&
                (!f.status ||
                  (f.status === 'solved'
                    ? p.solved
                    : f.status === 'unsolved'
                      ? !p.solved && p.attempt_count > 0
                      : p.attempt_count === 0)),
            );
            const practiced = enriched.filter((p) => !p.deleted_at && p.attempt_count > 0);
            const result: Snapshot = {
              problems: filtered.slice(f.page * 30, f.page * 30 + 30),
              total: filtered.length,
              attempts: db.attempts
                .filter((a) => a.problem_id === f.selected)
                .map((a) => ({ ...a, focus_seconds: 0 })),
              contests: db.contests,
              journeys: [],
              tasks: [],
              focus: [],
              analytics: {
                from: f.week,
                until: f.week,
                problems: practiced.length,
                solved: practiced.filter((p) => p.solved).length,
                attempts: live.length,
                first_pass_numerator: practiced.filter(
                  (p) => live.find((a) => a.problem_id === p.id)?.outcome === 'accepted',
                ).length,
                first_pass_denominator: practiced.length,
                manual_seconds: live.reduce((n, a) => n + a.manual_seconds, 0),
                focus_seconds: 0,
                contests: db.contests.filter((c) => c.participated && !c.deleted_at).length,
                difficulty: practiced.length ? [{ label: 'medium', count: practiced.length }] : [],
                topics: practiced.length ? [{ label: 'DP', count: practiced.length }] : [],
              },
            };
            return result;
          }
          throw Error(command);
        },
      },
    });
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/coding');
  await expect(page.getByText('还没有题目记录。')).toBeVisible();
  await page.getByRole('button', { name: '周统计', exact: true }).click();
  await expect(page.getByText('暂无样本').first()).toBeVisible();
  await page.getByRole('button', { name: '添加竞赛' }).click();
  await page.getByLabel('名称', { exact: true }).fill('手工竞赛');
  await page.getByLabel('参加状态').selectOption('true');
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByRole('heading', { name: '竞赛资料' })).toHaveCount(0);
  await page.getByRole('button', { name: '题目', exact: true }).click();
  await page.getByRole('button', { name: '添加题目' }).click();
  await page.getByLabel('名称', { exact: true }).fill('原生算法练习');
  await page.getByLabel('平台题号').fill('phase10-fixture');
  await page.getByLabel('难度', { exact: true }).selectOption('medium');
  await page.getByLabel('算法标签（逗号分隔）').fill('DP, Graph');
  await page.getByLabel('关联竞赛').selectOption({ label: '手工竞赛' });
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByRole('heading', { name: '原生算法练习', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '尝试记录', exact: true }).click();
  await page.getByRole('button', { name: '记录尝试', exact: true }).click();
  await page.getByLabel('语言', { exact: true }).fill('Rust');
  await page.getByLabel('结果', { exact: true }).selectOption('wrong_answer');
  await page.getByLabel('额外手工用时（分钟）').fill('10');
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByRole('heading', { name: /答案错误/ })).toBeVisible();
  await page.getByRole('button', { name: '记录尝试', exact: true }).click();
  await page.getByLabel('语言', { exact: true }).fill('Rust');
  await page.getByLabel('结果', { exact: true }).selectOption('accepted');
  await page.getByLabel('额外手工用时（分钟）').fill('5');
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByText(/已解出 · 2 次尝试/)).toBeVisible();
  await page.getByRole('button', { name: '周统计', exact: true }).click();
  await expect(page.getByText('0.0% (0/1)')).toBeVisible();
  await expect(page.getByText('15.0 分钟').first()).toBeVisible();
  for (const mode of ['light', 'dark']) {
    await page.evaluate((mode) => {
      document.documentElement.dataset.theme = mode;
    }, mode);
    await page.screenshot({
      path: `tests/screenshots/phase10-coding-stats-${mode}-${info.project.name}.png`,
      fullPage: true,
      animations: 'disabled',
    });
  }
  await page.getByRole('button', { name: '题目', exact: true }).click();
  const card = page.locator('.coding-problem');
  await card.getByRole('button', { name: '编辑', exact: true }).click();
  await page.getByLabel('笔记').fill('保存解题思路');
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByText('保存解题思路')).toBeVisible();
  await page.reload();
  await expect(page.getByText('保存解题思路')).toBeVisible();
  await page.getByRole('button', { name: '添加题目' }).click();
  await page.getByLabel('名称', { exact: true }).fill('重复题');
  await page.getByLabel('平台题号').fill('phase10-fixture');
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByRole('alert')).toContainText('平台与题号已存在');
  await expect(page.getByLabel('名称', { exact: true })).toHaveValue('重复题');
  await page.getByLabel('平台题号').fill('second');
  await page.getByRole('button', { name: '保存', exact: true }).click();
  await page.getByLabel('搜索名称或题号').fill('原生算法');
  await expect(page.locator('.coding-problem')).toHaveCount(1);
  await page.screenshot({
    path: `tests/screenshots/phase10-coding-list-${info.project.name}.png`,
    fullPage: true,
    animations: 'disabled',
  });
  await page.locator('.coding-problem').getByRole('button', { name: '移入回收站' }).click();
  await expect(page.getByText('还没有题目记录。')).toBeVisible();
  await page.getByLabel('题目范围').selectOption('true');
  await expect(page.locator('.coding-problem')).toHaveCount(1);
  await page.getByRole('button', { name: '恢复', exact: true }).click();
  await page.getByLabel('题目范围').selectOption('false');
  await expect(page.getByText(/已解出 · 2 次尝试/)).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(
    true,
  );
});

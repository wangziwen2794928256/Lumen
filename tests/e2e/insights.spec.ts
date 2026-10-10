import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { test, expect } from '@playwright/test';
test('Insights range, annual activity, reflection recovery, private exports and themes', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    const now = new Date();
    const day = [
      now.getFullYear(),
      String(now.getMonth() + 1).padStart(2, '0'),
      String(now.getDate()).padStart(2, '0'),
    ].join('-');
    const read = () =>
      JSON.parse(
        localStorage.getItem('fixture.insights') ??
          '{"body":"","next_focus":"","revision":0,"status":"normal","status_revision":0,"preferences_revision":0,"fail":true}',
      );
    const prefs = {
      weights: [1, 1, 10, 10, 10, 1],
      thresholds: [1, 30, 90, 180],
      stop_words: [],
      memories: true,
    };
    const make = (from: string, until: string) => {
      const s = read();
      const days = [];
      for (
        let d = new Date(from + 'T12:00:00Z');
        d.toISOString().slice(0, 10) <= until;
        d.setUTCDate(d.getUTCDate() + 1)
      ) {
        const date = d.toISOString().slice(0, 10);
        const active = date === day;
        days.push({
          date,
          deep_minutes: active ? 30 : 0,
          coding_minutes: active ? 10 : 0,
          tasks_completed: active ? 1 : 0,
          journal_count: active ? 1 : 0,
          problem_count: active ? 1 : 0,
          music_count: active ? 2 : 0,
          planned_minutes: active ? 60 : 0,
          capacity_minutes: 210,
          status: active ? s.status : 'normal',
          status_revision: active ? s.status_revision : 0,
          intensity: active ? 3 : 0,
        });
      }
      return {
        from,
        until,
        days,
        heatmap: days,
        preferences: s.preferences ?? prefs,
        preferences_revision: s.preferences_revision,
        words: [
          {
            word: '数学',
            count: 2,
            records: [
              {
                kind: 'journal',
                id: '00000000-0000-4000-8000-000000000001',
                date: day,
                title: '隔离数学记录',
              },
            ],
          },
        ],
        subjects: [{ label: '数学', minutes: 30 }],
        journeys: [{ label: '隔离 Journey', total: 2, completed: 1 }],
        music: [
          { label: '隔离歌曲', artist: '隔离艺术家', album: '隔离专辑', count: 2, seconds: 180 },
        ],
        topics: [{ label: 'DP', count: 1 }],
        records: [],
        task_summary: { total: 2, completed: 1, unfinished: 1, moved: 0 },
        consistency: {
          days7: { active: 1, eligible: 7, protected: 0 },
          days28: { active: 1, eligible: 28, protected: 0 },
        },
        previous: {
          deep_minutes: 20,
          coding_minutes: 0,
          planned_minutes: 60,
          tasks_completed: 0,
          journals: 0,
        },
      };
    };
    Object.assign(window, {
      isTauri: true,
      __TAURI_INTERNALS__: {
        invoke: async (
          cmd: string,
          args: {
            filter?: { from: string; until: string };
            week?: string;
            month?: string;
            action?: {
              type: string;
              date?: string;
              status?: string;
              body?: string;
              next_focus?: string;
              value?: unknown;
            };
          },
        ) => {
          const s = read();
          if (cmd === 'insights_snapshot') {
            const d = make(args.filter!.from, args.filter!.until);
            d.heatmap = make(day.slice(0, 4) + '-01-01', day.slice(0, 4) + '-12-31').days;
            return d;
          }
          if (cmd === 'insights_action') {
            const a = args.action!;
            if (a.type === 'review') {
              if (s.fail) {
                s.fail = false;
                localStorage.setItem('fixture.insights', JSON.stringify(s));
                throw { code: 'INSIGHTS_CONFLICT' };
              }
              s.body = a.body;
              s.next_focus = a.next_focus;
              s.revision++;
            } else if (a.type === 'day') {
              s.status = a.status;
              s.status_revision++;
            } else {
              s.preferences = a.value;
              s.preferences_revision++;
            }
            localStorage.setItem('fixture.insights', JSON.stringify(s));
            return;
          }
          if (cmd === 'insights_review')
            return {
              week: args.week,
              facts: make(day, day),
              body: s.body,
              next_focus: s.next_focus,
              revision: s.revision,
            };
          if (cmd === 'insights_month') {
            const facts = make(day, day);
            return {
              version: 1,
              visibility: 'private',
              month: args.month,
              from: day,
              until: day,
              closed: false,
              generated_at: now.toISOString(),
              totals: {
                deep_minutes: 30,
                coding_minutes: 10,
                planned_minutes: 60,
                journals: 1,
                tasks_completed: 1,
              },
              days: facts.days,
              subjects: facts.subjects,
              topics: facts.topics,
              music: facts.music,
              words: [{ word: '数学', count: 2 }],
              counts: { competitions: 1, achievements: 0, projects: 0, photos: 0 },
              task_summary: facts.task_summary,
            };
          }
          if (cmd === 'insights_memories') return [];
          if (cmd === 'capture_status')
            return { shortcut: 'Control+Shift+Space', registered: true, error: null };
          throw { code: 'FIXTURE_UNSUPPORTED' };
        },
      },
    });
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/insights');
  await expect(page.getByText('Journey 计划任务进度')).toBeVisible();
  await page.getByLabel('时间范围').selectOption('30');
  await expect(page.getByRole('img', { name: /实际专注趋势，30 天/ })).toBeVisible();
  for (const theme of ['light', 'dark']) {
    await page.evaluate((theme) => (document.documentElement.dataset.theme = theme), theme);
    await page.screenshot({
      path: `tests/screenshots/phase10-insights-${theme}-${info.project.name}.png`,
      fullPage: true,
      animations: 'disabled',
    });
  }
  await page.getByRole('button', { name: '年度活动', exact: true }).click();
  const current = new Date();
  const label = [
    current.getFullYear(),
    String(current.getMonth() + 1).padStart(2, '0'),
    String(current.getDate()).padStart(2, '0'),
  ].join('-');
  await page.getByRole('button', { name: new RegExp(`^${label} 普通日`) }).click();
  await expect(page.getByRole('heading', { name: /当日详情/ })).toBeVisible();
  await page.getByLabel('当日状态').selectOption('rest');
  await expect(page.getByText('已保存', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: '返回洞察' }).click();
  await page.screenshot({
    path: `tests/screenshots/phase10-heatmap-${info.project.name}.png`,
    fullPage: true,
    animations: 'disabled',
  });
  await page.getByRole('button', { name: '本周词语', exact: true }).click();
  await page.getByRole('button', { name: '数学，2 次，查看来源' }).click();
  await expect(page.getByRole('link', { name: /隔离数学记录/ })).toHaveAttribute(
    'href',
    /journal\?entry=/,
  );
  await page.getByRole('button', { name: '周回顾', exact: true }).click();
  await page.getByLabel('My Reflection').fill('只由用户填写的真实心得');
  await page.getByRole('button', { name: '保存我的心得' }).click();
  await expect(page.getByRole('alert')).toBeVisible();
  await page.reload();
  await page.getByRole('button', { name: '周回顾', exact: true }).click();
  await expect(page.getByLabel('My Reflection')).toHaveValue('只由用户填写的真实心得');
  await page.getByRole('button', { name: '保存我的心得' }).click();
  await expect(page.getByRole('alert')).toHaveCount(0);
  await page.screenshot({
    path: `tests/screenshots/phase10-review-${info.project.name}.png`,
    fullPage: true,
    animations: 'disabled',
  });
  await page.getByRole('button', { name: '月快照', exact: true }).click();
  const jsonWait = page.waitForEvent('download');
  await page.getByRole('button', { name: '导出私密 JSON' }).click();
  const json = await jsonWait;
  await json.saveAs(`docs/testing/phase10/exports/fixture-${info.project.name}-private.json`);
  expect(json.suggestedFilename()).toMatch(/private\.json$/);
  const stream = await json.createReadStream();
  let content = '';
  for await (const chunk of stream!) content += chunk.toString();
  expect(JSON.parse(content).visibility).toBe('private');
  expect(content).not.toContain('真实心得');
  const pngWait = page.waitForEvent('download');
  await page.getByRole('button', { name: '导出私密 PNG' }).click();
  const png = await pngWait;
  await png.saveAs(`docs/testing/phase10/exports/fixture-${info.project.name}-private.png`);
  expect(png.suggestedFilename()).toMatch(/private\.png$/);
  const pngStream = await png.createReadStream();
  const chunks = [];
  for await (const chunk of pngStream!) chunks.push(chunk);
  const bytes = Buffer.concat(chunks);
  expect(bytes.subarray(0, 8).toString('hex')).toBe('89504e470d0a1a0a');
  expect(bytes.readUInt32BE(16)).toBe(1200);
  expect(bytes.readUInt32BE(20)).toBe(800);
  await page.getByRole('button', { name: '活动配置', exact: true }).click();
  await page.getByLabel('自定义停用词（空格分隔）').fill('数学 研究');
  await page.getByRole('button', { name: '保存活动配置' }).click();
  await expect(page.getByLabel('自定义停用词（空格分隔）')).toHaveValue('数学 研究');
  await page.screenshot({
    path: `tests/screenshots/phase10-settings-${info.project.name}.png`,
    fullPage: true,
    animations: 'disabled',
  });
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
});

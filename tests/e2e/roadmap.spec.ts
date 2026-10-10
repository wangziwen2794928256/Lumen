import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { test, expect } from '@playwright/test';
import type {
  RoadmapAction,
  RoadmapSnapshot,
  Entity,
  Details,
} from '../../src/features/roadmap/model';
import type { TodaySnapshot, TodayAction } from '../../src/features/today/model';
test('Roadmap CRUD, capacity, editable audited decisions and annual rule snapshots', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    const detail: Details = {
      description: '',
      category: 'study',
      status: 'active',
      start_date: null,
      target_date: null,
      minimum_minutes: 0,
      preferred_minutes: 0,
      maximum_minutes: 10080,
      priority: 1,
      color: 'sage',
      icon: 'route',
      progress_mode: 'manual',
      progress: 0,
      parent_id: null,
      preparation_phase: '',
      position: 0,
      planning_target: null,
    };
    const snapshot: RoadmapSnapshot = {
      week_start: '',
      entities: [],
      tasks: [],
      preferences: {
        weekday_minutes: 210,
        weekend_minutes: 300,
        side_quest_max: 360,
        planning_target: 400,
        revision: 0,
      },
      days: [],
      blocks: [],
      decisions: [],
      rules: [],
      budgets: [],
    };
    const today: TodaySnapshot = {
      date: new Date().toLocaleDateString('sv-SE'),
      tasks: [],
      trash: [],
      session: null,
      capacity_minutes: 210,
      planned_minutes: 0,
      actual_seconds: 0,
      journey: null,
      notes: [],
      timeline: [],
    };
    const entity = (
      id: string,
      kind: Entity['kind'],
      title: string,
      journey_id: string | null = null,
    ): Entity => ({
      id,
      kind,
      title,
      journey_id,
      data: { ...detail },
      revision: 0,
      deleted_at: null,
      archived_at: null,
    });
    Object.assign(window, {
      isTauri: true,
      __TAURI_INTERNALS__: {
        invoke: async (
          command: string,
          args: { week: string; action: RoadmapAction | TodayAction },
        ) => {
          if (command === 'today_snapshot') return today;
          if (command === 'today_action') {
            const a = args.action as TodayAction;
            if (a.type === 'create') {
              today.tasks.push({
                ...a.input,
                id: a.id,
                remaining_minutes: a.input.estimated_minutes,
                lifecycle: 'planned',
                revision: 0,
                deleted_at: null,
              });
              snapshot.tasks.push({
                id: a.id,
                title: a.input.title,
                remaining_minutes: a.input.estimated_minutes,
                priority: a.input.priority,
                deadline: a.input.due_date,
                journey_id: null,
                subject_id: null,
                quest_id: null,
                revision: 0,
                options: {
                  earliest_start: null,
                  minimum_session: 15,
                  preferred_session: 25,
                  splittable: true,
                  energy_level: 'normal',
                  fixed_time: false,
                },
              });
            }
            return today;
          }
          snapshot.week_start = args.week;
          if (command === 'roadmap_snapshot') return snapshot;
          if (command !== 'roadmap_action') throw Error('Unsupported fixture');
          const a = args.action as RoadmapAction;
          if (a.type === 'template') {
            snapshot.entities.push(
              entity(a.id, 'journey', 'Fudan 2028'),
              entity('bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb', 'subject', '数学', a.id),
            );
          }
          if (a.type === 'save') {
            const existing = snapshot.entities.find((e) => e.id === a.id);
            if (existing) {
              Object.assign(existing, a.input);
              existing.revision++;
            } else snapshot.entities.push({ ...entity(a.id, a.kind, a.input.title), ...a.input });
          }
          if (a.type === 'lifecycle') {
            const existing = snapshot.entities.find((e) => e.id === a.id)!;
            existing.revision++;
            if (a.operation === 'trash') existing.deleted_at = new Date().toISOString();
            if (a.operation === 'restore') existing.deleted_at = null;
            if (a.operation === 'archive') existing.archived_at = new Date().toISOString();
            if (a.operation === 'unarchive') existing.archived_at = null;
          }
          if (a.type === 'preferences')
            snapshot.preferences = { ...a.input, revision: a.input.revision + 1 };
          if (a.type === 'capacity') {
            snapshot.days = snapshot.days.filter((d) => d.date !== a.input.date);
            snapshot.days.push(a.input);
          }
          if (a.type === 'task_plan') {
            const task = snapshot.tasks.find((t) => t.id === a.id)!;
            Object.assign(task, {
              journey_id: a.journey_id,
              subject_id: a.subject_id,
              quest_id: a.quest_id,
              options: a.input,
              revision: task.revision + 1,
            });
          }
          if (a.type === 'propose') {
            snapshot.decisions.unshift({
              id: a.id,
              week_start: args.week,
              status: 'proposed',
              reason: '固定事件扣除后安排 25 分钟；总容量不增加。',
              before: snapshot.blocks.map((a) => ({ ...a })),
              after: {
                days: [
                  {
                    date: args.week,
                    capacity: 210,
                    fixed_minutes: 0,
                    available: 210,
                    status: 'normal',
                  },
                ],
                allocations: snapshot.tasks.length
                  ? [{ task_id: snapshot.tasks[0].id, date: args.week, minutes: 25 }]
                  : [],
                unscheduled: [],
                warnings: [],
                reason: '总容量不增加',
              },
              created_at: new Date().toISOString(),
              updated_at: new Date().toISOString(),
            });
          }
          if (a.type === 'decide') {
            const d = snapshot.decisions.find((d) => d.id === a.id)!;
            d.status =
              a.operation === 'accept'
                ? 'accepted'
                : a.operation === 'modify'
                  ? 'modified'
                  : a.operation === 'reject'
                    ? 'rejected'
                    : 'undone';
            if (a.operation === 'modify') d.after.allocations = a.allocations!;
            if (['accept', 'modify'].includes(a.operation))
              snapshot.blocks = d.after.allocations.map((a) => ({ ...a }));
            if (a.operation === 'undo') snapshot.blocks = d.before.map((a) => ({ ...a }));
          }
          if (a.type === 'rule')
            snapshot.rules.push({
              id: a.id,
              input: a.input,
              captured_at: new Date().toISOString(),
              content_hash: 'f'.repeat(64),
              deleted_at: null,
            });
          if (a.type === 'rule_trash')
            snapshot.rules.find((r) => r.id === a.id)!.deleted_at = a.deleted
              ? new Date().toISOString()
              : null;
          return snapshot;
        },
      },
    });
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/roadmap');
  await expect(
    page.getByText('还没有长期目标。从一个目标开始，也可以选择可编辑模板。'),
  ).toBeVisible();
  await page.getByRole('button', { name: '创建 Fudan 2028 模板' }).click();
  await expect(page.getByRole('heading', { name: 'Fudan 2028', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '新建长期目标', exact: true }).click();
  let dialog = page.getByRole('dialog');
  await dialog.getByLabel('名称', { exact: true }).fill('自主长期目标');
  await dialog.getByLabel('每周偏好分钟').fill('120');
  await dialog.getByRole('button', { name: '保存', exact: true }).click();
  const own = page
    .locator('article')
    .filter({ has: page.getByRole('heading', { name: '自主长期目标', exact: true }) });
  await own.getByRole('button', { name: '编辑', exact: true }).click();
  dialog = page.getByRole('dialog');
  await dialog.getByLabel('名称', { exact: true }).fill('自主长期目标已编辑');
  await dialog.getByRole('button', { name: '保存', exact: true }).click();
  for (const name of ['阶段', '学科', '支线任务']) {
    await page.getByRole('button', { name: '新建' + name, exact: true }).click();
    dialog = page.getByRole('dialog');
    await dialog.getByLabel('名称', { exact: true }).fill('隔离' + name);
    await dialog.getByLabel('关联长期目标').selectOption({ label: 'Fudan 2028' });
    await dialog.getByRole('button', { name: '保存', exact: true }).click();
    await expect(page.getByRole('heading', { name: '隔离' + name, exact: true })).toBeVisible();
  }
  await page.getByRole('button', { name: '新建任务', exact: true }).click();
  dialog = page.getByRole('dialog');
  await dialog.getByLabel('任务标题').fill('隔离规划任务');
  await dialog.getByRole('button', { name: '保存任务', exact: true }).click();
  await page.getByRole('button', { name: '配置规划', exact: true }).click();
  dialog = page.getByRole('dialog');
  await dialog.getByLabel('关联长期目标').selectOption({ label: 'Fudan 2028' });
  await dialog.getByLabel('学科', { exact: true }).selectOption({ label: '数学' });
  await dialog.getByRole('button', { name: '保存', exact: true }).click();
  for (const mode of ['light', 'dark']) {
    await page.getByRole('link', { name: '设置', exact: true }).click();
    await page
      .getByRole('button', { name: mode === 'light' ? '浅色' : '深色', exact: true })
      .click();
    await page.getByRole('link', { name: '路线图', exact: true }).click();
    await page.screenshot({
      path: `tests/screenshots/phase10-roadmap-${mode}-${info.project.name}.png`,
      fullPage: true,
    });
  }
  await page.getByRole('button', { name: '生成建议' }).click();
  await expect(page.getByRole('heading', { name: '待确认', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '修改安排', exact: true }).click();
  await page.getByLabel('安排分钟').fill('20');
  await page.getByRole('button', { name: '保存修改', exact: true }).click();
  await expect(page.getByRole('heading', { name: '已修改接受', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '撤销调整', exact: true }).click();
  await expect(page.getByRole('heading', { name: '已撤销', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '生成建议' }).click();
  await page.getByRole('button', { name: '接受建议' }).click();
  await page.getByRole('button', { name: '撤销调整', exact: true }).click();
  await page.getByRole('button', { name: '生成建议' }).click();
  await page.getByRole('button', { name: '拒绝建议' }).click();
  await expect(page.getByRole('heading', { name: '已拒绝', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '保存年度资料' }).click();
  dialog = page.getByRole('dialog');
  await dialog.getByLabel('学校', { exact: true }).fill('复旦大学');
  await dialog.getByLabel('招生年度').fill('2027');
  await dialog.getByLabel('官方来源网址').fill('https://gsao.fudan.edu.cn/15015/list.htm');
  await dialog.getByLabel('来源摘录 / 用户摘要').fill('隔离摘要，非未来规则');
  await page.screenshot({ path: `tests/screenshots/phase10-rule-dialog-${info.project.name}.png` });
  await dialog.getByRole('button', { name: '保存', exact: true }).click();
  await expect(page.getByText('复旦大学 · 2027 ·', { exact: false })).toBeVisible();
  const removed = page
    .locator('article')
    .filter({ has: page.getByRole('heading', { name: '自主长期目标已编辑', exact: true }) });
  await removed.getByRole('button', { name: '移入回收站' }).click();
  await page.getByText('回收站与归档', { exact: true }).click();
  await page.getByRole('button', { name: '恢复', exact: true }).click();
  await expect(
    page.getByRole('heading', { name: '自主长期目标已编辑', exact: true }),
  ).toBeVisible();
  await page.screenshot({
    path: `tests/screenshots/phase10-planner-decisions-${info.project.name}.png`,
    fullPage: true,
  });
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
});

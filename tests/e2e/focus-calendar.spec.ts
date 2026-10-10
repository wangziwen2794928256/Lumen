import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { test, expect } from '@playwright/test';
import type { TodayAction, TodaySnapshot } from '../../src/features/today/model';
import type { CalendarSnapshot } from '../../src/features/calendar/model';
test('Focus recovery and four calendar views edit planned events beside observed intervals', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    const day = new Date().toLocaleDateString('sv-SE'),
      taskId = '33333333-3333-4333-8333-333333333333';
    const at = (time: string) => new Date(day + 'T' + time).toISOString();
    const task = {
      id: taskId,
      title: '隔离专注验收任务',
      body: '',
      due_date: day,
      estimated_minutes: 25,
      remaining_minutes: 25,
      priority: 1,
      lifecycle: 'active' as const,
      revision: 1,
      deleted_at: null,
    };
    const today: TodaySnapshot = {
      date: day,
      tasks: [task],
      trash: [],
      session: {
        id: '44444444-4444-4444-8444-444444444444',
        task_id: taskId,
        state: 'interrupted',
        elapsed_ms: 240000,
        paused_ms: 60000,
        interrupted_ms: 3600000,
        planned_seconds: 1500,
        last_activity_at: at('10:04:00'),
        interruption_reason: 'restart',
      },
      capacity_minutes: 210,
      planned_minutes: 25,
      actual_seconds: 0,
      journey: null,
      notes: [],
      timeline: [],
      system_blocked: false,
      native_hooks_ready: true,
    };
    const calendar: CalendarSnapshot = {
      from: '',
      until: '',
      tasks: [task],
      task_options: [task],
      trash: [],
      events: [
        {
          id: '55555555-5555-4555-8555-555555555555',
          title: '课程计划 fixture',
          event_type: 'class',
          event_date: null,
          starts_at: at('10:00:00'),
          ends_at: at('11:00:00'),
          task_id: taskId,
          revision: 0,
        },
      ],
      actual: [
        {
          id: 'interval-1',
          session_id: today.session!.id,
          task_id: taskId,
          title: task.title,
          local_date: day,
          starts_at: at('10:00:00'),
          ends_at: at('10:04:00'),
          elapsed_ms: 240000,
          state: 'interrupted',
          provenance: 'observed',
        },
      ],
    };
    Object.assign(window, {
      isTauri: true,
      __TAURI_INTERNALS__: {
        invoke: async (
          command: string,
          args: { action: TodayAction; from: string; until: string },
        ) => {
          if (command === 'today_snapshot') return today;
          if (command === 'calendar_snapshot')
            return { ...calendar, from: args.from, until: args.until };
          if (command !== 'today_action') throw Error('Unsupported fixture IPC');
          const a = args.action;
          if (a.type === 'session' && today.session) {
            if (a.operation === 'resume') today.session.state = 'running';
            if (a.operation === 'pause') today.session.state = 'paused';
            if (a.operation === 'interrupt') today.session.state = 'interrupted';
            if (a.operation === 'cancel') {
              today.session = null;
              task.lifecycle = 'active';
            }
          }
          if (a.type === 'calendar_create' && !calendar.events.some((e) => e.id === a.id))
            calendar.events.push({ ...a.input, id: a.id, revision: 0 });
          if (a.type === 'calendar_edit') {
            const event = calendar.events.find((e) => e.id === a.id)!;
            Object.assign(event, a.input, { revision: event.revision + 1 });
          }
          if (a.type === 'calendar_trash') {
            const from = a.deleted ? calendar.events : calendar.trash,
              to = a.deleted ? calendar.trash : calendar.events;
            const index = from.findIndex((e) => e.id === a.id);
            const event = from.splice(index, 1)[0];
            if (event) to.push({ ...event, revision: event.revision + 1 });
          }
          return today;
        },
      },
    });
  });
  await installModuleRegistryFixture(page);
  await page.goto('/focus');
  await expect(page.getByText('等待恢复', { exact: true })).toBeVisible();
  await expect(page.getByLabel('实际专注时长')).toHaveText('04:00');
  await page.getByRole('button', { name: '继续', exact: true }).click();
  await expect(page.getByText('专注中', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: '暂停', exact: true }).click();
  await expect(page.getByText('已暂停', { exact: true })).toBeVisible();
  for (const [mode, label] of [
    ['light', '浅色'],
    ['dark', '深色'],
  ]) {
    await page.getByRole('link', { name: '设置', exact: true }).click();
    await page.getByRole('button', { name: label, exact: true }).click();
    await page.getByRole('link', { name: '专注', exact: true }).click();
    await page.screenshot({
      path: `tests/screenshots/phase10-focus-${mode}-${info.project.name}.png`,
      fullPage: true,
    });
    await page.getByRole('link', { name: '日历', exact: true }).click();
    for (const view of ['日', '周', '月', '议程']) {
      await page.getByRole('button', { name: view, exact: true }).click();
      await expect(
        page.getByText('计划 · 课程计划 fixture · 课程 · 10:00–11:00', { exact: true }),
      ).toBeVisible();
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(
        true,
      );
      await page.screenshot({
        path: `tests/screenshots/phase10-calendar-${view}-${mode}-${info.project.name}.png`,
        fullPage: true,
      });
    }
  }
  await page.getByRole('button', { name: '新建日历事件', exact: true }).click();
  const dialog = page.getByRole('dialog');
  await dialog.getByLabel('事件标题').fill('新增计划 fixture');
  await dialog.getByLabel('全天', { exact: true }).check();
  await dialog.getByRole('button', { name: '保存事件' }).click();
  await expect(dialog).not.toBeVisible();
  await expect(
    page.getByText('计划 · 新增计划 fixture · 个人安排 · 全天', { exact: true }),
  ).toBeVisible();
  const row = page.getByRole('listitem').filter({ hasText: '新增计划 fixture' });
  await row.getByRole('button', { name: '编辑事件' }).click();
  await dialog.getByLabel('事件标题').fill('修改计划 fixture');
  await dialog.getByRole('button', { name: '保存事件' }).click();
  const edited = page.getByRole('listitem').filter({ hasText: '修改计划 fixture' });
  await edited.getByRole('button', { name: '移入事件回收站' }).click();
  await page.getByText('事件回收站 (1)', { exact: true }).click();
  await page.getByRole('button', { name: '恢复事件' }).click();
  await expect(
    page.getByText('计划 · 修改计划 fixture · 个人安排 · 全天', { exact: true }),
  ).toBeVisible();
  await page.getByRole('button', { name: '显示计划', exact: true }).click();
  await expect(
    page.getByText('计划 · 修改计划 fixture · 个人安排 · 全天', { exact: true }),
  ).not.toBeVisible();
  await expect(
    page.getByText('实际 · 隔离专注验收任务 · 04:00 · 进行中的已确认片段', { exact: true }),
  ).toBeVisible();
});

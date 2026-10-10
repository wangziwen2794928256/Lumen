import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { test, expect } from '@playwright/test';
import type { TodayAction, TodaySnapshot } from '../../src/features/today/model';
// Test-only IPC fixture. Native SQLite/monotonic semantics are separately verified.
test('Today UI creates, pauses, finishes and retains an IPC snapshot across reload', async ({
  page,
}, info) => {
  await page.addInitScript(() => {
    const date = new Date().toLocaleDateString('sv-SE');
    const snapshot: TodaySnapshot = JSON.parse(
      localStorage.getItem('phase4-test-fixture') ?? 'null',
    ) ?? {
      date,
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
    Object.assign(window, {
      isTauri: true,
      __TAURI_INTERNALS__: {
        invoke: async (command: string, args: { action: TodayAction }) => {
          if (command === 'today_snapshot') return snapshot;
          if (command !== 'today_action') throw new Error('Unsupported test IPC');
          const a = args.action;
          if (a.type === 'create' && !snapshot.tasks.some((t) => t.id === a.id)) {
            snapshot.tasks.push({
              ...a.input,
              id: a.id,
              lifecycle: 'planned',
              remaining_minutes: a.input.estimated_minutes,
              revision: 0,
              deleted_at: null,
            });
            snapshot.planned_minutes += a.input.estimated_minutes;
          } else if (a.type === 'start') {
            const task = snapshot.tasks.find((t) => t.id === a.task_id)!;
            task.lifecycle = 'active';
            task.revision++;
            snapshot.session = {
              id: a.id,
              task_id: a.task_id,
              state: 'running',
              elapsed_ms: 7000,
              paused_ms: 0,
              interrupted_ms: 0,
              planned_seconds: 1500,
              last_activity_at: new Date().toISOString(),
            };
          } else if (a.type === 'session' && snapshot.session) {
            if (a.operation === 'pause') snapshot.session.state = 'paused';
            if (a.operation === 'resume') snapshot.session.state = 'running';
            if (a.operation === 'finish') {
              const task = snapshot.tasks.find((t) => t.id === snapshot.session!.task_id)!;
              task.lifecycle = 'done';
              task.revision++;
              snapshot.actual_seconds = 7;
              snapshot.timeline = [
                {
                  id: a.id,
                  title: task.title,
                  started_at: new Date().toISOString(),
                  actual_seconds: 7,
                },
              ];
              snapshot.session = null;
            }
          }
          localStorage.setItem('phase4-test-fixture', JSON.stringify(snapshot));
          return snapshot;
        },
      },
    });
  });
  await installModuleRegistryFixture(page);
  await page.goto('/');
  const noteBox = await page.getByLabel('随手记', { exact: true }).boundingBox();
  const noteSaveBox = await page
    .getByRole('button', { name: '保存随手记', exact: true })
    .boundingBox();
  expect(noteBox).not.toBeNull();
  expect(noteSaveBox).not.toBeNull();
  expect(noteSaveBox!.y - (noteBox!.y + noteBox!.height)).toBeGreaterThanOrEqual(12);
  await page.getByRole('button', { name: '新建任务', exact: true }).click();
  const dialog = page.getByRole('dialog');
  await dialog.getByLabel('任务标题').fill('隔离 UI 验收任务');
  await dialog.getByLabel('任务备注').fill('仅存在于 Playwright fixture');
  await page.screenshot({ path: `tests/screenshots/phase10-task-form-${info.project.name}.png` });
  await dialog.getByRole('button', { name: '保存任务' }).click();
  await expect(dialog).not.toBeVisible();
  await expect(page.getByRole('heading', { name: '隔离 UI 验收任务' })).toBeVisible();
  await page.getByRole('button', { name: '开始专注', exact: true }).click();
  await expect(page.getByLabel('实际专注时长')).toHaveText('00:07');
  for (const [mode, label] of [
    ['light', '浅色'],
    ['dark', '深色'],
  ]) {
    await page.getByRole('link', { name: '设置', exact: true }).click();
    await page.getByRole('button', { name: label, exact: true }).click();
    await page.getByRole('link', { name: '今日', exact: true }).click();
    await expect(page.getByLabel('实际专注时长')).toHaveText('00:07');
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(
      true,
    );
    await page.screenshot({
      path: `tests/screenshots/phase10-task-running-${mode}-${info.project.name}.png`,
      fullPage: true,
    });
  }
  await page.getByRole('button', { name: '暂停', exact: true }).click();
  await expect(page.getByText('已暂停', { exact: true })).toBeVisible();
  await page.reload();
  await expect(page.getByText('已暂停', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: '继续', exact: true }).click();
  await page.getByRole('button', { name: '完成任务并结束', exact: true }).click();
  await expect(page.getByText('已完成', { exact: false }).first()).toBeVisible();
  await expect(page.getByText('实际完成专注：00:07', { exact: true })).toBeVisible();
  await page.reload();
  await expect(page.getByText('实际完成专注：00:07', { exact: true })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  await page.screenshot({
    path: `tests/screenshots/phase10-task-finished-${info.project.name}.png`,
    fullPage: true,
  });
});

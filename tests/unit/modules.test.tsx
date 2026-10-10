import { it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { ModuleProvider } from '../../src/kernel/modules/ModuleProvider';
import { ModuleSettings } from '../../src/kernel/modules/ModuleSettings';
import { ModuleGate, TodayWidgets } from '../../src/kernel/modules/Extensions';
import {
  defaults,
  normalizeState,
  ordered,
  reorder,
  validateCatalogue,
} from '../../src/kernel/modules/registry';
import { planExchangeSchema, planContextSchema } from '../../src/kernel/plan/exchange';
vi.mock('../../src/features/insights/Memories', () => ({
  Memories: () => <span>official-widget</span>,
}));
vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => false, invoke: vi.fn() }));
beforeEach(() => localStorage.clear());
it('plan context rejects unselected private bodies and unknown fields', () => {
  const context = {
    schema_version: '1.0',
    request_id: crypto.randomUUID(),
    basis_hash: 'a'.repeat(64),
    tasks: [
      {
        task_id: crypto.randomUUID(),
        revision: 0,
        estimate_minutes: 25,
        actual_ms: 26000,
        due_date: '2026-10-04',
      },
    ],
    capacities: [{ date: '2026-10-04', minutes: 210 }],
  };
  expect(planContextSchema.safeParse(context).success).toBe(true);
  expect(planContextSchema.safeParse({ ...context, journal_body: 'private' }).success).toBe(false);
  expect(
    planContextSchema.safeParse({ ...context, tasks: [{ ...context.tasks[0], body: 'private' }] })
      .success,
  ).toBe(false);
});
it('registry rejects duplicate identities and required kernel disable', () => {
  expect(() => validateCatalogue([{ id: 'lumen.a' }])).toThrow();
  const state = structuredClone(defaults);
  state.items[0].enabled = false;
  expect(() => normalizeState(state)).toThrow();
  const duplicate = structuredClone(defaults);
  duplicate.items[1].id = duplicate.items[0].id;
  expect(() => normalizeState(duplicate)).toThrow();
});
it('hidden and pinned navigation plus stable drag order do not discard modules', () => {
  const state = structuredClone(defaults);
  state.items.find((i) => i.id === 'lumen.journal')!.hidden = true;
  state.items.find((i) => i.id === 'lumen.insights')!.pinned = true;
  expect(ordered(state, true)[0].id).toBe('lumen.insights');
  expect(ordered(state, true).some((i) => i.id === 'lumen.journal')).toBe(false);
  const moved = reorder(defaults, 'lumen.insights', 'lumen.focus');
  expect(ordered(moved)[1].id).toBe('lumen.insights');
  expect(moved.items).toHaveLength(10);
});
it('explicit module disable survives reload and guards the official route', async () => {
  const first = render(
    <ModuleProvider>
      <ModuleSettings />
      <TodayWidgets slot="today.after-summary" />
      <ModuleGate id="lumen.insights">
        <span>official-content</span>
      </ModuleGate>
    </ModuleProvider>,
  );
  fireEvent.click(screen.getByRole('checkbox', { name: '洞察 启用' }));
  fireEvent.click(screen.getByRole('button', { name: '保存模块配置' }));
  await waitFor(() => expect(screen.queryByText('official-content')).not.toBeInTheDocument());
  expect(screen.queryByText('official-widget')).not.toBeInTheDocument();
  expect(screen.getByRole('checkbox', { name: '今日 启用' })).toBeDisabled();
  first.unmount();
  render(
    <ModuleProvider>
      <ModuleGate id="lumen.insights">
        <span>official-content</span>
      </ModuleGate>
    </ModuleProvider>,
  );
  expect(screen.getByRole('status')).toHaveTextContent('已停用');
});
it('plan proposals reject capacity overflow, invalid dates, duplicate blocks and unrecognized fields', () => {
  const p = {
    schema_version: '1.0',
    request_id: crypto.randomUUID(),
    basis_hash: 'a'.repeat(64),
    source: 'manual',
    capacity_minutes: 30,
    items: [{ task_id: crypto.randomUUID(), date: '2026-10-04', minutes: 25 }],
  };
  expect(planExchangeSchema.safeParse(p).success).toBe(true);
  expect(planExchangeSchema.safeParse({ ...p, capacity_minutes: 20 }).success).toBe(false);
  expect(planExchangeSchema.safeParse({ ...p, network: 'https://example.com' }).success).toBe(
    false,
  );
  expect(
    planExchangeSchema.safeParse({ ...p, items: [{ ...p.items[0], date: '2026-02-29' }] }).success,
  ).toBe(false);
  expect(
    planExchangeSchema.safeParse({ ...p, capacity_minutes: 60, items: [p.items[0], p.items[0]] })
      .success,
  ).toBe(false);
});

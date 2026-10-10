import type { Page } from '@playwright/test';
import { defaults } from '../../src/kernel/modules/registry';
// Completes existing isolated domain IPC fixtures with the new Kernel API.
// It does not relax domain errors or assertions and never touches native storage.
export async function installModuleRegistryFixture(page: Page) {
  await page.addInitScript((initial) => {
    const host = (
      window as unknown as {
        __TAURI_INTERNALS__?: { invoke: (command: string, args: unknown) => Promise<unknown> };
      }
    ).__TAURI_INTERNALS__;
    if (!host) return;
    const original = host.invoke;
    host.invoke = async (command, args) => {
      if (command === 'module_registry_get')
        return JSON.parse(
          localStorage.getItem('fixture.kernel.modules') ?? JSON.stringify(initial),
        );
      if (command === 'module_registry_save') {
        const { value } = args as { value: typeof initial };
        const stored = JSON.parse(
          localStorage.getItem('fixture.kernel.modules') ?? JSON.stringify(initial),
        ) as typeof initial;
        if (stored.revision !== value.revision) throw Error('MODULE_CONFLICT');
        const next = { ...value, revision: value.revision + 1 };
        localStorage.setItem('fixture.kernel.modules', JSON.stringify(next));
        return next;
      }
      return original(command, args);
    };
  }, defaults);
}

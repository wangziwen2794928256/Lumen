import { invoke, isTauri } from '@tauri-apps/api/core';
import type { Action, Filter, Snapshot } from './model';
function desktop() {
  if (!isTauri()) throw new Error('DESKTOP_REQUIRED');
}
export const codingApi = {
  snapshot: async (filter: Filter) => {
    desktop();
    return invoke<Snapshot>('coding_snapshot', { filter });
  },
  action: (action: Action) => {
    desktop();
    return invoke<void>('coding_action', { action });
  },
};

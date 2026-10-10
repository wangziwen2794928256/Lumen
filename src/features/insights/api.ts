import { invoke, isTauri } from '@tauri-apps/api/core';
import type { Action, Facts, Review, Monthly, RecordLink } from './model';
function call<T>(cmd: string, args: Record<string, unknown>) {
  if (!isTauri()) return Promise.reject(new Error('DESKTOP_REQUIRED'));
  return invoke<T>(cmd, args);
}
export const insightsApi = {
  snapshot: (from: string, until: string, today: string) =>
    call<Facts>('insights_snapshot', { filter: { from, until, today } }),
  action: (action: Action) => call<void>('insights_action', { action }),
  review: (week: string) => call<Review>('insights_review', { week }),
  month: (month: string, today: string) => call<Monthly>('insights_month', { month, today }),
  memories: (today: string) => call<RecordLink[]>('insights_memories', { today }),
};

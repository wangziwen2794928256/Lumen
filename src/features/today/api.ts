import { invoke, isTauri } from '@tauri-apps/api/core';
import type { TodayAction, TodaySnapshot } from './model';
export async function loadToday(): Promise<TodaySnapshot | null> {
  return isTauri() ? invoke<TodaySnapshot>('today_snapshot') : null;
}
export async function mutateToday(action: TodayAction): Promise<TodaySnapshot> {
  if (!isTauri()) throw new Error('DESKTOP_REQUIRED');
  return invoke<TodaySnapshot>('today_action', { action });
}

import { invoke, isTauri } from '@tauri-apps/api/core';
import type { Snapshot, MediaStatus } from './model';
export const musicApi = {
  snapshot: (from: string, until: string, trash: boolean) =>
    invoke<Snapshot>('music_snapshot', { from, until, trash }),
  action: async (action: Record<string, unknown>) => {
    const key = 'lumen.music.pending.v1';
    const { request_id, ...payload } = action;
    let previous: { payload: string; request_id: string } | null = null;
    try {
      previous = JSON.parse(localStorage.getItem(key) ?? 'null');
    } catch {
      /* A malformed draft never authorizes a write. */
    }
    const value = JSON.stringify(payload),
      id = previous?.payload === value ? previous.request_id : request_id;
    localStorage.setItem(key, JSON.stringify({ payload: value, request_id: id }));
    await invoke<void>('music_action', { action: { ...payload, request_id: id } });
    localStorage.removeItem(key);
  },
  status: () => invoke<MediaStatus>('music_status'),
  available: isTauri,
};

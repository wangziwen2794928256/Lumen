import { invoke, isTauri } from '@tauri-apps/api/core';
export { isTauri };
export async function vaultInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args);
}
export async function vaultAction(payload: Record<string, unknown>) {
  const key = 'lumen.vault.pending.v1';
  let previous: { payload: string; request_id: string } | null = null;
  try {
    previous = JSON.parse(localStorage.getItem(key) ?? 'null');
  } catch {
    /* Ignore malformed pending metadata. */
  }
  const serialized = JSON.stringify(payload),
    request_id = previous?.payload === serialized ? previous.request_id : crypto.randomUUID();
  localStorage.setItem(key, JSON.stringify({ payload: serialized, request_id }));
  await invoke('vault_action', { action: { ...payload, request_id } });
  localStorage.removeItem(key);
}

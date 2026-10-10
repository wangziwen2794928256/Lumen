import { invoke, isTauri } from '@tauri-apps/api/core';
export async function journalInvoke<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!isTauri()) throw new Error('DESKTOP_REQUIRED');
  return invoke<T>(command, args);
}

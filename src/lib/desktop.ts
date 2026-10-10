import { invoke, isTauri } from '@tauri-apps/api/core';
export interface AppInfo {
  name: string;
  version: string;
  platform: string;
}
export async function verifyDesktop(): Promise<AppInfo | null> {
  return isTauri() ? invoke<AppInfo>('app_info') : null;
}
export interface DatabaseStatus {
  schema_version: number;
  integrity: string;
  storage: string;
  task_count: number;
  journal_count: number;
}
export async function verifyDatabase(): Promise<DatabaseStatus | null> {
  return isTauri() ? invoke<DatabaseStatus>('database_status') : null;
}

# IPC 与类型契约参考

本卷从当前源码提取签名与模型，作为开发者查阅入口。它描述可信的官方模块 IPC，不是社区插件可访问的 API。Tauri 注入 AppHandle/State，renderer 只传业务参数。完整 Rust 实现仍是权威校验。

## 调用与错误约定

```ts
import { invoke } from '@tauri-apps/api/core';
import type { TodaySnapshot } from '../../features/today/model';
const snapshot = await invoke<TodaySnapshot>('today_snapshot');
// See the module api.ts for validated write payloads and failure handling.
```

invoke 参数采用 JS camelCase 对应 Rust snake_case 参数，内部 JSON 字段遵循具体 serde 模型。不要根据命令名称猜 payload。读取 api.ts 的实际调用与 model.ts 的联合类型，再检查 Rust serde deny_unknown_fields / Action。mutation 使用稳定请求 UUID；收到成功回执前保留输入。冲突时重新载入并让用户决策，不自动覆盖。错误对象包含 code/message，业务界面优先按 code 映射；不要把私人正文或文件路径复制到日志。日期仅 yyyy-MM-dd 的字段不当作 UTC 时刻，UTC 字段保留 RFC3339 Z。

## 注册命令清单

| Command | Source |
| --- | --- |
| `coding_snapshot` | [src-tauri/src/commands/coding.rs](../../src-tauri/src/commands/coding.rs) |
| `coding_action` | [src-tauri/src/commands/coding.rs](../../src-tauri/src/commands/coding.rs) |
| `desktop_status` | [src-tauri/src/commands/desktop.rs](../../src-tauri/src/commands/desktop.rs) |
| `desktop_preferences_save` | [src-tauri/src/commands/desktop.rs](../../src-tauri/src/commands/desktop.rs) |
| `desktop_autostart` | [src-tauri/src/commands/desktop.rs](../../src-tauri/src/commands/desktop.rs) |
| `desktop_test_notification` | [src-tauri/src/commands/desktop.rs](../../src-tauri/src/commands/desktop.rs) |
| `onboarding_complete` | [src-tauri/src/commands/desktop.rs](../../src-tauri/src/commands/desktop.rs) |
| `onboarding_skip` | [src-tauri/src/commands/desktop.rs](../../src-tauri/src/commands/desktop.rs) |
| `insights_snapshot` | [src-tauri/src/commands/insights.rs](../../src-tauri/src/commands/insights.rs) |
| `insights_action` | [src-tauri/src/commands/insights.rs](../../src-tauri/src/commands/insights.rs) |
| `insights_review` | [src-tauri/src/commands/insights.rs](../../src-tauri/src/commands/insights.rs) |
| `insights_month` | [src-tauri/src/commands/insights.rs](../../src-tauri/src/commands/insights.rs) |
| `insights_memories` | [src-tauri/src/commands/insights.rs](../../src-tauri/src/commands/insights.rs) |
| `journal_page` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_get` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_save` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_trash` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_attachments` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_pick_attachment` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_link` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_verify` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `capture_open` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `journal_open_attachment` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `capture_close` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `capture_status` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `capture_shortcut` | [src-tauri/src/commands/journal.rs](../../src-tauri/src/commands/journal.rs) |
| `roadmap_snapshot` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `roadmap_action` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `calendar_snapshot` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `today_snapshot` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `today_action` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `database_status` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `app_info` | [src-tauri/src/commands/mod.rs](../../src-tauri/src/commands/mod.rs) |
| `module_registry_get` | [src-tauri/src/commands/modules.rs](../../src-tauri/src/commands/modules.rs) |
| `module_registry_save` | [src-tauri/src/commands/modules.rs](../../src-tauri/src/commands/modules.rs) |
| `music_snapshot` | [src-tauri/src/commands/music.rs](../../src-tauri/src/commands/music.rs) |
| `music_action` | [src-tauri/src/commands/music.rs](../../src-tauri/src/commands/music.rs) |
| `music_status` | [src-tauri/src/commands/music.rs](../../src-tauri/src/commands/music.rs) |
| `backup_status` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `backup_policy_save` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `backup_create` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `privacy_status` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `privacy_disable` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `privacy_open_folder` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `private_export` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `import_review` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `import_confirm` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `restore_rehearse` | [src-tauri/src/commands/privacy.rs](../../src-tauri/src/commands/privacy.rs) |
| `profile_get` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `profile_save` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `public_projection` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `public_export` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `public_preview` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `deep_link_initial` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `deep_link_register` | [src-tauri/src/commands/profile.rs](../../src-tauri/src/commands/profile.rs) |
| `vault_snapshot` | [src-tauri/src/commands/vault.rs](../../src-tauri/src/commands/vault.rs) |
| `vault_action` | [src-tauri/src/commands/vault.rs](../../src-tauri/src/commands/vault.rs) |
| `vault_files` | [src-tauri/src/commands/vault.rs](../../src-tauri/src/commands/vault.rs) |
| `vault_pick_attachment` | [src-tauri/src/commands/vault.rs](../../src-tauri/src/commands/vault.rs) |
| `vault_verify` | [src-tauri/src/commands/vault.rs](../../src-tauri/src/commands/vault.rs) |

## 参数与返回签名

### `coding_snapshot`

```rust
fn coding_snapshot(
    state: tauri::State<'_, DatabaseState>,
    filter: Filter,
) -> Result<Snapshot>;
```

### `coding_action`

```rust
fn coding_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()>;
```

### `desktop_status`

```rust
fn desktop_status(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Status>;
```

### `desktop_preferences_save`

```rust
fn desktop_preferences_save(
    state: tauri::State<'_, DatabaseState>,
    preferences: Preferences,
) -> Result<Preferences>;
```

### `desktop_autostart`

```rust
fn desktop_autostart(app: tauri::AppHandle, enabled: bool) -> Result<bool>;
```

### `desktop_test_notification`

```rust
fn desktop_test_notification(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
) -> Result<()>;
```

### `onboarding_complete`

```rust
fn onboarding_complete(
    state: tauri::State<'_, DatabaseState>,
    input: OnboardingInput,
) -> Result<()>;
```

### `onboarding_skip`

```rust
fn onboarding_skip(state: tauri::State<'_, DatabaseState>) -> Result<()>;
```

### `insights_snapshot`

```rust
fn insights_snapshot(
    state: tauri::State<'_, DatabaseState>,
    filter: Filter,
) -> Result<Snapshot>;
```

### `insights_action`

```rust
fn insights_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()>;
```

### `insights_review`

```rust
fn insights_review(
    state: tauri::State<'_, DatabaseState>,
    week: String,
) -> Result<serde_json::Value>;
```

### `insights_month`

```rust
fn insights_month(
    state: tauri::State<'_, DatabaseState>,
    month: String,
    today: String,
) -> Result<serde_json::Value>;
```

### `insights_memories`

```rust
fn insights_memories(
    state: tauri::State<'_, DatabaseState>,
    today: String,
) -> Result<Vec<Record>>;
```

### `journal_page`

```rust
fn journal_page(state: tauri::State<'_, DatabaseState>, filter: Filter) -> Result<Page>;
```

### `journal_get`

```rust
fn journal_get(state: tauri::State<'_, DatabaseState>, id: String) -> Result<Entry>;
```

### `journal_save`

```rust
fn journal_save(state: tauri::State<'_, DatabaseState>, input: Save) -> Result<Entry>;
```

### `journal_trash`

```rust
fn journal_trash(
    state: tauri::State<'_, DatabaseState>,
    id: String,
    revision: i64,
    restore: bool,
) -> Result<Entry>;
```

### `journal_attachments`

```rust
fn journal_attachments(
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<Vec<Attachment>>;
```

### `journal_pick_attachment`

```rust
fn journal_pick_attachment(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<Vec<Attachment>>;
```

### `journal_link`

```rust
fn journal_link(
    state: tauri::State<'_, DatabaseState>,
    id: String,
    url: String,
) -> Result<Vec<Attachment>>;
```

### `journal_verify`

```rust
fn journal_verify(state: tauri::State<'_, DatabaseState>, id: String) -> Result<String>;
```

### `capture_open`

```rust
fn capture_open(app: tauri::AppHandle) -> Result<()>;
```

### `journal_open_attachment`

```rust
fn journal_open_attachment(
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<()>;
```

### `capture_close`

```rust
fn capture_close(app: tauri::AppHandle) -> Result<()>;
```

### `capture_status`

```rust
fn capture_status(state: tauri::State<'_, capture::CaptureState>) -> Result<capture::Status>;
```

### `capture_shortcut`

```rust
fn capture_shortcut(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    shortcut: String,
) -> Result<capture::Status>;
```

### `roadmap_snapshot`

```rust
fn roadmap_snapshot(
    state: tauri::State<'_, TodayState>,
    week: String,
) -> crate::database::Result<crate::today::roadmap::Snapshot>;
```

### `roadmap_action`

```rust
fn roadmap_action(
    state: tauri::State<'_, TodayState>,
    week: String,
    action: crate::today::roadmap::Action,
) -> crate::database::Result<crate::today::roadmap::Snapshot>;
```

### `calendar_snapshot`

```rust
fn calendar_snapshot(
    state: tauri::State<'_, TodayState>,
    from: String,
    until: String,
) -> crate::database::Result<crate::today::calendar::CalendarSnapshot>;
```

### `today_snapshot`

```rust
fn today_snapshot(
    state: tauri::State<'_, TodayState>,
) -> crate::database::Result<crate::today::Snapshot>;
```

### `today_action`

```rust
fn today_action(
    state: tauri::State<'_, TodayState>,
    action: crate::today::Action,
) -> crate::database::Result<crate::today::Snapshot>;
```

### `database_status`

```rust
fn database_status(
    state: tauri::State<'_, DatabaseState>,
) -> crate::database::Result<crate::database::DatabaseStatus>;
```

### `app_info`

```rust
fn app_info() -> AppInfo;
```

### `module_registry_get`

```rust
fn module_registry_get(state: tauri::State<'_, DatabaseState>) -> Result<Registry>;
```

### `module_registry_save`

```rust
fn module_registry_save(
    state: tauri::State<'_, DatabaseState>,
    value: Registry,
) -> Result<Registry>;
```

### `music_snapshot`

```rust
fn music_snapshot(
    state: tauri::State<'_, DatabaseState>,
    from: String,
    until: String,
    trash: bool,
) -> Result<serde_json::Value>;
```

### `music_action`

```rust
fn music_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()>;
```

### `music_status`

```rust
fn music_status(state: tauri::State<'_, MediaState>) -> Result<MediaStatus>;
```

### `backup_status`

```rust
fn backup_status(state: tauri::State<'_, DatabaseState>) -> Result<BackupStatus>;
```

### `backup_policy_save`

```rust
fn backup_policy_save(
    state: tauri::State<'_, DatabaseState>,
    policy: backup_schedule::Policy,
) -> Result<()>;
```

### `backup_create`

```rust
fn backup_create(state: tauri::State<'_, DatabaseState>) -> Result<String>;
```

### `privacy_status`

```rust
fn privacy_status(
    state: tauri::State<'_, DatabaseState>,
) -> Result<crate::kernel::privacy::PrivacyStatus>;
```

### `privacy_disable`

```rust
fn privacy_disable(state: tauri::State<'_, DatabaseState>, delete: bool) -> Result<()>;
```

### `privacy_open_folder`

```rust
fn privacy_open_folder(state: tauri::State<'_, DatabaseState>) -> Result<()>;
```

### `private_export`

```rust
fn private_export(
    state: tauri::State<'_, DatabaseState>,
    format: String,
) -> Result<String>;
```

### `import_review`

```rust
fn import_review(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    pending: tauri::State<'_, ImportState>,
) -> Result<Option<ImportReview>>;
```

### `import_confirm`

```rust
fn import_confirm(
    state: tauri::State<'_, DatabaseState>,
    pending: tauri::State<'_, ImportState>,
    token: String,
    fingerprint: String,
) -> Result<crate::kernel::data_exchange::Review>;
```

### `restore_rehearse`

```rust
fn restore_rehearse(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
) -> Result<Option<crate::kernel::privacy::RestoreReview>>;
```

### `profile_get`

```rust
fn profile_get(state: tauri::State<'_, DatabaseState>) -> Result<Profile>;
```

### `profile_save`

```rust
fn profile_save(state: tauri::State<'_, DatabaseState>, profile: Profile) -> Result<()>;
```

### `public_projection`

```rust
fn public_projection(
    state: tauri::State<'_, DatabaseState>,
) -> Result<serde_json::Value>;
```

### `public_export`

```rust
fn public_export(
    state: tauri::State<'_, DatabaseState>,
    expected_hash: String,
) -> Result<ExportReceipt>;
```

### `public_preview`

```rust
fn public_preview(
    state: tauri::State<'_, DatabaseState>,
    expected_hash: String,
) -> Result<String>;
```

### `deep_link_initial`

```rust
fn deep_link_initial() -> Option<String>;
```

### `deep_link_register`

```rust
fn deep_link_register() -> Result<()>;
```

### `vault_snapshot`

```rust
fn vault_snapshot(state: tauri::State<'_, DatabaseState>) -> Result<serde_json::Value>;
```

### `vault_action`

```rust
fn vault_action(state: tauri::State<'_, DatabaseState>, action: Action) -> Result<()>;
```

### `vault_files`

```rust
fn vault_files(
    state: tauri::State<'_, DatabaseState>,
    id: String,
) -> Result<serde_json::Value>;
```

### `vault_pick_attachment`

```rust
fn vault_pick_attachment(
    app: tauri::AppHandle,
    state: tauri::State<'_, DatabaseState>,
    id: String,
    revision: i64,
) -> Result<()>;
```

### `vault_verify`

```rust
fn vault_verify(
    state: tauri::State<'_, DatabaseState>,
    id: String,
    open: bool,
) -> Result<String>;
```

## 领域模型与校验器

### `src/features/today/model.ts`

[Source](../../src/features/today/model.ts)

```ts
import { z } from 'zod';
export const taskInputSchema = z.object({
  title: z.string().trim().min(1).max(250),
  body: z.string().max(100000),
  due_date: z.union([z.iso.date(), z.null()]),
  estimated_minutes: z.number().int().min(1).max(1440),
  priority: z.number().int().min(0).max(3),
});
export type TaskInput = z.infer<typeof taskInputSchema>;
export type Lifecycle = 'planned' | 'active' | 'done' | 'skipped' | 'moved' | 'cancelled';
export interface Task extends TaskInput {
  id: string;
  lifecycle: Lifecycle;
  remaining_minutes: number;
  revision: number;
  deleted_at: string | null;
}
export interface FocusSession {
  id: string;
  task_id: string;
  state: 'running' | 'paused' | 'interrupted';
  elapsed_ms: number;
  paused_ms: number;
  interrupted_ms: number;
  planned_seconds: number;
  last_activity_at: string;
  interruption_reason?: string | null;
}
export interface TodaySnapshot {
  date: string;
  tasks: Task[];
  trash: Task[];
  session: FocusSession | null;
  capacity_minutes: number;
  planned_minutes: number;
  actual_seconds: number;
  journey: string | null;
  notes: { id: string; body: string }[];
  timeline: { id: string; title: string; started_at: string; actual_seconds: number }[];
  system_blocked?: boolean;
  native_hooks_ready?: boolean;
}
export type TodayAction =
  | { type: 'create'; id: string; input: TaskInput }
  | { type: 'edit'; id: string; revision: number; input: TaskInput }
  | { type: 'task_state'; id: string; revision: number; state: Exclude<Lifecycle, 'active'> }
  | { type: 'trash'; id: string; revision: number; deleted: boolean }
  | { type: 'start'; id: string; task_id: string }
  | {
      type: 'session';
      id: string;
      operation: 'pause' | 'resume' | 'finish' | 'cancel' | 'interrupt';
    }
  | { type: 'capture'; id: string; body: string }
  | { type: 'capacity'; minutes: number }
  | { type: 'calendar_create'; id: string; input: import('../calendar/model').EventInput }
  | {
      type: 'calendar_edit';
      id: string;
      revision: number;
      input: import('../calendar/model').EventInput;
    }
  | { type: 'calendar_trash'; id: string; revision: number; deleted: boolean };
export function localDate() {
  const date = new Date();
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, '0'),
    String(date.getDate()).padStart(2, '0'),
  ].join('-');
}
export function duration(seconds: number) {
  const whole = Math.max(0, Math.floor(seconds));
  return (
    String(Math.floor(whole / 60)).padStart(2, '0') + ':' + String(whole % 60).padStart(2, '0')
  );
}
```

### `src/features/calendar/model.ts`

[Source](../../src/features/calendar/model.ts)

```ts
import { z } from 'zod';
import type { Task } from '../today/model';
export type CalendarView = 'day' | 'week' | 'month' | 'agenda';
export const eventTypes = [
  'study',
  'class',
  'competition',
  'exam',
  'deadline',
  'paper',
  'personal',
] as const;
export interface EventInput {
  title: string;
  event_type: (typeof eventTypes)[number];
  event_date: string | null;
  starts_at: string | null;
  ends_at: string | null;
  task_id: string | null;
}
export interface CalendarEvent extends EventInput {
  id: string;
  revision: number;
}
export interface ActualInterval {
  id: string;
  session_id: string;
  task_id: string | null;
  title: string;
  local_date: string;
  starts_at: string;
  ends_at: string;
  elapsed_ms: number;
  state: string;
  provenance: string;
}
export interface CalendarSnapshot {
  from: string;
  until: string;
  events: CalendarEvent[];
  actual: ActualInterval[];
  tasks: Task[];
  task_options: Task[];
  trash: CalendarEvent[];
}
export const eventSchema = z
  .object({
    title: z.string().trim().min(1).max(250),
    event_type: z.enum(eventTypes),
    event_date: z.iso.date().nullable(),
    starts_at: z.iso.datetime().nullable(),
    ends_at: z.iso.datetime().nullable(),
    task_id: z.uuid().nullable(),
  })
  .refine((input) =>
    input.starts_at && input.ends_at
      ? input.event_date === null &&
        Date.parse(input.ends_at) > Date.parse(input.starts_at) &&
        Date.parse(input.ends_at) - Date.parse(input.starts_at) <= 7 * 86400000
      : input.starts_at === null && input.ends_at === null && input.event_date !== null,
  );
export function dateString(date: Date) {
  return [
    date.getFullYear(),
    String(date.getMonth() + 1).padStart(2, '0'),
    String(date.getDate()).padStart(2, '0'),
  ].join('-');
}
export function addDays(day: string, count: number) {
  const d = new Date(day + 'T12:00:00');
  d.setDate(d.getDate() + count);
  return dateString(d);
}
export function range(day: string, view: CalendarView) {
  const d = new Date(day + 'T12:00:00');
  let from = day;
  let count = 1;
  if (view === 'month') {
    d.setDate(1);
    from = addDays(dateString(d), -((d.getDay() + 6) % 7));
    count = 42;
  }
  if (view === 'week') {
    from = addDays(day, -((d.getDay() + 6) % 7));
    count = 7;
  }
  if (view === 'agenda') count = 14;
  return {
    from,
    until: addDays(from, count),
    days: Array.from({ length: count }, (_, i) => addDays(from, i)),
  };
}
export function shift(day: string, view: CalendarView, step: number) {
  if (view !== 'month') return addDays(day, step * (view === 'day' ? 1 : view === 'week' ? 7 : 14));
  const d = new Date(day + 'T12:00:00');
  const target = d.getMonth() + step;
  d.setDate(1);
  d.setMonth(target);
  return dateString(d);
}
export function localDateTime(value: string) {
  const d = new Date(value);
  return (
    dateString(d) +
    'T' +
    String(d.getHours()).padStart(2, '0') +
    ':' +
    String(d.getMinutes()).padStart(2, '0')
  );
}
export function parseLocalDateTime(value: string) {
  if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/.test(value)) throw new RangeError('INVALID_LOCAL_TIME');
  const utc = new Date(value).toISOString();
  // Date normalizes impossible dates and skipped DST times; reject that change.
  if (localDateTime(utc) !== value) throw new RangeError('INVALID_LOCAL_TIME');
  return utc;
}
export function onDay(event: CalendarEvent, day: string) {
  return (
    event.event_date === day ||
    !!(
      event.starts_at &&
      event.ends_at &&
      Date.parse(event.starts_at) < new Date(addDays(day, 1) + 'T00:00:00').getTime() &&
      Date.parse(event.ends_at) > new Date(day + 'T00:00:00').getTime()
    )
  );
}
export function timePosition(start: string, end: string, day: string) {
  const first = new Date(day + 'T00:00:00').getTime(),
    last = new Date(addDays(day, 1) + 'T00:00:00').getTime();
  const a = Math.max(first, Date.parse(start)),
    b = Math.min(last, Date.parse(end));
  return {
    top: (100 * (a - first)) / (last - first),
    height: (100 * Math.max(0, b - a)) / (last - first),
  };
}
```

### `src/features/roadmap/model.ts`

[Source](../../src/features/roadmap/model.ts)

```ts
import { z } from 'zod';
import { localDate } from '../today/model';
import { addDays } from '../calendar/model';
export type Kind = 'journey' | 'phase' | 'subject' | 'quest';
const optionalDate = z.union([z.iso.date(), z.null()]);
export const detailsSchema = z
  .object({
    description: z.string().max(100000),
    category: z.string().max(100),
    status: z.enum(['active', 'paused', 'done']),
    start_date: optionalDate,
    target_date: optionalDate,
    minimum_minutes: z.number().int().min(0).max(10080),
    preferred_minutes: z.number().int().min(0).max(10080),
    maximum_minutes: z.number().int().min(0).max(10080),
    priority: z.number().int().min(0).max(3),
    color: z.enum(['sage', 'blue', 'lavender', 'warm']),
    icon: z.string().max(50),
    progress_mode: z.enum(['manual', 'tasks']),
    progress: z.number().int().min(0).max(100),
    parent_id: z.union([z.uuid(), z.null()]),
    preparation_phase: z.string().max(1000),
    position: z.number().int().min(0).max(10000),
    planning_target: z.union([z.number().int().min(0).max(750), z.null()]),
  })
  .refine(
    (d) =>
      d.minimum_minutes <= d.preferred_minutes &&
      d.preferred_minutes <= d.maximum_minutes &&
      (!d.start_date || !d.target_date || d.start_date <= d.target_date),
  );
export type Details = z.infer<typeof detailsSchema>;
export const defaults: Details = {
  description: '',
  category: 'study',
  status: 'active',
  start_date: null,
  target_date: null,
  minimum_minutes: 0,
  preferred_minutes: 0,
  maximum_minutes: 10080,
  priority: 1,
  color: 'sage',
  icon: 'route',
  progress_mode: 'manual',
  progress: 0,
  parent_id: null,
  preparation_phase: '',
  position: 0,
  planning_target: null,
};
export interface Entity {
  id: string;
  kind: Kind;
  title: string;
  journey_id: string | null;
  data: Details;
  revision: number;
  deleted_at: string | null;
  archived_at: string | null;
}
export const entitySchema = z.object({
  title: z.string().trim().min(1).max(250),
  journey_id: z.union([z.uuid(), z.null()]),
  data: detailsSchema,
});
export type EntityInput = z.infer<typeof entitySchema>;
export interface TaskOptions {
  earliest_start: string | null;
  minimum_session: number;
  preferred_session: number;
  splittable: boolean;
  energy_level: 'low' | 'normal' | 'high';
  fixed_time: boolean;
}
export const taskOptionsSchema = z
  .object({
    earliest_start: optionalDate,
    minimum_session: z.number().int().min(1).max(1440),
    preferred_session: z.number().int().min(1).max(1440),
    splittable: z.boolean(),
    energy_level: z.enum(['low', 'normal', 'high']),
    fixed_time: z.boolean(),
  })
  .refine((d) => d.minimum_session <= d.preferred_session);
export interface PlanningTask {
  id: string;
  title: string;
  remaining_minutes: number;
  priority: number;
  deadline: string | null;
  journey_id: string | null;
  subject_id: string | null;
  quest_id: string | null;
  revision: number;
  options: TaskOptions;
}
export interface Preferences {
  weekday_minutes: number;
  weekend_minutes: number;
  side_quest_max: number;
  planning_target: number;
  revision: number;
}
export interface CapacityDay {
  date: string;
  minutes: number;
  status: keyof typeof import('./text').roadmapText.dayStatuses;
}
export interface Allocation {
  task_id: string;
  date: string;
  minutes: number;
}
export interface DayBudget {
  date: string;
  capacity: number;
  fixed_minutes: number;
  available: number;
  status: string;
}
export interface Plan {
  days: DayBudget[];
  allocations: Allocation[];
  unscheduled: [string, number][];
  warnings: string[];
  reason: string;
}
export interface Decision {
  id: string;
  week_start: string;
  status: 'proposed' | 'accepted' | 'modified' | 'rejected' | 'undone';
  reason: string;
  before: Allocation[];
  after: Plan;
  created_at: string;
  updated_at: string;
}
export interface RuleInput {
  journey_id: string | null;
  school: string;
  college: string;
  major: string;
  year: number;
  written_exam: string;
  interview: string;
  source_url: string;
  body: string;
  notes: string;
}
export interface Rule {
  id: string;
  input: RuleInput;
  captured_at: string;
  content_hash: string;
  deleted_at: string | null;
}
export interface RoadmapSnapshot {
  week_start: string;
  entities: Entity[];
  tasks: PlanningTask[];
  preferences: Preferences;
  days: CapacityDay[];
  blocks: Allocation[];
  decisions: Decision[];
  rules: Rule[];
  budgets: DayBudget[];
}
export type RoadmapAction =
  | { type: 'save'; id: string; kind: Kind; revision: number | null; input: EntityInput }
  | {
      type: 'lifecycle';
      id: string;
      kind: Kind;
      revision: number;
      operation: 'trash' | 'restore' | 'archive' | 'unarchive';
    }
  | { type: 'template'; id: string }
  | { type: 'preferences'; input: Preferences }
  | { type: 'capacity'; input: CapacityDay }
  | {
      type: 'task_plan';
      id: string;
      revision: number;
      journey_id: string | null;
      subject_id: string | null;
      quest_id: string | null;
      input: TaskOptions;
    }
  | { type: 'propose'; id: string }
  | {
      type: 'decide';
      id: string;
      operation: 'accept' | 'modify' | 'reject' | 'undo';
      allocations: Allocation[] | null;
    }
  | { type: 'rule'; id: string; input: RuleInput }
  | { type: 'rule_trash'; id: string; deleted: boolean };
export function weekStart(value = localDate()) {
  const day = new Date(value + 'T12:00:00');
  return addDays(value, -((day.getDay() + 6) % 7));
}
export function isLive(entity: Entity, all: Entity[]) {
  return (
    !entity.deleted_at &&
    !entity.archived_at &&
    (!entity.journey_id ||
      all.some((e) => e.id === entity.journey_id && !e.deleted_at && !e.archived_at))
  );
}
```

### `src/features/journal/model.ts`

[Source](../../src/features/journal/model.ts)

```ts
import { z } from 'zod';
export const saveSchema = z.object({
  id: z.uuid(),
  client_id: z.uuid(),
  sequence: z.number().int().positive(),
  revision: z.number().int().nonnegative(),
  title: z.string().max(1000),
  body: z.string().max(1000000),
  date: z.iso.date(),
  entry_type: z.enum([
    'reflection',
    'quick_note',
    'study_note',
    'memory',
    'mood',
    'music_note',
    'competition_note',
    'research_note',
  ]),
  tags: z.array(z.string().trim().min(1).max(100)).max(30),
  journey_id: z.uuid().nullable(),
  task_id: z.uuid().nullable(),
  music_id: z.uuid().nullable(),
  publish: z.boolean(),
});
export type Save = z.infer<typeof saveSchema>;
export type Entry = {
  id: string;
  title: string;
  body: string;
  date: string;
  entry_type: string;
  is_draft: boolean;
  revision: number;
  journey_id: string | null;
  task_id: string | null;
  music_id: string | null;
  tags: string[];
  deleted_at: string | null;
  updated_at: string;
};
export type Choice = { id: string; title: string };
export type Filter = {
  query: string;
  from: string;
  until: string;
  entry_type: string;
  tag: string;
  mode: string;
  page: number;
};
export type JournalPage = {
  entries: Entry[];
  total: number;
  page: number;
  recoveries: { payload: Save; updated_at: string }[];
  journeys: Choice[];
  tasks: Choice[];
  music: Choice[];
};
export type Attachment = {
  id: string;
  name: string;
  url: string | null;
  size_bytes: number | null;
  sha256: string | null;
};
export const emptyFilter: Filter = {
  query: '',
  from: '',
  until: '',
  entry_type: '',
  tag: '',
  mode: 'published',
  page: 0,
};
export function localDate() {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
export function fresh(quick = false): Save {
  return {
    id: crypto.randomUUID(),
    client_id: crypto.randomUUID(),
    sequence: 1,
    revision: 0,
    title: '',
    body: '',
    date: localDate(),
    entry_type: quick ? 'quick_note' : 'reflection',
    tags: [],
    journey_id: null,
    task_id: null,
    music_id: null,
    publish: false,
  };
}
export function fromEntry(e: Entry): Save {
  return {
    ...fresh(),
    id: e.id,
    revision: e.revision,
    title: e.title,
    body: e.body,
    date: e.date,
    entry_type: e.entry_type as Save['entry_type'],
    tags: e.tags,
    journey_id: e.journey_id,
    task_id: e.task_id,
    music_id: e.music_id,
    publish: !e.is_draft,
  };
}
export function asCopy(s: Save): Save {
  return { ...s, id: crypto.randomUUID(), revision: 0, sequence: s.sequence + 1, publish: false };
}
export function pendingKey(quick: boolean, client?: string) {
  return `lumen.journal.pending.${quick ? 'capture' : 'main'}` + (client ? '.' + client : '');
}
export function readPending(quick: boolean): Save | null {
  try {
    const prefix = pendingKey(quick);
    const key = Object.keys(localStorage).find((x) => x.startsWith(prefix + '.')) ?? prefix;
    const raw = localStorage.getItem(key);
    return raw ? saveSchema.parse(JSON.parse(raw)) : null;
  } catch {
    return null;
  }
}
```

### `src/features/coding/model.ts`

[Source](../../src/features/coding/model.ts)

```ts
import { translate } from '../../lib/locale';
export const platforms = [
  'Luogu',
  'NowCoder',
  'Codeforces',
  'LeetCode',
  'AtCoder',
  'Other',
] as const;
export const difficulties = ['unknown', 'easy', 'medium', 'hard'] as const;
export const outcomes = [
  'accepted',
  'wrong_answer',
  'time_limit',
  'runtime_error',
  'compile_error',
  'unfinished',
] as const;
export type ProblemInput = {
  title: string;
  platform: string;
  external_id: string;
  source_url: string;
  difficulty: string;
  tags: string[];
  notes: string;
  journey_id: string | null;
  task_id: string | null;
  competition_id: string | null;
};
export type AttemptInput = {
  problem_id: string;
  language: string;
  outcome: string;
  notes: string;
  attempted_on: string;
  started_at: string;
  manual_seconds: number;
  focus_session_id: string | null;
};
export type ContestInput = {
  title: string;
  platform: string;
  event_date: string;
  source_url: string;
  participated: boolean;
  notes: string;
};
export type Problem = ProblemInput & {
  id: string;
  revision: number;
  deleted_at: string | null;
  attempt_count: number;
  solved: boolean;
  first_attempt: string | null;
  solved_at: string | null;
  language: string | null;
  manual_seconds: number;
  focus_seconds: number;
};
export type Attempt = AttemptInput & {
  id: string;
  revision: number;
  deleted_at: string | null;
  focus_seconds: number;
};
export type Contest = ContestInput & { id: string; revision: number; deleted_at: string | null };
export type Choice = { id: string; title: string; task_id: string | null };
export type Analytics = {
  from: string;
  until: string;
  problems: number;
  solved: number;
  attempts: number;
  first_pass_numerator: number;
  first_pass_denominator: number;
  manual_seconds: number;
  focus_seconds: number;
  contests: number;
  difficulty: { label: string; count: number }[];
  topics: { label: string; count: number }[];
};
export type Snapshot = {
  problems: Problem[];
  total: number;
  attempts: Attempt[];
  contests: Contest[];
  journeys: Choice[];
  tasks: Choice[];
  focus: Choice[];
  analytics: Analytics;
};
export type Filter = {
  week: string;
  query: string;
  platform: string;
  status: string;
  tag: string;
  trash: boolean;
  page: number;
  selected: string | null;
};
type DraftInput =
  | { type: 'problem'; id: string; revision: number | null; input: ProblemInput }
  | { type: 'attempt'; id: string; revision: number | null; input: AttemptInput }
  | { type: 'contest'; id: string; revision: number | null; input: ContestInput };
export type Action =
  | (Draft & { request_id: string })
  | {
      type: 'trash';
      request_id: string;
      id: string;
      kind: string;
      revision: number;
      restore: boolean;
    };
export const pendingKey = 'lumen.coding.pending';
export type Draft = DraftInput & { request_id?: string };
export function draftKey(d: Draft) {
  return `${pendingKey}.${d.type}.${d.id}`;
}
export function normalize(d: Draft): Draft {
  if (d.type === 'problem') {
    const {
      title,
      platform,
      external_id,
      source_url,
      difficulty,
      tags,
      notes,
      journey_id,
      task_id,
      competition_id,
    } = d.input;
    return {
      ...d,
      input: {
        title,
        platform,
        external_id,
        source_url,
        difficulty,
        tags,
        notes,
        journey_id,
        task_id,
        competition_id,
      },
    };
  }
  if (d.type === 'attempt') {
    const {
      problem_id,
      language,
      outcome,
      notes,
      attempted_on,
      started_at,
      manual_seconds,
      focus_session_id,
    } = d.input;
    return {
      ...d,
      input: {
        problem_id,
        language,
        outcome,
        notes,
        attempted_on,
        started_at,
        manual_seconds,
        focus_session_id,
      },
    };
  }
  const { title, platform, event_date, source_url, participated, notes } = d.input;
  return { ...d, input: { title, platform, event_date, source_url, participated, notes } };
}
export function today() {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
export function fresh(type: Draft['type'], problem_id = ''): Draft {
  const id = crypto.randomUUID();
  if (type === 'problem')
    return {
      type,
      id,
      revision: null,
      input: {
        title: '',
        platform: 'LeetCode',
        external_id: '',
        source_url: '',
        difficulty: 'unknown',
        tags: [],
        notes: '',
        journey_id: null,
        task_id: null,
        competition_id: null,
      },
    };
  if (type === 'contest')
    return {
      type,
      id,
      revision: null,
      input: {
        title: '',
        platform: 'Other',
        event_date: today(),
        source_url: '',
        participated: false,
        notes: '',
      },
    };
  return {
    type,
    id,
    revision: null,
    input: {
      problem_id,
      language: '',
      outcome: 'unfinished',
      notes: '',
      attempted_on: today(),
      started_at: new Date().toISOString(),
      manual_seconds: 0,
      focus_session_id: null,
    },
  };
}
export function readDraft(): Draft | null {
  try {
    const key = Object.keys(localStorage).find((k) => k.startsWith(pendingKey + '.')) ?? pendingKey;
    const v = JSON.parse(localStorage.getItem(key) ?? 'null');
    return v &&
      ['problem', 'attempt', 'contest'].includes(v.type) &&
      typeof v.id === 'string' &&
      v.input
      ? v
      : null;
  } catch {
    return null;
  }
}
export function safeLink(value: string) {
  try {
    const u = new URL(value);
    return ['http:', 'https:'].includes(u.protocol) && !u.username && !u.password ? u.href : null;
  } catch {
    return null;
  }
}
export function firstPass(a: Pick<Analytics, 'first_pass_numerator' | 'first_pass_denominator'>) {
  return a.first_pass_denominator === 0
    ? translate('暂无样本')
    : `${((100 * a.first_pass_numerator) / a.first_pass_denominator).toFixed(1)}% (${a.first_pass_numerator}/${a.first_pass_denominator})`;
}
export function minutes(seconds: number) {
  return `${(seconds / 60).toFixed(1)} 分钟`;
}
```

### `src/features/insights/model.ts`

[Source](../../src/features/insights/model.ts)

```ts
import { translate } from '../../lib/locale';
export type Preferences = {
  weights: number[];
  thresholds: number[];
  stop_words: string[];
  memories: boolean;
};
export type Day = {
  date: string;
  deep_minutes: number;
  coding_minutes: number;
  tasks_completed: number;
  journal_count: number;
  problem_count: number;
  music_count: number;
  planned_minutes: number;
  capacity_minutes: number;
  status: string;
  status_revision: number;
  intensity: number;
};
export type RecordLink = { kind: string; id: string; date: string; title: string };
export type Word = { word: string; count: number; records: RecordLink[] };
export type LabelValue = {
  label: string;
  minutes?: number;
  total?: number;
  completed?: number;
  count?: number;
  seconds?: number;
  artist?: string;
  album?: string;
};
export type Facts = {
  from: string;
  until: string;
  days: Day[];
  heatmap: Day[];
  preferences: Preferences;
  preferences_revision: number;
  words: Word[];
  subjects: LabelValue[];
  journeys: LabelValue[];
  music: LabelValue[];
  topics: LabelValue[];
  records: RecordLink[];
  task_summary: { total: number; completed: number; unfinished: number; moved: number };
  consistency: {
    days7: { active: number; eligible: number; protected: number };
    days28: { active: number; eligible: number; protected: number };
  };
  previous: {
    deep_minutes: number;
    coding_minutes: number;
    planned_minutes: number;
    tasks_completed: number;
    journals: number;
  };
};
export type Review = {
  week: string;
  facts: Facts;
  body: string;
  next_focus: string;
  revision: number;
};
export type Monthly = {
  highlights?: RecordLink[];
  version: number;
  visibility: 'private';
  month: string;
  from: string;
  until: string;
  closed: boolean;
  generated_at: string;
  totals: {
    deep_minutes: number;
    coding_minutes: number;
    planned_minutes: number;
    journals: number;
    tasks_completed: number;
  };
  days: Day[];
  subjects: LabelValue[];
  topics: LabelValue[];
  music: LabelValue[];
  words: { word: string; count: number }[];
  counts: { competitions: number; achievements: number; projects: number; photos: number };
  task_summary: Facts['task_summary'];
};
export type Action = { request_id: string } & (
  | { type: 'preferences'; revision: number; value: Preferences }
  | { type: 'day'; date: string; revision: number; status: string }
  | { type: 'review'; week: string; revision: number; body: string; next_focus: string }
);
export function today() {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
export function shift(day: string, days: number) {
  const d = new Date(day + 'T12:00:00');
  d.setDate(d.getDate() + days);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}
export function ratio(n: number, d: number) {
  return d > 0 ? `${Math.round((n / d) * 100)}%` : translate('暂无样本');
}
export function minutes(n: number) {
  return `${n.toFixed(1)} 分钟`;
}
export const statuses: Record<string, string> = {
  normal: translate('普通日'),
  rest: translate('休息日'),
  sick: translate('生病日'),
  exam: translate('考试日'),
  travel: translate('旅行日'),
  paper_deadline: translate('论文截止日'),
  special: translate('特殊日'),
  competition: translate('竞赛日'),
  paper_sprint: translate('论文冲刺日'),
  other: translate('其他'),
};
export function recordHref(r: RecordLink) {
  if (r.kind === 'journal') return `/journal?entry=${encodeURIComponent(r.id)}`;
  if (r.kind === 'review') return `/insights?week=${r.date}&view=review`;
  if (r.kind === 'problem' || r.kind === 'contest') return '/coding';
  if (r.kind === 'event') return '/calendar';
  if (r.kind === 'photo' || r.kind === 'project' || r.kind === 'achievement') return '/vault';
  return r.kind === 'music' ? '/insights?view=music' : '/focus';
}
```

### `src/features/music/model.ts`

[Source](../../src/features/music/model.ts)

```ts
import { translate } from '../../lib/locale';
export interface Preferences {
  enabled: boolean;
  threshold_seconds: number;
  allowed_app: string;
  revision: number;
}
export interface Play {
  id: string;
  revision: number;
  title: string;
  artist: string;
  album: string;
  platform: string;
  played_at: string;
  duration_seconds: number;
  source: 'manual' | 'windows-media';
  source_app: string;
  journal_id: string | null;
  journal_title: string | null;
}
export interface Group {
  title: string;
  artist: string;
  album: string;
  count: number;
  seconds: number;
}
export interface Snapshot {
  items: Play[];
  cloud: Group[];
  preferences: Preferences;
  journals: { id: string; title: string }[];
  totals: { count: number; seconds: number };
  visibility: 'private';
}
export interface MediaStatus {
  state: 'disabled' | 'observing' | 'manual-fallback';
  metadata_available: boolean;
  source_app: string;
  error: string | null;
}
export function musicCloud(rows: Group[], mode: 'title' | 'artist' | 'album') {
  const grouped = new Map<string, { label: string; count: number; seconds: number }>();
  for (const row of rows) {
    const label =
      (mode === 'title' ? `${row.title}${row.artist ? ' · ' + row.artist : ''}` : row[mode]) ||
      translate('未填写');
    const value = grouped.get(label) ?? { label, count: 0, seconds: 0 };
    value.count += row.count;
    value.seconds += row.seconds;
    grouped.set(label, value);
  }
  return [...grouped.values()].sort((a, b) => b.count - a.count || a.label.localeCompare(b.label));
}
export function localDateTime(utc: string) {
  const d = new Date(utc);
  d.setMinutes(d.getMinutes() - d.getTimezoneOffset());
  return d.toISOString().slice(0, 16);
}
```

### `src/features/vault/model.ts`

[Source](../../src/features/vault/model.ts)

```ts
import { translate } from '../../lib/locale';
export const categories = {
  award: translate('奖项'),
  competition: translate('比赛'),
  paper: translate('论文'),
  project: translate('项目'),
  certificate: translate('证书'),
  open_source: translate('开源'),
  course_project: translate('课程项目'),
  research: translate('研究'),
  presentation: translate('演示'),
  other: translate('其他'),
};
export interface Artifact {
  id: string;
  title: string;
  description: string;
  kind: string;
  artifact_date: string | null;
  role: string;
  organization: string;
  skills: string[];
  technologies: string[];
  repository_url: string;
  project_url: string;
  paper_url: string;
  journey_id: string | null;
  competition_id: string | null;
  visibility: string;
  revision: number;
  archived_at: string | null;
  deleted_at: string | null;
}
export interface Snapshot {
  artifacts: Artifact[];
  journeys: { id: string; title: string }[];
  competitions: { id: string; title: string }[];
  limit: number;
}
export interface Attachment {
  id: string;
  name: string;
  size_bytes: number;
  sha256: string;
  mime_type: string;
}
export function emptyArtifact(): Artifact {
  return {
    id: crypto.randomUUID(),
    title: '',
    description: '',
    kind: 'project',
    artifact_date: null,
    role: '',
    organization: '',
    skills: [],
    technologies: [],
    repository_url: '',
    project_url: '',
    paper_url: '',
    journey_id: null,
    competition_id: null,
    visibility: 'private',
    revision: 0,
    archived_at: null,
    deleted_at: null,
  };
}
export function portfolio(records: Artifact[]) {
  return records.filter((a) => a.visibility === 'public' && !a.deleted_at && !a.archived_at);
}
export function tags(text: string) {
  return [
    ...new Set(
      text
        .split(/[,，]/)
        .map((s) => s.trim())
        .filter(Boolean),
    ),
  ];
}
```

### `src/features/profile/model.ts`

[Source](../../src/features/profile/model.ts)

```ts
export interface Profile {
  display_name: string;
  headline: string;
  about: string;
  links: { label: string; url: string }[];
  public: boolean;
  revision: number;
}
export const emptyProfile: Profile = {
  display_name: '',
  headline: '',
  about: '',
  links: [],
  public: false,
  revision: 0,
};
export interface Projection {
  review_hash: string;
  schema_version: string;
  exported_at: string;
  profile: Profile | null;
  artifacts: { id: string; title: string; kind: string; public: true }[];
}
export interface Receipt {
  id: string;
  directory: string;
  sha256: string;
  artifacts: number;
}
```

### `src/features/desktop/model.ts`

[Source](../../src/features/desktop/model.ts)

```ts
export interface DesktopPreferences {
  close_to_tray: boolean;
  notifications_enabled: boolean;
  focus_finished: boolean;
  upcoming_event: boolean;
  weekly_review: boolean;
  backup_failure: boolean;
  revision: number;
}
export interface DesktopStatus {
  preferences: DesktopPreferences;
  autostart: boolean | null;
  notification_permission: string;
  runtime: { tray_ready: boolean; error_code: string | null };
  onboarding: boolean;
}
```

### `src/kernel/modules/model.ts`

[Source](../../src/kernel/modules/model.ts)

```ts
import { translate } from '../../lib/locale';
import { z } from 'zod';
export const manifestSchema = z
  .object({
    id: z
      .string()
      .regex(/^[a-z][a-z0-9]*(\.[a-z][a-z0-9-]*)+$/)
      .max(80),
    name: z.object({ 'zh-CN': z.string().min(1).max(80), en: z.string().min(1).max(80) }).strict(),
    version: z.string().regex(/^\d+\.\d+\.\d+$/),
    api_version: z.literal('1.0'),
    layer: z.enum(['kernel', 'official', 'community', 'theme']),
    route: z.string().regex(/^\/[a-z0-9/-]*$/),
    icon: z.string().max(40),
    required: z.boolean(),
    status: z.enum(['stable', 'module', 'legacy-bridge', 'planned']),
    permissions: z.array(z.string()).max(16),
  })
  .strict();
export type Manifest = z.infer<typeof manifestSchema>;
export interface ModulePreference {
  id: string;
  enabled: boolean;
  hidden: boolean;
  pinned: boolean;
  order: number;
}
export interface RegistryState {
  revision: number;
  items: ModulePreference[];
}
export const registryStateSchema = z
  .object({
    revision: z.number().int().nonnegative(),
    items: z
      .array(
        z
          .object({
            id: z.string(),
            enabled: z.boolean(),
            hidden: z.boolean(),
            pinned: z.boolean(),
            order: z.number().int().min(0).max(1000),
          })
          .strict(),
      )
      .max(32),
  })
  .strict();
export const moduleText = {
  heading: translate('模块管理'),
  hint: translate(
    '启用控制页面和模块入口；隐藏只改变侧栏。置顶模块优先显示。拖动排列，或用上下按钮。关闭模块不会删除记录。',
  ),
  enabled: translate('启用'),
  hidden: translate('隐藏'),
  pinned: translate('置顶'),
  up: translate('上移'),
  down: translate('下移'),
  save: translate('保存模块配置'),
  error: translate('保存失败或版本已变化；当前选择已保留，请重新载入后重试。'),
  reload: translate('重新载入模块配置'),
  disabled: translate('此模块已停用，可在设置中重新启用。'),
  saved: translate('模块配置已保存'),
  required: translate('核心模块不能停用；安全系统始终属于 Kernel。'),
};
```

### `src/kernel/plan/exchange.ts`

[Source](../../src/kernel/plan/exchange.ts)

```ts
import { z } from 'zod';
export const planExchangeSchema = z
  .object({
    schema_version: z.literal('1.0'),
    request_id: z.uuid(),
    basis_hash: z.string().regex(/^[0-9a-fA-F]{64}$/),
    source: z.enum(['manual', 'external-proposal', 'local-proposal']),
    capacity_minutes: z.number().int().min(0).max(10080),
    items: z
      .array(
        z
          .object({
            task_id: z.uuid(),
            date: z.iso.date(),
            minutes: z.number().int().min(1).max(1440),
          })
          .strict(),
      )
      .max(500),
  })
  .strict()
  .refine((p) => p.items.reduce((n, i) => n + i.minutes, 0) <= p.capacity_minutes, {
    message: 'CAPACITY_EXCEEDED',
  })
  .refine((p) => new Set(p.items.map((i) => i.task_id + '|' + i.date)).size === p.items.length, {
    message: 'DUPLICATE_BLOCK',
  });
export type PlanExchange = z.infer<typeof planExchangeSchema>;
export const planContextSchema = z
  .object({
    schema_version: z.literal('1.0'),
    request_id: z.uuid(),
    basis_hash: z.string().regex(/^[0-9a-fA-F]{64}$/),
    tasks: z
      .array(
        z
          .object({
            task_id: z.uuid(),
            revision: z.number().int().nonnegative(),
            estimate_minutes: z.number().int().min(1).max(1440),
            actual_ms: z.number().int().nonnegative(),
            due_date: z.iso.date(),
            label: z.string().max(1000).optional(),
          })
          .strict(),
      )
      .max(500),
    capacities: z
      .array(z.object({ date: z.iso.date(), minutes: z.number().int().min(0).max(1440) }).strict())
      .max(31),
  })
  .strict();
export type PlanContext = z.infer<typeof planContextSchema>;
// Data-only contract. No AI invocation or automatic acceptance exists.
```


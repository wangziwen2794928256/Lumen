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

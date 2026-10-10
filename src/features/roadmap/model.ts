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

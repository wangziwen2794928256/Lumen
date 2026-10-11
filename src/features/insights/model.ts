import { locale, translate } from '../../lib/locale';
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
  return `${n.toFixed(1)} ${locale === 'en' ? 'minutes' : '分钟'}`;
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

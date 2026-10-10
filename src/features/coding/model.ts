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
    ? '暂无样本'
    : `${((100 * a.first_pass_numerator) / a.first_pass_denominator).toFixed(1)}% (${a.first_pass_numerator}/${a.first_pass_denominator})`;
}
export function minutes(seconds: number) {
  return `${(seconds / 60).toFixed(1)} 分钟`;
}

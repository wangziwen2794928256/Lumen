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

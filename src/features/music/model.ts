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
      '未填写';
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

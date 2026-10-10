import { useCallback, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { musicApi as api } from '../../features/music/api';
import { musicText as t } from '../../features/music/text';
import {
  localDateTime,
  musicCloud,
  type Snapshot,
  type Play,
  type MediaStatus,
} from '../../features/music/model';
import { today, shift } from '../../features/insights/model';
const blank = () => ({
  title: '',
  artist: '',
  album: '',
  platform: '',
  played_at: localDateTime(new Date().toISOString()),
  duration_seconds: '180',
  journal_id: '',
});
export function MusicPage() {
  const [range, setRange] = useState({ from: shift(today(), -6), until: today() }),
    [applied, setApplied] = useState(range),
    [bin, setBin] = useState(false),
    [data, setData] = useState<Snapshot | null>(null),
    [status, setStatus] = useState<MediaStatus | null>(null),
    [draft, setDraft] = useState(blank),
    [editing, setEditing] = useState<Play | null>(null),
    [error, setError] = useState(''),
    [message, setMessage] = useState(''),
    [busy, setBusy] = useState(false),
    [mode, setMode] = useState<'title' | 'artist' | 'album'>('title');
  const reload = useCallback(async () => {
    if (!api.available()) return;
    try {
      setData(await api.snapshot(applied.from, applied.until, bin));
      setError('');
    } catch {
      setError(t.error);
    }
  }, [applied, bin]);
  useEffect(() => {
    let live = true;
    if (api.available())
      void api
        .snapshot(applied.from, applied.until, bin)
        .then((v) => {
          if (live) {
            setData(v);
            setError('');
          }
        })
        .catch(() => {
          if (live) setError(t.error);
        });
    return () => {
      live = false;
    };
  }, [applied, bin]);
  useEffect(() => {
    if (!api.available()) return;
    let live = true;
    const poll = () =>
      void api
        .status()
        .then((v) => {
          if (live) setStatus(v);
        })
        .catch(() => {
          if (live)
            setStatus({
              state: 'manual-fallback',
              source_app: '',
              metadata_available: false,
              error: null,
            });
        });
    poll();
    const timer = setInterval(poll, 3000);
    return () => {
      live = false;
      clearInterval(timer);
    };
  }, []);
  async function act(action: Record<string, unknown>, success: string) {
    setBusy(true);
    setError('');
    setMessage('');
    try {
      await api.action({ ...action, request_id: crypto.randomUUID() });
      await reload();
      setMessage(success);
      return true;
    } catch {
      setError(t.error);
      return false;
    } finally {
      setBusy(false);
    }
  }
  const disabled = busy || !api.available();
  return (
    <div className="music-page">
      <div className="page-heading">
        <div>
          <p className="eyebrow">MUSIC / {t.heading}</p>
          <h1>{t.heading}</h1>
          <p className="intro">{t.intro}</p>
        </div>
      </div>
      <p className="secondary-text">{t.private}</p>
      {!api.available() && <p>{t.browser}</p>}
      {error && <p role="alert">{error}</p>}
      {message && <p role="status">{message}</p>}
      <form
        className="music-controls"
        onSubmit={(e) => {
          e.preventDefault();
          setApplied({ ...range });
        }}
      >
        <label>
          {t.from}
          <input
            type="date"
            required
            value={range.from}
            onChange={(e) => setRange({ ...range, from: e.target.value })}
          />
        </label>
        <label>
          {t.until}
          <input
            type="date"
            required
            value={range.until}
            onChange={(e) => setRange({ ...range, until: e.target.value })}
          />
        </label>
        <Button type="submit">{t.filter}</Button>
        <Button type="button" variant="secondary" onClick={() => void reload()}>
          {t.reload}
        </Button>
        <label className="music-check">
          <input type="checkbox" checked={bin} onChange={(e) => setBin(e.target.checked)} />
          {t.bin}
        </label>
      </form>
      <div className="music-grid">
        <Card>
          <h2>{editing ? t.edit : t.manual}</h2>
          <form
            className="music-form"
            onSubmit={async (e) => {
              e.preventDefault();
              const ok = await act(
                {
                  type: 'save',
                  id: editing?.id ?? null,
                  revision: editing?.revision ?? 0,
                  ...draft,
                  played_at: new Date(draft.played_at).toISOString(),
                  duration_seconds: Number(draft.duration_seconds),
                  journal_id: draft.journal_id || null,
                },
                t.saved,
              );
              if (ok) {
                setDraft(blank());
                setEditing(null);
              }
            }}
          >
            <div className="music-fields">
              {(['title', 'artist', 'album', 'platform'] as const).map((key) => (
                <label key={key}>
                  {t[key]}
                  <input
                    required={key === 'title'}
                    maxLength={300}
                    value={draft[key]}
                    onChange={(e) => setDraft({ ...draft, [key]: e.target.value })}
                  />
                </label>
              ))}
              <label>
                {t.played}
                <input
                  type="datetime-local"
                  required
                  value={draft.played_at}
                  onChange={(e) => setDraft({ ...draft, played_at: e.target.value })}
                />
              </label>
              <label>
                {t.duration}
                <input
                  type="number"
                  min={0}
                  max={86400}
                  required
                  value={draft.duration_seconds}
                  onChange={(e) => setDraft({ ...draft, duration_seconds: e.target.value })}
                />
              </label>
              <label>
                {t.journal}
                <select
                  aria-label={t.journal}
                  value={draft.journal_id}
                  onChange={(e) => setDraft({ ...draft, journal_id: e.target.value })}
                >
                  <option value="">不关联</option>
                  {data?.journals.map((j) => (
                    <option key={j.id} value={j.id}>
                      {j.title || '无标题日记'}
                    </option>
                  ))}
                </select>
              </label>
            </div>
            <div className="music-actions">
              <Button type="submit" disabled={disabled}>
                {t.save}
              </Button>
              {editing && (
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => {
                    setEditing(null);
                    setDraft(blank());
                  }}
                >
                  {t.cancel}
                </Button>
              )}
            </div>
          </form>
        </Card>
        <Card>
          <h2>{t.cloud}</h2>
          <div className="music-controls">
            <label>
              云类型
              <select value={mode} onChange={(e) => setMode(e.target.value as typeof mode)}>
                <option value="title">Song</option>
                <option value="artist">Artist</option>
                <option value="album">Album</option>
              </select>
            </label>
            <p>
              {data?.totals?.count ?? 0} 次 · {Math.round((data?.totals?.seconds ?? 0) / 60)} 分钟
            </p>
          </div>
          <div className="music-cloud">
            {musicCloud(data?.cloud ?? [], mode)
              .slice(0, 50)
              .map((g) => (
                <span
                  key={g.label}
                  style={{ fontSize: Math.min(1.7, 1 + Math.log2(g.count + 1) / 8) + 'rem' }}
                >
                  {g.label}
                  <small>
                    {g.count} 次 · {Math.round(g.seconds / 60)} 分钟
                  </small>
                </span>
              ))}
          </div>
          {!data?.cloud.length && <p>{t.empty}</p>}
          <p className="secondary-text">{t.previewLimit}</p>
        </Card>
      </div>
      <Card>
        <h2>{t.history}</h2>
        <ul className="music-history">
          {data?.items.map((p) => (
            <li key={p.id}>
              <div>
                <strong>{p.title}</strong>
                <p>
                  {p.artist || t.unknown} · {p.album || t.unknown} · {p.platform || t.unknown}
                </p>
                <p>
                  {new Date(p.played_at).toLocaleString('zh-CN')} · {p.duration_seconds} 秒 ·{' '}
                  {p.source === 'manual' ? '手动' : p.source_app}
                </p>
                {p.journal_title && (
                  <Link to={'/journal?entry=' + p.journal_id}>
                    {t.journal}：{p.journal_title}
                  </Link>
                )}
              </div>
              <div className="music-actions">
                {!bin && p.source === 'manual' && (
                  <Button
                    variant="secondary"
                    disabled={disabled}
                    onClick={() => {
                      setEditing(p);
                      setDraft({
                        title: p.title,
                        artist: p.artist,
                        album: p.album,
                        platform: p.platform,
                        played_at: localDateTime(p.played_at),
                        duration_seconds: String(p.duration_seconds),
                        journal_id: p.journal_id ?? '',
                      });
                      document.querySelector('.music-form')?.scrollIntoView({ block: 'center' });
                    }}
                  >
                    {t.edit}
                  </Button>
                )}
                <Button
                  variant="secondary"
                  disabled={disabled}
                  onClick={() =>
                    void act(
                      { type: 'trash', id: p.id, revision: p.revision, restore: bin },
                      bin ? '已恢复' : '已移入音乐回收站',
                    )
                  }
                >
                  {bin ? t.restore : t.trash}
                </Button>
              </div>
            </li>
          ))}
        </ul>
        {!data?.items.length && <p>{api.available() && !data ? t.loading : t.empty}</p>}
      </Card>
      <Card>
        <h2>{t.listen}</h2>
        <p>{t.hint}</p>
        <p>{t.permission}</p>
        <p>{t.paused}</p>
        <p>{t.limits}</p>
        {data && (
          <form
            className="music-form"
            onSubmit={(e) => {
              e.preventDefault();
              void act({ type: 'preferences', preferences: data.preferences }, t.settingsSaved);
            }}
          >
            <div className="music-fields">
              <label className="music-check">
                <input
                  type="checkbox"
                  checked={data.preferences.enabled}
                  onChange={(e) =>
                    setData({
                      ...data,
                      preferences: { ...data.preferences, enabled: e.target.checked },
                    })
                  }
                />
                {t.enable}
              </label>
              <label>
                {t.threshold}
                <input
                  type="number"
                  required
                  min={5}
                  max={600}
                  value={data.preferences.threshold_seconds}
                  onChange={(e) =>
                    setData({
                      ...data,
                      preferences: {
                        ...data.preferences,
                        threshold_seconds: Number(e.target.value),
                      },
                    })
                  }
                />
              </label>
              <label>
                {t.app}
                <input
                  maxLength={300}
                  required={data.preferences.enabled}
                  value={data.preferences.allowed_app}
                  onChange={(e) =>
                    setData({
                      ...data,
                      preferences: { ...data.preferences, allowed_app: e.target.value },
                    })
                  }
                />
              </label>
            </div>
            <Button type="submit" disabled={disabled}>
              {t.settingsSave}
            </Button>
          </form>
        )}
        <p role="status">
          {status ? t.modes[status.state] : '手动模式可用'}
          {status?.source_app && ' · ' + status.source_app}
          {status?.error && ' · ' + status.error}
        </p>
        <p>{t.spotify}</p>
      </Card>
    </div>
  );
}

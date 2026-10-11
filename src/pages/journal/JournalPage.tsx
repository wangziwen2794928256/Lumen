import { translate } from '../../lib/locale';
import { useCallback, useEffect, useState } from 'react';
import { isTauri } from '@tauri-apps/api/core';
import { useSearchParams } from 'react-router-dom';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from '../../features/roadmap/Field';
import { journalInvoke } from '../../features/journal/api';
import { Editor } from '../../features/journal/Editor';
import { entryTypes, journalText as t } from '../../features/journal/text';
import {
  asCopy,
  emptyFilter,
  fresh,
  fromEntry,
  readPending,
  type Entry,
  type JournalPage as Page,
  type Save,
} from '../../features/journal/model';
export function JournalPage() {
  const [searchParams] = useSearchParams();
  const [filter, setFilter] = useState(() => {
      const date = searchParams.get('date');
      return date ? { ...emptyFilter, from: date, until: date } : emptyFilter;
    }),
    [data, setData] = useState<Page | null>(null),
    [error, setError] = useState(''),
    [editor, setEditor] = useState<Save | null>(null),
    [pending, setPending] = useState(() => readPending(false)),
    [undo, setUndo] = useState<Entry | null>(null);
  const refresh = useCallback(async () => {
    if (!isTauri()) return;
    try {
      setData(await journalInvoke<Page>('journal_page', { filter }));
      setError('');
    } catch {
      setError(t.error);
    }
  }, [filter]);
  useEffect(() => {
    const id = searchParams.get('entry');
    if (!id || !isTauri()) return;
    let alive = true;
    journalInvoke<Entry>('journal_get', { id })
      .then((e) => {
        if (alive) setEditor(fromEntry(e));
      })
      .catch(() => {
        if (alive) setError(t.error);
      });
    return () => {
      alive = false;
    };
  }, [searchParams]);
  useEffect(() => {
    let alive = true;
    async function fetch() {
      if (!isTauri()) return;
      try {
        const value = await journalInvoke<Page>('journal_page', { filter });
        if (alive) {
          setData(value);
          setError('');
        }
      } catch {
        if (alive) setError(t.error);
      }
    }
    const timer = setTimeout(() => void fetch(), 250),
      poll = setInterval(() => void fetch(), 5000);
    return () => {
      alive = false;
      clearTimeout(timer);
      clearInterval(poll);
    };
  }, [filter]);
  async function trash(e: Entry, restore = false) {
    try {
      const next = await journalInvoke<Entry>('journal_trash', {
        id: e.id,
        revision: e.revision,
        restore,
      });
      setUndo(restore ? null : next);
      await refresh();
    } catch {
      setError(t.error);
    }
  }
  async function reload() {
    if (!editor) return;
    try {
      setEditor(fromEntry(await journalInvoke<Entry>('journal_get', { id: editor.id })));
    } catch {
      setError(t.error);
    }
  }
  return (
    <>
      <div className="page-heading journal-page-heading">
        <p className="eyebrow">{translate('JOURNAL / 日记')}</p>
        <h1>{t.title}</h1>
        <p className="intro">{t.intro}</p>
      </div>
      {!isTauri() ? (
        <Card>{t.desktop}</Card>
      ) : editor ? (
        <Card>
          <Editor
            key={editor.client_id}
            initial={editor}
            choices={data ?? undefined}
            onBack={() => {
              setEditor(null);
              setPending(readPending(false));
              void refresh();
            }}
            onSaved={() => void refresh()}
            onRecover={setEditor}
            onReload={() => void reload()}
          />
        </Card>
      ) : (
        <>
          <div className="journal-toolbar">
            <Button onClick={() => setEditor(fresh())}>{t.new}</Button>
            <Button
              variant="secondary"
              onClick={() => void journalInvoke('capture_open').catch(() => setError(t.error))}
            >
              {t.capture}
            </Button>
            <span>{t.private}</span>
          </div>
          {pending && (
            <Card>
              <p>{t.recovery}</p>
              <Button
                onClick={() => {
                  setEditor(asCopy(pending));
                  setPending(null);
                }}
              >
                {t.recover}
              </Button>
            </Card>
          )}
          {data?.recoveries.map((r) => (
            <Card key={r.payload.client_id}>
              <p>
                {t.recovery} · {r.updated_at}
              </p>
              <Button onClick={() => setEditor(asCopy(r.payload))}>{t.recover}</Button>
            </Card>
          ))}
          <Card>
            <div className="journal-toolbar">
              {['published', 'drafts', 'trash'].map((mode, i) => (
                <Button
                  key={mode}
                  variant="secondary"
                  aria-pressed={filter.mode === mode}
                  onClick={() => setFilter({ ...filter, mode, page: 0 })}
                >
                  {[t.published, t.drafts, t.trash][i]}
                </Button>
              ))}
            </div>
            <FormGrid>
              <Field
                label={t.search}
                value={filter.query}
                onChange={(query) => setFilter({ ...filter, query, page: 0 })}
              />
              <Field
                label={t.type}
                value={filter.entry_type}
                options={[{ value: '', label: t.all }, ...entryTypes]}
                onChange={(entry_type) => setFilter({ ...filter, entry_type, page: 0 })}
              />
              <Field
                label={t.tag}
                value={filter.tag}
                onChange={(tag) => setFilter({ ...filter, tag, page: 0 })}
              />
              <Field
                label={t.from}
                value={filter.from}
                type="date"
                onChange={(from) => setFilter({ ...filter, from, page: 0 })}
              />
              <Field
                label={t.until}
                value={filter.until}
                type="date"
                onChange={(until) => setFilter({ ...filter, until, page: 0 })}
              />
            </FormGrid>
          </Card>
          {undo && (
            <div role="status" className="journal-toolbar">
              <span>{translate('记录已移入回收站。')}</span>
              <Button variant="secondary" onClick={() => void trash(undo, true)}>
                {t.undo}
              </Button>
            </div>
          )}
          {error && <p role="alert">{error}</p>}
          {data && data.entries.length === 0 && <Card>{t.empty}</Card>}
          <div className="journal-list">
            {data?.entries.map((e) => (
              <Card key={e.id}>
                <div className="journal-toolbar">
                  <span>
                    {e.date} · {entryTypes.find((x) => x.value === e.entry_type)?.label} ·{' '}
                    {e.is_draft ? t.drafts : t.published}
                  </span>
                  <span>{t.private}</span>
                </div>
                <h2>{e.title}</h2>
                <p className="journal-excerpt">{e.body.slice(0, 180)}</p>
                <p className="muted">{e.tags.join(' · ')}</p>
                {e.deleted_at ? (
                  <Button variant="secondary" onClick={() => void trash(e, true)}>
                    {t.restore}
                  </Button>
                ) : (
                  <div className="journal-toolbar">
                    <Button variant="secondary" onClick={() => setEditor(fromEntry(e))}>
                      {t.edit}
                    </Button>
                    <Button variant="ghost" onClick={() => void trash(e)}>
                      {t.remove}
                    </Button>
                  </div>
                )}
              </Card>
            ))}
          </div>
          {data && (
            <div className="journal-toolbar">
              <Button
                disabled={filter.page === 0}
                variant="secondary"
                onClick={() => setFilter({ ...filter, page: filter.page - 1 })}
              >
                {t.prev}
              </Button>
              <span>
                {filter.page + 1} / {Math.max(1, Math.ceil(data.total / 30))} · {data.total}{' '}
                {translate('条')}
              </span>
              <Button
                disabled={(filter.page + 1) * 30 >= data.total}
                variant="secondary"
                onClick={() => setFilter({ ...filter, page: filter.page + 1 })}
              >
                {t.next}
              </Button>
            </div>
          )}
        </>
      )}
    </>
  );
}

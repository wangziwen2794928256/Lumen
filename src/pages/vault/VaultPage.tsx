import { useCallback, useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { Button } from '../../components/ui/button';
import { Field } from '../../features/roadmap/Field';
import { vaultText as t } from '../../features/vault/text';
import {
  categories,
  emptyArtifact,
  portfolio,
  tags,
  type Artifact,
  type Snapshot,
  type Attachment,
} from '../../features/vault/model';
import { isTauri, vaultAction, vaultInvoke } from '../../features/vault/api';
const blank: Snapshot = { artifacts: [], journeys: [], competitions: [], limit: 1000 };
function group(kind: string) {
  return kind === 'award'
    ? 'Awards'
    : kind === 'competition'
      ? 'Competition'
      : kind === 'paper'
        ? 'Publications'
        : kind === 'research'
          ? 'Research'
          : kind === 'certificate' || kind === 'course_project'
            ? 'Education'
            : ['project', 'open_source'].includes(kind)
              ? 'Projects'
              : 'Links';
}
export function VaultPage() {
  const [searchParams] = useSearchParams();
  const [data, setData] = useState<Snapshot>(blank),
    [draft, setDraft] = useState<Artifact | null>(null),
    [mode, setMode] = useState('active'),
    [error, setError] = useState(''),
    [status, setStatus] = useState(''),
    [busy, setBusy] = useState(false),
    [query, setQuery] = useState(''),
    [kind, setKind] = useState('');
  const load = useCallback(async () => {
    if (isTauri()) setData(await vaultInvoke<Snapshot>('vault_snapshot'));
  }, []);
  useEffect(() => {
    let alive = true;
    if (isTauri())
      void vaultInvoke<Snapshot>('vault_snapshot')
        .then((x) => {
          if (alive) {
            setData(x);
            const id = searchParams.get('artifact');
            if (id)
              setDraft(
                x.artifacts.find((a) => a.id === id && !a.deleted_at && !a.archived_at) ?? null,
              );
          }
        })
        .catch(() => {
          if (alive) setError(t.error);
        });
    return () => {
      alive = false;
    };
  }, [searchParams]);
  async function run(fn: () => Promise<void>) {
    setBusy(true);
    setError('');
    setStatus('');
    try {
      await fn();
      await load();
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  async function save() {
    if (!draft) return;
    await run(async () => {
      await vaultAction({ type: 'save', artifact: draft });
      setDraft(null);
      setStatus(t.saved);
    });
  }
  function change<K extends keyof Artifact>(key: K, value: Artifact[K]) {
    setDraft((a) => (a ? { ...a, [key]: value } : a));
  }
  const shown = data.artifacts
    .filter((a) =>
      mode === 'deleted'
        ? !!a.deleted_at
        : mode === 'archived'
          ? !!a.archived_at && !a.deleted_at
          : !a.deleted_at && !a.archived_at,
    )
    .filter(
      (a) =>
        (!kind || a.kind === kind) &&
        `${a.title} ${a.description} ${a.skills.join(' ')} ${a.technologies.join(' ')}`
          .toLowerCase()
          .includes(query.toLowerCase()),
    );
  const publicRecords = portfolio(data.artifacts);
  return (
    <div className="vault-page">
      <header>
        <h1>{t.heading}</h1>
        <p className="muted">{t.hint}</p>
      </header>
      {!isTauri() && <p role="status">{t.browser}</p>}
      <div className="vault-actions">
        {[
          ['active', t.active],
          ['archived', t.archived],
          ['deleted', t.deleted],
          ['preview', t.preview],
        ].map(([value, label]) => (
          <Button
            key={value}
            variant={mode === value ? 'default' : 'secondary'}
            onClick={() => setMode(value)}
          >
            {label}
          </Button>
        ))}
        <Button
          disabled={!isTauri() || busy}
          onClick={() => {
            setDraft(emptyArtifact());
            setMode('active');
          }}
        >
          {t.create}
        </Button>
        <Button disabled={busy} variant="ghost" onClick={() => void run(load)}>
          {t.reload}
        </Button>
      </div>
      {error && <p role="alert">{error}</p>}
      {status && <p role="status">{status}</p>}
      {draft && (
        <section className="card vault-editor">
          <h2>{draft.revision ? t.edit : t.create}</h2>
          <form
            key={draft.id}
            onSubmit={(e) => {
              e.preventDefault();
              void save();
            }}
          >
            <div className="vault-form-grid">
              <Field label={t.title} value={draft.title} onChange={(v) => change('title', v)} />
              <label>
                {t.kind}
                <select value={draft.kind} onChange={(e) => change('kind', e.target.value)}>
                  {Object.entries(categories).map(([key, label]) => (
                    <option value={key} key={key}>
                      {label}
                    </option>
                  ))}
                </select>
              </label>
              <Field
                label={t.date}
                type="date"
                value={draft.artifact_date ?? ''}
                onChange={(v) => change('artifact_date', v || null)}
              />
              <Field label={t.role} value={draft.role} onChange={(v) => change('role', v)} />
              <Field
                label={t.organization}
                value={draft.organization}
                onChange={(v) => change('organization', v)}
              />
              <label>
                {t.visibility}
                <select
                  value={draft.visibility}
                  onChange={(e) => change('visibility', e.target.value)}
                >
                  <option value="private">{t.private}</option>
                  <option value="public">{t.public}</option>
                </select>
              </label>
              <label>
                {t.journey}
                <select
                  value={draft.journey_id ?? ''}
                  onChange={(e) => change('journey_id', e.target.value || null)}
                >
                  <option value="">{t.none}</option>
                  {data.journeys.map((a) => (
                    <option key={a.id} value={a.id}>
                      {a.title}
                    </option>
                  ))}
                </select>
              </label>
              <label>
                {t.competition}
                <select
                  value={draft.competition_id ?? ''}
                  onChange={(e) => change('competition_id', e.target.value || null)}
                >
                  <option value="">{t.none}</option>
                  {data.competitions.map((a) => (
                    <option key={a.id} value={a.id}>
                      {a.title}
                    </option>
                  ))}
                </select>
              </label>
              <TagField
                label={t.skills}
                values={draft.skills}
                onChange={(v) => change('skills', v)}
              />
              <TagField
                label={t.technologies}
                values={draft.technologies}
                onChange={(v) => change('technologies', v)}
              />
              {(['repository_url', 'project_url', 'paper_url'] as const).map((key) => (
                <Field
                  key={key}
                  label={t[key]}
                  type="url"
                  value={draft[key]}
                  onChange={(v) => change(key, v)}
                />
              ))}
            </div>
            <label>
              {t.description}
              <textarea
                rows={5}
                maxLength={20000}
                value={draft.description}
                onChange={(e) => change('description', e.target.value)}
              />
            </label>
            <div className="vault-actions">
              <Button type="submit" disabled={busy || !draft.title.trim()}>
                {t.save}
              </Button>
              <Button
                type="button"
                variant="secondary"
                disabled={busy}
                onClick={() => setDraft(null)}
              >
                {t.cancel}
              </Button>
            </div>
          </form>
        </section>
      )}
      {mode === 'preview' ? (
        <section aria-label={t.preview} className="vault-preview">
          <h2>{t.preview}</h2>
          {!publicRecords.length && <p>{t.previewEmpty}</p>}
          {Object.entries(t.groups).map(([key, label]) => {
            const items = publicRecords.filter((a) => group(a.kind) === key);
            return items.length ? (
              <section key={key}>
                <h3>{label}</h3>
                {items.map((a) => (
                  <article className="card" key={a.id}>
                    <h4>{a.title}</h4>
                    <p>
                      {a.artifact_date} {a.role} {a.organization}
                    </p>
                    <p className="vault-body">{a.description}</p>
                    <p>{[...a.skills, ...a.technologies].join(' · ')}</p>
                    <div className="vault-actions">
                      {[a.repository_url, a.project_url, a.paper_url].filter(Boolean).map((url) => (
                        <a href={url} key={url} target="_blank" rel="noreferrer">
                          {url}
                        </a>
                      ))}
                    </div>
                  </article>
                ))}
              </section>
            ) : null;
          })}
          {!!publicRecords.length && (
            <section>
              <h3>{t.technical}</h3>
              <p>
                {[...new Set(publicRecords.flatMap((a) => [...a.skills, ...a.technologies]))].join(
                  ' · ',
                )}
              </p>
            </section>
          )}
        </section>
      ) : (
        <>
          <div className="vault-filters">
            <Field label={t.search} value={query} onChange={setQuery} />
            <label>
              {t.kind}
              <select value={kind} onChange={(e) => setKind(e.target.value)}>
                <option value="">{t.all}</option>
                {Object.entries(categories).map(([key, label]) => (
                  <option value={key} key={key}>
                    {label}
                  </option>
                ))}
              </select>
            </label>
          </div>
          {!shown.length && <p>{t.empty}</p>}
          <div className="vault-list">
            {shown.map((a) => (
              <article className="card" key={a.id}>
                <h2>{a.title}</h2>
                <p className="muted">
                  {categories[a.kind as keyof typeof categories] ?? a.kind} · {a.artifact_date} ·{' '}
                  {a.visibility === 'public' ? t.public : t.private}
                </p>
                <p className="vault-body">{a.description}</p>
                <p>
                  {a.role} {a.organization}
                </p>
                <p>{[...a.skills, ...a.technologies].join(' · ')}</p>
                <div className="vault-actions">
                  {!a.deleted_at && !a.archived_at && (
                    <>
                      <Button
                        disabled={busy}
                        variant="secondary"
                        onClick={() =>
                          setDraft({ ...a, kind: a.kind in categories ? a.kind : 'other' })
                        }
                      >
                        {t.edit}
                      </Button>
                      <Button
                        disabled={busy}
                        variant="ghost"
                        onClick={() =>
                          void run(() =>
                            vaultAction({
                              type: 'state',
                              id: a.id,
                              revision: a.revision,
                              state: 'archive',
                            }),
                          )
                        }
                      >
                        {t.archive}
                      </Button>
                    </>
                  )}
                  {a.deleted_at || a.archived_at ? (
                    <Button
                      disabled={busy}
                      onClick={() =>
                        void run(() =>
                          vaultAction({
                            type: 'state',
                            id: a.id,
                            revision: a.revision,
                            state: 'restore',
                          }),
                        )
                      }
                    >
                      {t.restore}
                    </Button>
                  ) : null}
                  {!a.deleted_at && (
                    <Button
                      disabled={busy}
                      variant="ghost"
                      onClick={() =>
                        void run(() =>
                          vaultAction({
                            type: 'state',
                            id: a.id,
                            revision: a.revision,
                            state: 'trash',
                          }),
                        )
                      }
                    >
                      {t.trash}
                    </Button>
                  )}
                </div>
                {!a.deleted_at && (
                  <Files key={`${a.id}-${a.revision}`} artifact={a} reload={load} />
                )}
              </article>
            ))}
          </div>
        </>
      )}
      <p className="muted">{t.limit}</p>
    </div>
  );
}
function TagField({
  label,
  values,
  onChange,
}: {
  label: string;
  values: string[];
  onChange: (v: string[]) => void;
}) {
  const [value, setValue] = useState(values.join(', '));
  return (
    <Field
      label={label}
      value={value}
      onChange={(v) => {
        setValue(v);
        onChange(tags(v));
      }}
    />
  );
}
function Files({ artifact: a, reload }: { artifact: Artifact; reload: () => Promise<void> }) {
  const [files, setFiles] = useState<Attachment[]>([]),
    [checks, setChecks] = useState<Record<string, string>>({}),
    [error, setError] = useState(''),
    [busy, setBusy] = useState(false);
  useEffect(() => {
    let alive = true;
    void vaultInvoke<Attachment[]>('vault_files', { id: a.id })
      .then((x) => {
        if (alive) setFiles(x);
      })
      .catch(() => {
        if (alive) setError(t.error);
      });
    return () => {
      alive = false;
    };
  }, [a.id]);
  async function act(fn: () => Promise<void>) {
    setBusy(true);
    setError('');
    try {
      await fn();
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  return (
    <section className="vault-files">
      <h3>{t.files}</h3>
      <p className="muted">{t.fileHint}</p>
      <Button
        disabled={busy || !!a.archived_at}
        variant="secondary"
        onClick={() =>
          void act(async () => {
            await vaultInvoke('vault_pick_attachment', { id: a.id, revision: a.revision });
            await reload();
          })
        }
      >
        {t.import}
      </Button>
      {error && <p role="alert">{error}</p>}
      <ul>
        {files.map((f) => (
          <li key={f.id}>
            <strong>{f.name}</strong>
            <p className="vault-hash">
              {f.mime_type} · {f.size_bytes} bytes · SHA-256 {f.sha256}
            </p>
            <div className="vault-actions">
              {[
                [false, t.verify],
                [true, t.open],
              ].map(([open, label]) => (
                <Button
                  key={String(open)}
                  disabled={busy}
                  variant="ghost"
                  onClick={() =>
                    void act(async () => {
                      const status = await vaultInvoke<string>('vault_verify', { id: f.id, open });
                      setChecks((x) => ({ ...x, [f.id]: status }));
                    })
                  }
                >
                  {String(label)}
                </Button>
              ))}
            </div>
            {checks[f.id] && (
              <p role="status">
                {checks[f.id] === 'verified'
                  ? t.verified
                  : checks[f.id] === 'missing'
                    ? t.missing
                    : t.changed}
              </p>
            )}
          </li>
        ))}
      </ul>
    </section>
  );
}

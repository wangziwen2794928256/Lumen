import { useEffect, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { Button } from '../../components/ui/button';
import type { ReactNode } from 'react';
function Field({ label, children }: { label: string; children: ReactNode }) {
  return (
    <label className="profile-field">
      <span>{label}</span>
      {children}
    </label>
  );
}
import {
  emptyProfile,
  type Profile,
  type Projection,
  type Receipt,
} from '../../features/profile/model';
import { profileText as t } from '../../features/profile/text';
export function ProfilePage() {
  const [draft, setDraft] = useState<Profile>(emptyProfile),
    [saved, setSaved] = useState<Profile | null>(null),
    [projection, setProjection] = useState<Projection | null>(null),
    [receipt, setReceipt] = useState<Receipt | null>(null),
    [status, setStatus] = useState(''),
    [error, setError] = useState(''),
    [busy, setBusy] = useState(false);
  const dirty = JSON.stringify(draft) !== JSON.stringify(saved);
  useEffect(() => {
    let alive = true;
    if (isTauri())
      void invoke<Profile>('profile_get')
        .then((p) => {
          if (alive) {
            setDraft(p);
            setSaved(p);
          }
        })
        .catch(() => {
          if (alive) setError(t.error);
        });
    return () => {
      alive = false;
    };
  }, []);
  async function run(task: () => Promise<void>) {
    setBusy(true);
    setError('');
    setStatus('');
    try {
      await task();
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  function update<K extends keyof Profile>(key: K, value: Profile[K]) {
    setDraft({ ...draft, [key]: value });
    setProjection(null);
  }
  const readonly = !isTauri() || !saved || busy;
  return (
    <div className="profile-page">
      <header className="page-heading">
        <div>
          <p className="eyebrow">PROFILE · PUBLIC EXPORT</p>
          <h1>{t.title}</h1>
          <p className="muted">{t.subtitle}</p>
        </div>
      </header>
      {!isTauri() && <p role="status">{t.browser}</p>}
      {error && <p role="alert">{error}</p>}
      {status && <p role="status">{status}</p>}
      <section className="profile-card">
        <h2>{t.identity}</h2>
        <p className="muted">{t.rule}</p>
        <div className="profile-form">
          <Field label={t.name}>
            <input
              value={draft.display_name}
              maxLength={100}
              disabled={readonly}
              onChange={(e) => update('display_name', e.target.value)}
            />
          </Field>
          <Field label={t.headline}>
            <input
              value={draft.headline}
              maxLength={300}
              disabled={readonly}
              onChange={(e) => update('headline', e.target.value)}
            />
          </Field>
          <Field label={t.about}>
            <textarea
              value={draft.about}
              maxLength={10000}
              rows={5}
              disabled={readonly}
              onChange={(e) => update('about', e.target.value)}
            />
          </Field>
        </div>
        <div className="profile-links">
          {draft.links.map((link, i) => (
            <div className="profile-link" key={i}>
              <Field label={`${t.label} ${i + 1}`}>
                <input
                  value={link.label}
                  maxLength={100}
                  disabled={readonly}
                  onChange={(e) =>
                    update(
                      'links',
                      draft.links.map((l, n) => (n === i ? { ...l, label: e.target.value } : l)),
                    )
                  }
                />
              </Field>
              <Field label={`${t.url} ${i + 1}`}>
                <input
                  type="url"
                  value={link.url}
                  maxLength={2048}
                  disabled={readonly}
                  onChange={(e) =>
                    update(
                      'links',
                      draft.links.map((l, n) => (n === i ? { ...l, url: e.target.value } : l)),
                    )
                  }
                />
              </Field>
              <Button
                variant="ghost"
                disabled={readonly}
                onClick={() =>
                  update(
                    'links',
                    draft.links.filter((_, n) => n !== i),
                  )
                }
              >
                {t.remove} {i + 1}
              </Button>
            </div>
          ))}
        </div>
        <div className="profile-actions">
          <Button
            variant="secondary"
            disabled={readonly || draft.links.length >= 20}
            onClick={() => update('links', [...draft.links, { label: '', url: '' }])}
          >
            {t.addLink}
          </Button>
        </div>
        <label className="profile-visibility">
          <input
            type="checkbox"
            checked={draft.public}
            disabled={readonly}
            onChange={(e) => update('public', e.target.checked)}
          />
          {t.visibility}
        </label>
        <div className="profile-actions">
          <Button
            disabled={readonly || !dirty}
            onClick={() =>
              void run(async () => {
                await invoke('profile_save', { profile: draft });
                const p = await invoke<Profile>('profile_get');
                setDraft(p);
                setSaved(p);
                setStatus(t.saved);
              })
            }
          >
            {t.save}
          </Button>
        </div>
      </section>
      <section className="profile-card">
        <h2>{t.reviewed}</h2>
        <p>{t.excluded}</p>
        <p className="muted">{t.snapshot}</p>
        {dirty && <p>{t.unsaved}</p>}
        <div className="profile-actions">
          <Button
            variant="secondary"
            disabled={readonly || dirty}
            onClick={() =>
              void run(async () => setProjection(await invoke<Projection>('public_projection')))
            }
          >
            {t.review}
          </Button>
          <Button
            disabled={readonly || dirty || !projection}
            onClick={() =>
              void run(async () => {
                setReceipt(
                  await invoke<Receipt>('public_export', { expectedHash: projection?.review_hash }),
                );
                setStatus(t.exported);
              })
            }
          >
            {t.export}
          </Button>
          <Button
            variant="secondary"
            disabled={readonly || dirty || !projection}
            onClick={() =>
              void run(async () =>
                setStatus(
                  `${t.opened}：${await invoke<string>('public_preview', { expectedHash: projection?.review_hash })}`,
                ),
              )
            }
          >
            {t.preview}
          </Button>
        </div>
        {projection && (
          <div className="profile-projection">
            <p>
              {projection.profile?.display_name || t.unpublished} · {projection.artifacts.length}{' '}
              {t.count}
            </p>
            {projection.artifacts.length ? (
              <ul>
                {projection.artifacts.map((a) => (
                  <li key={a.id}>
                    {a.title} · {a.kind}
                  </li>
                ))}
              </ul>
            ) : (
              <p>{t.empty}</p>
            )}
            <details>
              <summary>public-profile.json</summary>
              <pre>{JSON.stringify(projection, null, 2)}</pre>
            </details>
          </div>
        )}
        {receipt && (
          <div className="profile-receipt">
            <p>{receipt.directory}</p>
            <p>
              {t.hash}：{receipt.sha256}
            </p>
          </div>
        )}
      </section>
      <section className="profile-card">
        <h2>Open in Lumen</h2>
        <p className="muted">{t.deepLinkNote}</p>
        <Button
          variant="secondary"
          disabled={readonly}
          onClick={() =>
            void run(async () => {
              await invoke('deep_link_register');
              setStatus(t.registered);
            })
          }
        >
          {t.register}
        </Button>
      </section>
    </div>
  );
}

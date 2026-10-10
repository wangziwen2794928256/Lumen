import { useEffect, useState } from 'react';
import { Button } from '../../components/ui/button';
import { Field } from '../roadmap/Field';
import { journalInvoke } from './api';
import { journalText as t } from './text';
import type { Attachment } from './model';
export function Attachments({ id }: { id: string }) {
  const [items, setItems] = useState<Attachment[]>([]),
    [url, setUrl] = useState(''),
    [error, setError] = useState(''),
    [busy, setBusy] = useState(false),
    [checks, setChecks] = useState<Record<string, string>>({});
  useEffect(() => {
    let alive = true;
    void journalInvoke<Attachment[]>('journal_attachments', { id })
      .then((x) => {
        if (alive) setItems(x);
      })
      .catch(() => {
        if (alive) setError(t.error);
      });
    return () => {
      alive = false;
    };
  }, [id]);
  async function add(file: boolean) {
    setBusy(true);
    setError('');
    try {
      setItems(
        await journalInvoke<Attachment[]>(
          file ? 'journal_pick_attachment' : 'journal_link',
          file ? { id } : { id, url },
        ),
      );
      if (!file) setUrl('');
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  async function verify(key: string) {
    try {
      const result = await journalInvoke<string>('journal_verify', { id: key });
      setChecks((x) => ({ ...x, [key]: result }));
    } catch {
      setError(t.error);
    }
  }
  return (
    <section className="journal-attachments">
      <h3>{t.attachments}</h3>
      <p className="muted">{t.attachmentHint}</p>
      <Button disabled={busy} variant="secondary" onClick={() => void add(true)}>
        {t.import}
      </Button>
      <div className="journal-link-input">
        <Field label={t.link} value={url} onChange={setUrl} type="url" />
        <Button disabled={busy || !url} onClick={() => void add(false)}>
          {t.addLink}
        </Button>
      </div>
      {error && <p role="alert">{error}</p>}
      <ul>
        {items.map((a) => (
          <li key={a.id}>
            {a.url ? (
              <a href={a.url} target="_blank" rel="noreferrer">
                {a.name}
              </a>
            ) : (
              <>
                <strong>{a.name}</strong>
                <p className="muted">
                  {a.size_bytes} bytes · SHA256 {a.sha256}
                </p>
                <Button size="sm" variant="ghost" onClick={() => void verify(a.id)}>
                  {t.verify}
                </Button>
                <Button
                  size="sm"
                  variant="secondary"
                  onClick={() =>
                    void journalInvoke('journal_open_attachment', { id: a.id }).catch(() =>
                      setError(t.error),
                    )
                  }
                >
                  {t.openAttachment}
                </Button>
                {checks[a.id] && (
                  <p role="status">
                    {checks[a.id] === 'verified'
                      ? t.verified
                      : checks[a.id] === 'missing'
                        ? t.missing
                        : t.changed}
                  </p>
                )}
              </>
            )}
          </li>
        ))}
      </ul>
    </section>
  );
}

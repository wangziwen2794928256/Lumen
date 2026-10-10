import { useCallback, useEffect, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { privacyCenterText as t } from './centerText';
type Status = {
  data_directory: string;
  activity_tracking: boolean;
  windows_media_enabled: boolean;
  integrations: number;
  credential_references: number;
  network_upload: boolean;
};
type Import = {
  token: string;
  review: {
    add: number;
    update: number;
    conflicts: number;
    invalid: number;
    unchanged: number;
    fingerprint: string;
  };
};
type Restore = {
  candidate: string;
  schema: number;
  integrity: string;
  tables: number;
  sample_rows: number;
  replaces_live_database: boolean;
};
export function PrivacyCenter() {
  const [status, setStatus] = useState<Status | null>(null),
    [busy, setBusy] = useState(false),
    [error, setError] = useState(''),
    [message, setMessage] = useState('');
  const [review, setReview] = useState<Import | null>(null),
    [restore, setRestore] = useState<Restore | null>(null),
    [consent, setConsent] = useState(false),
    [deleteConsent, setDeleteConsent] = useState(false);
  const refresh = useCallback(async () => setStatus(await invoke<Status>('privacy_status')), []);
  useEffect(() => {
    if (!isTauri()) return;
    let alive = true;
    void invoke<Status>('privacy_status')
      .then((s) => {
        if (alive) setStatus(s);
      })
      .catch(() => {
        if (alive) setError(t.error);
      });
    return () => {
      alive = false;
    };
  }, []);
  async function run(action: () => Promise<void>) {
    setBusy(true);
    setError('');
    setMessage('');
    try {
      await action();
      await refresh();
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  const disabled = busy || !status;
  return (
    <Card className="privacy-center backup-settings">
      <h2>{t.title}</h2>
      <p>{t.hint}</p>
      <p>{t.collected}</p>
      {!isTauri() && <p>{t.browser}</p>}
      <p className="privacy-path">
        {t.storage}：{status?.data_directory ?? '—'}
      </p>
      <Button
        variant="secondary"
        disabled={disabled}
        onClick={() =>
          void run(async () => {
            await invoke('privacy_open_folder');
          })
        }
      >
        {t.open}
      </Button>
      <p>
        {t.media}：{status?.windows_media_enabled ? t.on : t.off} · {t.integrations}：
        {status?.integrations ?? '—'} · {t.credentials}：{status?.credential_references ?? '—'}
      </p>
      <Button
        disabled={disabled}
        onClick={() =>
          void run(async () => {
            await invoke('privacy_disable', { delete: false });
            setMessage(t.disconnected);
          })
        }
      >
        {t.disable}
      </Button>
      <label>
        <input
          type="checkbox"
          disabled={disabled}
          checked={deleteConsent}
          onChange={(e) => setDeleteConsent(e.target.checked)}
        />{' '}
        {t.deleteConsent}
      </label>
      <Button
        variant="destructive"
        disabled={disabled || !deleteConsent}
        onClick={() =>
          void run(async () => {
            await invoke('privacy_disable', { delete: true });
            setDeleteConsent(false);
            setMessage(t.disconnected);
          })
        }
      >
        {t.delete}
      </Button>
      <h3>{t.export}</h3>
      <p>{t.exportHint}</p>
      <div className="backup-actions">
        {(['json', 'csv', 'markdown'] as const).map((format) => (
          <Button
            key={format}
            variant="secondary"
            disabled={disabled}
            onClick={() =>
              void run(async () => {
                setMessage(t.saved + (await invoke<string>('private_export', { format })));
              })
            }
          >
            {t[format]}
          </Button>
        ))}
      </div>
      <h3>{t.import}</h3>
      <p>{t.importHint}</p>
      <Button
        variant="secondary"
        disabled={disabled}
        onClick={() =>
          void run(async () => {
            setReview(null);
            setConsent(false);
            setReview(await invoke<Import | null>('import_review'));
          })
        }
      >
        {t.choose}
      </Button>
      {review && (
        <>
          <p>
            {(['add', 'update', 'conflicts', 'invalid', 'unchanged'] as const)
              .map((k) => `${t[k]}：${review.review[k]}`)
              .join(' · ')}
          </p>
          <label>
            <input
              type="checkbox"
              checked={consent}
              disabled={busy}
              onChange={(e) => setConsent(e.target.checked)}
            />{' '}
            {t.importConsent}
          </label>
          <Button
            disabled={
              disabled || !consent || review.review.conflicts > 0 || review.review.invalid > 0
            }
            onClick={() =>
              void run(async () => {
                await invoke('import_confirm', {
                  token: review.token,
                  fingerprint: review.review.fingerprint,
                });
                setReview(null);
                setConsent(false);
                setMessage(t.imported);
              })
            }
          >
            {t.confirm}
          </Button>
        </>
      )}
      <h3>{t.restore}</h3>
      <p>{t.restoreHint}</p>
      <Button
        variant="secondary"
        disabled={disabled}
        onClick={() =>
          void run(async () => {
            setRestore(null);
            setRestore(await invoke<Restore | null>('restore_rehearse'));
          })
        }
      >
        {t.rehearse}
      </Button>
      {restore && (
        <p className="privacy-path">
          {t.candidate}：{restore.candidate} · {t.schema} {restore.schema} · {t.integrity}{' '}
          {restore.integrity} · {t.samples} {restore.sample_rows}/{restore.tables}
        </p>
      )}
      {error && <p role="alert">{error}</p>}
      {message && (
        <p role="status" className="privacy-path">
          {message}
        </p>
      )}
    </Card>
  );
}

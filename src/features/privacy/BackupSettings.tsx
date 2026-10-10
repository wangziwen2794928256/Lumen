import { useEffect, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Field, FormGrid } from '../roadmap/Field';
import { privacyText as t } from './text';
type Policy = { enabled: boolean; daily: number; weekly: number; monthly: number };
type Status = {
  policy: Policy;
  scheduler: { last_success: string | null; error_code: string | null };
};
export function BackupSettings() {
  const [policy, setPolicy] = useState<Policy>({ enabled: true, daily: 7, weekly: 4, monthly: 6 });
  const [status, setStatus] = useState<Status | null>(null);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');
  useEffect(() => {
    if (!isTauri()) return;
    let alive = true;
    void invoke<Status>('backup_status')
      .then((s) => {
        if (alive) {
          setStatus(s);
          setPolicy(s.policy);
        }
      })
      .catch(() => {
        if (alive) setError(t.error);
      });
    return () => {
      alive = false;
    };
  }, []);
  const valid =
    Number.isInteger(policy.daily) &&
    policy.daily >= 1 &&
    policy.daily <= 90 &&
    Number.isInteger(policy.weekly) &&
    policy.weekly >= 1 &&
    policy.weekly <= 52 &&
    Number.isInteger(policy.monthly) &&
    policy.monthly >= 1 &&
    policy.monthly <= 60;
  async function run(create: boolean) {
    setBusy(true);
    setError('');
    setMessage('');
    try {
      if (create) {
        const id = await invoke<string>('backup_create');
        setMessage(t.created + id);
      } else {
        await invoke('backup_policy_save', { policy });
        setMessage(t.saved);
      }
      setStatus(await invoke<Status>('backup_status'));
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Card className="backup-settings">
      <h2>{t.backup}</h2>
      <p>{t.hint}</p>
      <p>{t.retention}</p>
      {!isTauri() && <p>{t.browser}</p>}
      <label>
        <input
          type="checkbox"
          checked={policy.enabled}
          disabled={busy || !status}
          onChange={(e) => setPolicy({ ...policy, enabled: e.target.checked })}
        />{' '}
        {t.enabled}
      </label>
      <FormGrid>
        {(['daily', 'weekly', 'monthly'] as const).map((key) => (
          <Field
            key={key}
            label={t[key]}
            type="number"
            min={1}
            max={{ daily: 90, weekly: 52, monthly: 60 }[key]}
            value={policy[key]}
            onChange={(v) => setPolicy({ ...policy, [key]: Number(v) })}
          />
        ))}
      </FormGrid>
      <div className="backup-actions">
        <Button disabled={busy || !status || !valid} onClick={() => void run(false)}>
          {t.save}
        </Button>
        <Button variant="secondary" disabled={busy || !status} onClick={() => void run(true)}>
          {t.create}
        </Button>
      </div>
      <p>
        {status?.scheduler.last_success ? t.success + status.scheduler.last_success : t.pending}
      </p>
      {(error || status?.scheduler.error_code) && <p role="alert">{error || t.error}</p>}
      {message && <p role="status">{message}</p>}
      <p className="muted">{t.private}</p>
    </Card>
  );
}

import { useEffect, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import type { DesktopStatus } from './model';
import { desktopText as t } from './text';
export function DesktopSettings() {
  const [status, setStatus] = useState<DesktopStatus | null>(null),
    [busy, setBusy] = useState(false),
    [error, setError] = useState(''),
    [message, setMessage] = useState('');
  useEffect(() => {
    if (!isTauri()) return;
    let alive = true;
    void invoke<DesktopStatus>('desktop_status')
      .then((s) => {
        if (alive) setStatus(s);
      })
      .catch(() => {
        if (alive) setError(t.unavailable);
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
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  return (
    <Card className="desktop-settings">
      <h3>{t.title}</h3>
      <p>{t.hint}</p>
      {!isTauri() && <p className="muted">{t.browser}</p>}
      {error && <p role="alert">{error}</p>}
      {status && (
        <>
          <p>
            {t.tray}：{status.runtime.tray_ready ? t.ready : t.failed}
          </p>
          {status.runtime.error_code && (
            <p role="status">
              {t.unavailable}（{status.runtime.error_code}）
            </p>
          )}
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={status.autostart === true}
              disabled={busy || status.autostart === null}
              onChange={(e) => {
                const enabled = e.target.checked;
                void run(async () => {
                  const value = await invoke<boolean>('desktop_autostart', { enabled });
                  setStatus((s) => (s ? { ...s, autostart: value } : s));
                  setMessage(t.saved);
                });
              }}
            />
            {t.autostart}
          </label>
          {(
            [
              'close_to_tray',
              'notifications_enabled',
              'focus_finished',
              'upcoming_event',
              'weekly_review',
              'backup_failure',
            ] as const
          ).map((key, i) => (
            <label className="checkbox-label" key={key}>
              <input
                type="checkbox"
                checked={status.preferences[key]}
                disabled={busy || (key === 'close_to_tray' && !status.runtime.tray_ready)}
                onChange={(e) => {
                  const value = e.target.checked;
                  setStatus((s) =>
                    s ? { ...s, preferences: { ...s.preferences, [key]: value } } : s,
                  );
                }}
              />
              {[t.close, t.notifications, t.focus, t.event, t.review, t.backup][i]}
            </label>
          ))}
          <p>
            {t.permission}：{status.notification_permission}。{t.permissionHint}
          </p>
          <div className="desktop-actions">
            <Button
              disabled={busy}
              onClick={() =>
                void run(async () => {
                  const preferences = await invoke<DesktopStatus['preferences']>(
                    'desktop_preferences_save',
                    { preferences: status.preferences },
                  );
                  setStatus((s) => (s ? { ...s, preferences } : s));
                  setMessage(t.saved);
                })
              }
            >
              {t.save}
            </Button>
            <Button
              variant="secondary"
              disabled={busy}
              onClick={() =>
                void run(async () => {
                  setStatus(await invoke<DesktopStatus>('desktop_status'));
                })
              }
            >
              {t.reload}
            </Button>
            <Button
              variant="secondary"
              disabled={
                busy ||
                !status.preferences.notifications_enabled ||
                !status.preferences.focus_finished
              }
              onClick={() =>
                void run(async () => {
                  await invoke('desktop_test_notification');
                  setMessage(t.sent);
                })
              }
            >
              {t.test}
            </Button>
          </div>
        </>
      )}
      {message && <p role="status">{message}</p>}
    </Card>
  );
}

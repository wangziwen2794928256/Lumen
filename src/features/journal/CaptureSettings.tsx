import { useEffect, useState } from 'react';
import { isTauri } from '@tauri-apps/api/core';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Field } from '../roadmap/Field';
import { journalInvoke } from './api';
import { journalText as t } from './text';
type Status = { shortcut: string; registered: boolean; error: string | null };
export function CaptureSettings() {
  const [value, setValue] = useState('Control+Shift+Space'),
    [status, setStatus] = useState<Status | null>(null),
    [error, setError] = useState('');
  useEffect(() => {
    if (!isTauri()) return;
    let alive = true;
    void journalInvoke<Status>('capture_status')
      .then((x) => {
        if (alive) {
          setValue(x.shortcut);
          setStatus(x);
        }
      })
      .catch(() => {
        if (alive) setError(t.shortcutFail);
      });
    return () => {
      alive = false;
    };
  }, []);
  async function save() {
    try {
      setStatus(await journalInvoke<Status>('capture_shortcut', { shortcut: value }));
      setError('');
    } catch {
      setError(t.shortcutFail);
    }
  }
  return (
    <Card className="journal-settings">
      <h2>{t.shortcut}</h2>
      <p>{t.shortcutHint}</p>
      <Field label={t.shortcut} value={value} onChange={setValue} />
      <Button disabled={!isTauri()} onClick={() => void save()}>
        {t.shortcutSave}
      </Button>
      {(error || status?.error) && <p role="alert">{error || status?.error}</p>}
      {status?.registered && <p role="status">{t.shortcutOK}</p>}
    </Card>
  );
}

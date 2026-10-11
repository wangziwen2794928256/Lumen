import { locale } from '../../lib/locale';
import { useState } from 'react';
import { useModules } from './ModuleProvider';
import { type RegistryState } from './model';
import { moduleText as t } from './text';
import { ordered, reorder } from './registry';
import { Button } from '../../components/ui/button';
import { Card } from '../../components/ui/card';
export function ModuleSettings() {
  const { state, ready, save, reload } = useModules();
  const [draft, setDraft] = useState<RegistryState | null>(null),
    [message, setMessage] = useState(''),
    [busy, setBusy] = useState(false),
    [drag, setDrag] = useState('');
  const value = draft ?? state;
  function change(id: string, key: 'enabled' | 'hidden' | 'pinned', checked: boolean) {
    setDraft({
      ...value,
      items: value.items.map((i) => (i.id === id ? { ...i, [key]: checked } : i)),
    });
    setMessage('');
  }
  async function submit() {
    setBusy(true);
    try {
      await save(value);
      setDraft(null);
      setMessage(t.saved);
    } catch {
      setMessage(t.error);
    } finally {
      setBusy(false);
    }
  }
  const rows = ordered(value);
  return (
    <Card>
      <h2>{t.heading}</h2>
      <p>{t.hint}</p>
      <p>{t.required}</p>
      <fieldset disabled={!ready || busy}>
        <ol className="module-settings-list">
          {rows.map((m, index) => (
            <li
              key={m.id}
              draggable
              onDragStart={(e) => {
                e.dataTransfer.setData('text/plain', m.id);
                setDrag(m.id);
              }}
              onDragOver={(e) => e.preventDefault()}
              onDrop={(e) => {
                e.preventDefault();
                if (e.dataTransfer.getData('text/plain') === drag)
                  setDraft(reorder(value, drag, m.id));
                setDrag('');
              }}
            >
              <strong>{m.name[locale]}</strong>
              <span className="secondary-text">
                {m.layer} · {m.status}
              </span>
              {(['enabled', 'hidden', 'pinned'] as const).map((k) => (
                <label key={k}>
                  <input
                    type="checkbox"
                    aria-label={`${m.name[locale]} ${t[k]}`}
                    checked={m.preference[k]}
                    disabled={k === 'enabled' && m.required}
                    onChange={(e) => change(m.id, k, e.target.checked)}
                  />
                  {t[k]}
                </label>
              ))}
              <Button
                variant="secondary"
                aria-label={`${m.name[locale]} ${t.up}`}
                disabled={index === 0 || rows[index - 1].preference.pinned !== m.preference.pinned}
                onClick={() => setDraft(reorder(value, m.id, rows[index - 1].id))}
              >
                {t.up}
              </Button>
              <Button
                variant="secondary"
                aria-label={`${m.name[locale]} ${t.down}`}
                disabled={
                  index === rows.length - 1 ||
                  rows[index + 1].preference.pinned !== m.preference.pinned
                }
                onClick={() => setDraft(reorder(value, rows[index + 1].id, m.id))}
              >
                {t.down}
              </Button>
            </li>
          ))}
        </ol>
        <Button onClick={() => void submit()}>{t.save}</Button>
        <Button
          variant="secondary"
          onClick={() => {
            void reload()
              .then(() => setDraft(null))
              .catch(() => setMessage(t.error));
          }}
        >
          {t.reload}
        </Button>
      </fieldset>
      {message && <p role="status">{message}</p>}
    </Card>
  );
}

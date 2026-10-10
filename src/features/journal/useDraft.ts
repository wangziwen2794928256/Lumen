import { useCallback, useEffect, useRef, useState } from 'react';
import { journalInvoke } from './api';
import { pendingKey, saveSchema, type Entry, type Save } from './model';
import { journalText as t } from './text';
export function useDraft(initial: Save, quick: boolean) {
  const [value, setValue] = useState(initial),
    [status, setStatus] = useState(''),
    [error, setError] = useState('');
  const latest = useRef(initial),
    revision = useRef(initial.revision),
    busy = useRef<Promise<boolean> | null>(null),
    alive = useRef(true),
    dirty = useRef(false);
  useEffect(() => {
    alive.current = true;
    return () => {
      alive.current = false;
    };
  }, []);
  const persist = useCallback(
    (draft: Save) => {
      try {
        localStorage.setItem(pendingKey(quick, draft.client_id), JSON.stringify(draft));
        return true;
      } catch {
        if (alive.current) setError(t.error);
        return false;
      }
    },
    [quick],
  );
  const change = useCallback(
    (patch: Partial<Save>) => {
      latest.current = {
        ...latest.current,
        ...patch,
        revision: revision.current,
        sequence: latest.current.sequence + 1,
      };
      dirty.current = true;
      persist(latest.current);
      setValue(latest.current);
      setStatus(t.dirty);
      setError('');
    },
    [persist],
  );
  const flush = useCallback(
    async function flush(): Promise<boolean> {
      if (busy.current) {
        const ok = await busy.current;
        if (!ok) return false;
        return flush();
      }
      if (!dirty.current) return true;
      const input = { ...latest.current, revision: revision.current };
      if (!saveSchema.safeParse(input).success) {
        if (alive.current) setError(t.error);
        return false;
      }
      const operation = (async () => {
        if (alive.current) setStatus(t.saving);
        try {
          const saved = await journalInvoke<Entry>('journal_save', { input });
          revision.current = saved.revision;
          latest.current = { ...latest.current, revision: saved.revision };
          if (latest.current.sequence === input.sequence) {
            dirty.current = false;
            try {
              const raw = localStorage.getItem(pendingKey(quick, input.client_id));
              const pending = raw ? saveSchema.parse(JSON.parse(raw)) : null;
              if (pending?.client_id === input.client_id && pending.sequence === input.sequence)
                localStorage.removeItem(pendingKey(quick, input.client_id));
            } catch {
              /* Server acknowledgement is durable even if local cleanup fails. */
            }
            if (alive.current) {
              setValue(latest.current);
              setStatus(t.saved);
              setError('');
            }
          } else {
            if (alive.current) persist(latest.current);
            if (alive.current) setStatus(t.dirty);
          }
          return true;
        } catch (e) {
          if (alive.current) {
            setStatus(t.dirty);
            setError(
              typeof e === 'object' && e && 'code' in e && e.code === 'JOURNAL_CONFLICT'
                ? t.conflict
                : t.error,
            );
          }
          return false;
        }
      })();
      busy.current = operation;
      const ok = await operation;
      busy.current = null;
      return ok;
    },
    [persist, quick],
  );
  useEffect(() => {
    if (!dirty.current) return;
    const timer = setTimeout(() => void flush(), 600);
    return () => clearTimeout(timer);
  }, [value, flush]);
  const publish = async () => {
    change({ publish: true });
    let ok = await flush();
    if (ok && dirty.current) ok = await flush();
    return ok;
  };
  return { value, change, status, error, flush, publish };
}

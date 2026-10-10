import { useCallback, useEffect, useRef, useState } from 'react';
import { loadToday, mutateToday } from './api';
import type { TodayAction, TodaySnapshot } from './model';
import { todayText as t } from './text';
function message(error: unknown) {
  const code = typeof error === 'object' && error !== null && 'code' in error ? error.code : '';
  return code === 'STATE_CONFLICT' ? t.conflict : code === 'DATABASE_BUSY' ? t.busy : t.error;
}
export function useToday() {
  const [data, setData] = useState<TodaySnapshot | null>(null),
    [browser, setBrowser] = useState(false),
    [loading, setLoading] = useState(true),
    [busy, setBusy] = useState(false),
    [error, setError] = useState('');
  const sequence = useRef(0),
    blocked = useRef(false),
    mounted = useRef(true);
  const refresh = useCallback(async () => {
    if (blocked.current) return;
    const request = ++sequence.current;
    try {
      const snapshot = await loadToday();
      if (mounted.current && request === sequence.current) {
        setData(snapshot);
        setBrowser(snapshot === null);
      }
    } catch (error) {
      if (mounted.current && request === sequence.current) setError(message(error));
    } finally {
      if (mounted.current && request === sequence.current) setLoading(false);
    }
  }, []);
  useEffect(() => {
    mounted.current = true;
    const initial = window.setTimeout(() => void refresh(), 0);
    const timer = window.setInterval(() => void refresh(), 1000);
    return () => {
      mounted.current = false;
      clearTimeout(initial);
      clearInterval(timer);
    };
  }, [refresh]);
  const act = useCallback(async (action: TodayAction) => {
    if (blocked.current) return false;
    blocked.current = true;
    const request = ++sequence.current;
    setBusy(true);
    setError('');
    try {
      const snapshot = await mutateToday(action);
      if (mounted.current && request === sequence.current) setData(snapshot);
      return true;
    } catch (error) {
      if (mounted.current) setError(message(error));
      return false;
    } finally {
      blocked.current = false;
      if (mounted.current) setBusy(false);
    }
  }, []);
  return { data, browser, loading, busy, error, refresh, act };
}

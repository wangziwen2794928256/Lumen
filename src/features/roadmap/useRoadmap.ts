import { useCallback, useEffect, useRef, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import type { RoadmapAction, RoadmapSnapshot } from './model';
import { roadmapText as t } from './text';
export function useRoadmap(week: string) {
  const [data, setData] = useState<RoadmapSnapshot | null>(null),
    [loading, setLoading] = useState(true),
    [error, setError] = useState(''),
    [busy, setBusy] = useState(false);
  const seq = useRef(0),
    mounted = useRef(false),
    blocked = useRef(false);
  const invalidate = useCallback(() => {
    ++seq.current;
  }, []);
  const refresh = useCallback(async () => {
    if (blocked.current) return;
    const n = ++seq.current;
    try {
      const value = isTauri() ? await invoke<RoadmapSnapshot>('roadmap_snapshot', { week }) : null;
      if (mounted.current && n === seq.current) {
        setData(value);
        setError('');
      }
    } catch {
      if (mounted.current && n === seq.current) setError(t.error);
    } finally {
      if (mounted.current && n === seq.current) setLoading(false);
    }
  }, [week]);
  useEffect(() => {
    mounted.current = true;
    const init = setTimeout(() => void refresh(), 0),
      timer = setInterval(() => void refresh(), 5000);
    return () => {
      mounted.current = false;
      invalidate();
      clearTimeout(init);
      clearInterval(timer);
    };
  }, [refresh, invalidate]);
  const act = useCallback(
    async (action: RoadmapAction) => {
      if (blocked.current || !isTauri()) return false;
      blocked.current = true;
      setBusy(true);
      setError('');
      const n = ++seq.current;
      try {
        const value = await invoke<RoadmapSnapshot>('roadmap_action', { week, action });
        if (mounted.current && seq.current === n) setData(value);
        return true;
      } catch (e) {
        const code = typeof e === 'object' && e && 'code' in e ? e.code : '';
        if (mounted.current)
          setError(
            code === 'STATE_CONFLICT'
              ? t.conflict
              : code === 'BUDGET_EXCEEDED' || code === 'MINIMUM_PROTECTED'
                ? t.budgetError
                : t.error,
          );
        return false;
      } finally {
        blocked.current = false;
        if (mounted.current) setBusy(false);
      }
    },
    [week],
  );
  return {
    data: data?.week_start === week ? data : null,
    loading,
    error,
    busy,
    browser: !isTauri(),
    refresh,
    act,
  };
}

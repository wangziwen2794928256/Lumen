import { useCallback, useEffect, useRef, useState } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import type { CalendarSnapshot } from './model';
import { calendarText as t } from './text';
export function useCalendar(from: string, until: string) {
  const [data, setData] = useState<CalendarSnapshot | null>(null),
    [error, setError] = useState(''),
    [loading, setLoading] = useState(true);
  const sequence = useRef(0),
    mounted = useRef(true);
  const invalidate = useCallback(() => {
    ++sequence.current;
  }, []);
  const refresh = useCallback(async () => {
    const request = ++sequence.current;
    try {
      const snapshot = isTauri()
        ? await invoke<CalendarSnapshot>('calendar_snapshot', { from, until })
        : null;
      if (mounted.current && request === sequence.current) {
        setData(snapshot);
        setError('');
      }
    } catch {
      if (mounted.current && request === sequence.current) setError(t.error);
    } finally {
      if (mounted.current && request === sequence.current) setLoading(false);
    }
  }, [from, until]);
  useEffect(() => {
    mounted.current = true;
    const initial = setTimeout(() => void refresh(), 0),
      timer = setInterval(() => void refresh(), 5000);
    return () => {
      mounted.current = false;
      invalidate();
      clearTimeout(initial);
      clearInterval(timer);
    };
  }, [refresh, invalidate]);
  return {
    data: data?.from === from && data.until === until ? data : null,
    error,
    loading,
    refresh,
  };
}

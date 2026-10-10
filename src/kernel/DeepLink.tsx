import { useEffect, useRef } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import { listen } from '@tauri-apps/api/event';
export function DeepLink() {
  const navigate = useNavigate();
  const handled = useRef(false);
  useEffect(() => {
    if (!isTauri()) return;
    let disposed = false;
    let unlisten: (() => void) | undefined;
    void listen<string>('lumen:navigate', ({ payload }) => {
      if (/^\/(?:focus|calendar|journal|vault|roadmap)?(?:\?[a-z]+=[\w-]+)?$/.test(payload))
        navigate(payload);
    })
      .then((stop) => {
        if (disposed) stop();
        else unlisten = stop;
      })
      .catch(() => {});
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, [navigate]);
  useEffect(() => {
    if (isTauri() && !handled.current) {
      handled.current = true;
      void invoke<string | null>('deep_link_initial')
        .then((route) => {
          if (route) navigate(route, { replace: true });
        })
        .catch(() => {});
    }
  }, [navigate]);
  return null;
}

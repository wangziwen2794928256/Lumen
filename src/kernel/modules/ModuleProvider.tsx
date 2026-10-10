import { createContext, useContext, useEffect, useState, type ReactNode } from 'react';
import { invoke, isTauri } from '@tauri-apps/api/core';
import { defaults, normalizeState } from './registry';
import { registryStateSchema, type RegistryState } from './model';
const storage = 'lumen.kernel.modules.v1';
function browserRead() {
  try {
    return normalizeState(
      registryStateSchema.parse(JSON.parse(localStorage.getItem(storage) ?? 'null')),
    );
  } catch {
    return defaults;
  }
}
const Context = createContext<{
  state: RegistryState;
  ready: boolean;
  error: string;
  save: (s: RegistryState) => Promise<void>;
  reload: () => Promise<void>;
}>({ state: defaults, ready: false, error: '', save: async () => {}, reload: async () => {} });
export function ModuleProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<RegistryState>(() => (isTauri() ? defaults : browserRead())),
    [ready, setReady] = useState(!isTauri()),
    [error, setError] = useState('');
  async function reload() {
    const value = isTauri()
      ? await invoke<RegistryState | undefined>('module_registry_get')
      : browserRead();
    setState(value ? normalizeState(registryStateSchema.parse(value)) : defaults);
    setReady(true);
    setError('');
  }
  useEffect(() => {
    let live = true;
    if (isTauri())
      void invoke<RegistryState | undefined>('module_registry_get')
        .then((v) => {
          if (live) {
            setState(v ? normalizeState(registryStateSchema.parse(v)) : defaults);
            setReady(true);
          }
        })
        .catch(() => {
          if (live) {
            setReady(true);
            setError('模块配置读取失败；显示默认导航，数据库权限检查仍生效。请在设置重新载入。');
          }
        });
    return () => {
      live = false;
    };
  }, []);
  async function save(value: RegistryState) {
    const next = normalizeState(registryStateSchema.parse(value));
    if (isTauri()) {
      const stored = await invoke<RegistryState>('module_registry_save', { value: next });
      setState(normalizeState(registryStateSchema.parse(stored)));
    } else {
      const actual = browserRead();
      if (actual.revision !== next.revision) throw Error('MODULE_CONFLICT');
      const stored = { ...next, revision: next.revision + 1 };
      localStorage.setItem(storage, JSON.stringify(stored));
      setState(stored);
    }
  }
  return (
    <Context.Provider value={{ state, ready, error, save, reload }}>{children}</Context.Provider>
  );
}
// Provider is part of the Kernel and cannot be supplied by community code.
// eslint-disable-next-line react-refresh/only-export-components
export function useModules() {
  return useContext(Context);
}

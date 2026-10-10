import catalogue from './catalogue.json' with { type: 'json' };
import { manifestSchema, type Manifest, type RegistryState } from './model';
export function validateCatalogue(input: unknown): Manifest[] {
  const manifests = manifestSchema.array().parse(input);
  if (
    new Set(manifests.map((m) => m.id)).size !== manifests.length ||
    new Set(manifests.map((m) => m.route)).size !== manifests.length ||
    manifests.some((m) => !m.id.startsWith('lumen.') || !['kernel', 'official'].includes(m.layer))
  )
    throw Error('INVALID_BUNDLED_REGISTRY');
  return manifests;
}
export const manifests = validateCatalogue(catalogue);
export const defaults: RegistryState = {
  revision: 0,
  items: manifests.map((m, order) => ({
    id: m.id,
    enabled: true,
    hidden: false,
    pinned: false,
    order,
  })),
};
export function normalizeState(state: RegistryState): RegistryState {
  if (
    new Set(state.items.map((i) => i.id)).size !== state.items.length ||
    state.items.some((i) => !manifests.some((m) => m.id === i.id))
  )
    throw Error('UNKNOWN_MODULE');
  return {
    ...state,
    items: defaults.items.map((d) => {
      const missing = defaults.items.filter((x) => !state.items.some((i) => i.id === x.id));
      const next = Math.max(-1, ...state.items.map((i) => i.order)) + 1;
      const i = state.items.find((i) => i.id === d.id) ?? {
        ...d,
        order: next + missing.findIndex((x) => x.id === d.id),
      };
      const m = manifests.find((m) => m.id === i.id)!;
      if (m.required && !i.enabled) throw Error('KERNEL_REQUIRED');
      return { ...i };
    }),
  };
}
export function ordered(state: RegistryState, sidebar = false) {
  return manifests
    .map((m) => ({ ...m, preference: state.items.find((i) => i.id === m.id)! }))
    .filter((m) => !sidebar || (m.preference.enabled && !m.preference.hidden))
    .sort(
      (a, b) =>
        Number(b.preference.pinned) - Number(a.preference.pinned) ||
        a.preference.order - b.preference.order ||
        a.id.localeCompare(b.id),
    );
}
export function reorder(state: RegistryState, id: string, before: string): RegistryState {
  const list = ordered(state),
    source = list.find((m) => m.id === id),
    target = list.find((m) => m.id === before);
  if (
    !source ||
    !target ||
    source.id === target.id ||
    source.preference.pinned !== target.preference.pinned
  )
    return state;
  const ids = list.map((m) => m.id).filter((m) => m !== id);
  ids.splice(ids.indexOf(before), 0, id);
  return { ...state, items: state.items.map((i) => ({ ...i, order: ids.indexOf(i.id) })) };
}

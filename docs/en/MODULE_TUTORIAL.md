# Your first official module

This tutorial builds a source-integrated, trusted module named `lumen.study-lab`. It runs in the application renderer with the same trust as existing official modules. It is **not** a way to execute an untrusted community plugin. Work in a fork and a feature branch. Do not use production AppData for development launches.

## 1. Prepare the source

Follow [environment setup](ENVIRONMENT_AND_CONFIGURATION.md), then run `pnpm install --frozen-lockfile`. Read [module inventory](MODULE_REFERENCE.md) and [API reference](API_REFERENCE.md). Start with a read-only page. It needs no schema change or new permission.

## 2. Add the page and module entry

Create `src/modules/official/study-lab/index.tsx`:

```tsx
import { Card } from '../../../components/ui/card';
import { locale } from '../../../lib/locale';
export function StudyLabRoute() {
  return <Card>
    <h1>{locale === 'en' ? 'Study Lab' : '学习实验室'}</h1>
    <p>{locale === 'en' ? 'Build a workflow that fits your goal.' : '为自己的目标编写工作流。'}</p>
  </Card>;
}
```

For substantial UI, put the page under `src/pages/study-lab` and export a lazy route from this entrypoint, following `src/modules/official/profile/index.tsx`. Keep stateful services in their existing domains. Add copy to language resources rather than translating user data.

## 3. Register the manifest

Append this object to `src/kernel/modules/catalogue.json`:

```json
{
  "id": "lumen.study-lab",
  "name": { "zh-CN": "学习实验室", "en": "Study Lab" },
  "version": "0.1.0",
  "api_version": "1.0",
  "layer": "official",
  "route": "/study-lab",
  "icon": "NotebookPen",
  "required": false,
  "status": "module",
  "permissions": []
}
```

`validateCatalogue` only accepts bundled `lumen.*` entries in kernel/official layers. IDs and routes must be unique. API version must be `1.0`. Names need both languages. Routes use lowercase path syntax. Icon names are resolved by the shell's icon map, with a safe fallback. Unknown fields fail strict validation. Adding a manifest contributes sidebar configuration, not executable code loading.

## 4. Register the trusted route

In `src/app/router.tsx`, add the import:

```tsx
import { StudyLabRoute } from '../modules/official/study-lab';
```

Within the existing `<Routes>` add:

```tsx
<Route path="/study-lab" element={
  <ModuleGate id="lumen.study-lab"><StudyLabRoute /></ModuleGate>
} />
```

Now the sidebar comes from Registry while the executable component remains an explicit bundled import. `normalizeState` appends a new module without losing existing preferences. Disabled routes show the standard message. Hiding affects navigation only. Pinning sorts a group first. Dragging reorders within the same pinned group. Keep keyboard move buttons and optimistic-revision error handling. If you choose lazy loading, add an optional prefetch entry following `src/app/preload.ts`; prefetch must never mutate data.

## 5. Add a Today widget, if useful

Read `src/modules/official/insights/index.tsx`: `insightsWidgets` contributes a component at `today.after-summary`. A descriptor has `id`, `moduleId`, `slot`, `order`, `Component`. To add your own trusted descriptor, create an exported array in your entrypoint, import it in `src/kernel/modules/Extensions.tsx`, and merge it with the existing descriptors before filtering/sorting. The current widget list is a static bundled list, not a generic third-party loader.

Keep `TodayWidgets` filtering by enabled module and wrapping each component in `WidgetBoundary`. A failed widget must not crash Today. Do not request arbitrary paths, SQL, HTML scripts or automatic writes. Use typed domain clients only for data the feature actually needs.

## 6. Add persistence only when required

Read-only UI needs no migration. For new persisted facts, add the next numbered migration after 010, add it to `database/migrations.rs`, document schema/indices/relationships and test upgrade and rollback on TempDir copies. Never edit 001–010. Kernel owns database access. Add a typed command, Rust validation, parameterized repository methods and an idempotent request/revision strategy before exposing a frontend client.

Do not put OS detection inside the module. Extend Kernel's platform capability boundary and return explicit unsupported errors. Do not grant community plugins official IPC access.

## 7. Verify the integration

```sh
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
pnpm test:e2e
node --experimental-strip-types scripts/check-contracts.mjs
```

Verify unique manifest/route validation, new-module defaults, disabled-route gating, hidden/pinned/order persistence, keyboard reordering, widget error isolation and records surviving disable/enable. Check both languages, both themes and all three test viewports. Add Rust tests for any new service/storage behavior. Browser fixtures do not prove native IPC or platform integration.

## Troubleshooting

| Symptom | Check |
| --- | --- |
| Registry rejects the application | Invalid API, namespace, duplicate route/id or unknown manifest key |
| Sidebar item exists but page redirects | Missing explicit Route registration |
| Module disappears from sidebar | hidden=true or enabled=false in preferences |
| Page says disabled | Enable the module and save in Settings |
| Save conflict | Reload Registry and reapply choices explicitly |
| Widget missing | Descriptor import/merge, slot name and enabled state |
| New data changes old records | Stop. Review migration, transactions and relationship ownership |

For external plugins, read [Plugin SDK](PLUGIN_SDK.md). Community `enable` currently returns `PLUGIN_RUNTIME_DEFERRED`; changing a database state cannot authorize execution.

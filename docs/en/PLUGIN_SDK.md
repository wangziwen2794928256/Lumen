# Community Plugin SDK contract

**Status:** interface design and validators only. There is no executable community SDK, marketplace, installer or full Wasmtime host. Passing manifest validation never grants execution. For a working source integration, use the [official-module tutorial](MODULE_TUTORIAL.md).

## Choose the right extension

| Need | Path available now | Trust |
| --- | --- | --- |
| New page, trusted data workflow | Fork and integrate an Official Module | Reviewed application source |
| Today panel | Static trusted widget descriptor | Same trust as official renderer |
| Colors/layout | Built-in theme tokens in source | Reviewed CSS, no runtime pack loader |
| Untrusted distributed plugin | Design against contracts below | Execution deliberately unavailable |

## Package manifest 1.0

```json
{
  "id": "org.example.revision",
  "version": "0.1.0",
  "api_version": "1.0",
  "runtime": "wasm-component",
  "entry": "plugin.wasm",
  "permissions": ["storage.own", "widget.contribute"]
}
```

The community schema is [community-manifest.schema.json](../contracts/community-manifest.schema.json). Kernel's [permissions.rs](../../src-tauri/src/kernel/permissions.rs) additionally reserves `lumen.*`. IDs are at most 80 bytes with lowercase dot-separated segments. Version has three numeric components. API is exactly `1.0`, runtime exactly `wasm-component` and entry exactly `plugin.wasm`. At most three unique known capabilities are allowed. Extra fields, native entrypoints, arbitrary paths and network/filesystem/SQL permissions fail validation.

This differs from the bundled module manifest, which includes bilingual navigation names and explicit trusted routes. Never put a community manifest into the bundled catalogue or claim official status to gain trust.

## Planned host API

The draft ABI is [lumen-plugin.wit](../contracts/lumen-plugin.wit), package `lumen:plugin@1.0.0`:

| Import | Shape | Intended authority |
| --- | --- | --- |
| storage.get | key → optional bytes / error | Own opaque namespace only |
| storage.put | key, bytes → success / error | Own namespace, bounded writes |
| widgets.contribute | declarative widget JSON → result | Validated host-rendered content |
| plans.propose | Plan Exchange JSON → result | Reviewable proposal, no task writes |
| initialize (export) | no arguments → result | Component initialization |
| shutdown (export) | no arguments | Component cleanup |

No host path, raw SQL, Tauri invoke, arbitrary HTML/script or ambient network handle appears. Wasmtime/WASI are not dependencies today. Future implementation must deny ambient WASI filesystem and sockets, bind host calls to current grants and enforce memory, fuel/deadline, calls, response size, storage quota and concurrent worker bounds. Those resource values are not finalized and cannot be claimed as enforced.

## Grants and revocation

A declaration is a request. Kernel's `Grant` binds plugin_id, version, content_hash, capabilities and revoked. `check_capability` validates identity, version, 64-character hexadecimal loaded hash, matching grant hash, declared capability, granted capability and revocation. Changed bytes or version cannot reuse stale grants. The current `authorize_community_execution` always returns `PLUGIN_RUNTIME_DEFERRED` even if capability checks pass. There is no grant UI yet.

Future opaque handles must be short-lived and scoped. Disabling or uninstalling must stop the worker and revoke handles before state transitions. Permission expansion during update requires explicit renewed consent.

## Lifecycle states

Current pure state transitions are tested design logic, not file install/update executors:

| Current | Action | Result |
| --- | --- | --- |
| Staged | install | Disabled |
| Disabled | enable | PLUGIN_RUNTIME_DEFERRED |
| Enabled | disable | Disabled |
| Disabled | update | UpdateStaged |
| UpdateStaged | commit / rollback | Disabled |
| Disabled | rollback | RollbackStaged |
| RollbackStaged | commit | Disabled |
| Disabled | uninstall | Uninstalled |
| Other combinations | any | Rejected |

Future staging must verify package identity/hash/API and archive containment before activation. Update must keep the last verified version and isolated data migration/rollback strategy. Uninstall should separately ask about deleting isolated plugin data and must never remove core records. Do not directly edit state tables to simulate these operations.

## Data layout and ownership

Kernel keeps the existing AppData/lumen.db. Isolated plugin storage resolves under plugins/<id> only. `plugin_namespace` validates the ID, canonicalizes an existing namespace and rejects escape outside plugins, including links. It does not create an executable runtime or authorize arbitrary user paths. extensions/<id>/<version> is the planned package area, separate from plugin data. Themes, attachments, backups, exports, logs and cache have distinct roles.

A future plugin database belongs to that plugin. It cannot attach or query core SQLite. Needed core facts require a separately designed bounded host projection with user permission and no internal SQL surface.

## Module API versioning

App version, module package version and API version are independent. API `1.0` is the only accepted version now; unsupported versions fail closed. A future additive minor negotiation requires explicit capability/schema compatibility. Breaking changes need a new major and documented deprecation/adapter period. Update paired TS/Rust fixtures, JSON Schema and WIT together. Do not infer compatibility from a matching app version.

## Planner exchange

Use [plan-exchange.schema.json](../contracts/plan-exchange.schema.json) and [exchange.ts](../../src/kernel/plan/exchange.ts). A proposal supplies schema_version, request_id, basis_hash, source, capacity_minutes and items. Items contain task_id, date and minutes. Limits: 500 items, 1–1440 minutes each, unique task/date pairs and total ≤ capacity. Rust must also verify references, current basis, dates, task remaining work and daily feasibility before a user accepts. JSON Schema alone cannot prove cross-field validity.

PlanContext can carry optional labels, which may reveal private task content. It is a contract only. No external LLM or local model runs, no context leaves the device automatically, and proposals never auto-accept.

## Security test plan for a future runtime

Test malformed archives/path traversal/symlinks, forged official IDs, unknown permissions, stale hashes/grants, permission revocation mid-call, resource exhaustion, malicious declarative widgets, oversized data, namespace escape, bad updates and rollback failures. Test that plugins cannot read core SQLite, backups, exports or credentials. An isolated process plus capability checks still requires sandbox verification. Full runtime acceptance remains deferred.

Current checks: `node --experimental-strip-types scripts/check-contracts.mjs`, module frontend tests and Rust Kernel tests. See [permission model](PERMISSION_MODEL.md), [threat model](THREAT_MODEL.md) and [testing](TESTING.md).

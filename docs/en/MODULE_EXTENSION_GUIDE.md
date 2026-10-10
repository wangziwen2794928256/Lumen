# Module extension guide

This covers official module integration and future community contracts. Manifest validation never authorizes third-party execution.

## Official integration

Read the initial migration at src/modules/official/insights, then src/modules/official/profile. Keep the existing Service/Repository and add a bounded module entry. The bundled catalogue lives at src/kernel/modules/catalogue.json. validateCatalogue accepts only lumen.* kernel/official entries and rejects duplicate ids/routes.

Illustrative data, not a runtime install command:

```json
{
  "id": "lumen.example",
  "name": {"zh-CN": "示例", "en": "Example"},
  "version": "0.1.0",
  "api_version": "1.0",
  "layer": "official",
  "route": "/example",
  "icon": "Leaf",
  "required": false,
  "status": "module",
  "permissions": []
}
```

Register the route/provider and widget contribution using existing patterns. Platform decisions stay in Kernel. Registry contains revision and items with id/enabled/hidden/pinned/order. Respect optimistic revision conflicts; do not silently overwrite another window. Pinned groups appear first, and dragging reorders within the same pinned group. Keep keyboard move buttons.

Acceptance covers invalid manifest/duplicate route rejection, disabled-route gates, persisted hidden/pinned/drag preferences, preserved records after restart, and widgets requesting no ambient data access. New storage requires a new numbered migration.

## Community manifest and grants

This is a separate contract from the bundled catalogue:

```json
{
  "id": "org.example",
  "version": "0.1.0",
  "api_version": "1.0",
  "runtime": "wasm-component",
  "entry": "plugin.wasm",
  "permissions": ["storage.own", "widget.contribute"]
}
```

The current validator accepts up to three unique capabilities: storage.own, widget.contribute and plan.propose. lumen.* is reserved. Unknown fields, arbitrary paths, native runtimes, network/filesystem/SQL permissions are rejected. Grants bind plugin_id, version, content_hash, capabilities and revoked. Changed code/version cannot reuse stale authorization.

authorize_community_execution always returns PLUGIN_RUNTIME_DEFERRED; lifecycle enable remains rejected. These examples are data contracts, not an installer or executable SDK. Do not bypass Kernel by editing state tables.

## Future host and planning API

docs/contracts/lumen-plugin.wit sketches opaque-key storage.get/put, declarative JSON widgets.contribute and data-only plans.propose. No SQL, host path, HTML script or Tauri IPC is exposed. Future implementation must enforce call/key/value bounds, memory/time/resource budgets and prompt revocation/termination.

Plan Exchange1.0 allows at most 500 items, unique task/date pairs, 1–1440 minutes per item, a capacity-bounded total and basis_hash for staleness. Proposals require manual acceptance; they do not directly mutate tasks or automatically schedule. Execute cross-field TS/Rust checks as well as the JSON Schema.

Run node --experimental-strip-types scripts/check-contracts.mjs, frontend module unit/E2E and Rust Kernel tests. Full WASM host, sandbox, actual install/update executor and AI invocation remain deferred.

# TESTING

Run scripts/verify/phase16.ps1 for lint, format, typecheck, unit, build, rust-fmt, Rust tests including migrations, clippy and the full E2E suite. UI gates cover three sizes, light/dark themes, drag/keyboard ordering, enabled/hidden/pinned, reload and form spacing. Phase16 evidence records 35 frontend tests, 72 Rust tests and 60 E2E cases; the ignored ten-year benchmark is executed separately.

Negative Kernel tests cover required-module enforcement, unknown/duplicate IDs, stale revisions, disabled IPC, unsupported API versions, forbidden SQL/files/network/native permissions, closed runtime, invalid paths/proposals, capacity overflow and append-only observations. Existing migration rollback and Windows message cleanup tests remain.

Native operations and production upgrade have separate evidence. Compare pre-upgrade columns/rows by SHA256 without storing bodies. macOS/Linux builds, full Wasm isolation and AI invocation are deferred/NOT RUN, never PASS.

Matching Chinese document：[TESTING](../zh-CN/TESTING.md)。

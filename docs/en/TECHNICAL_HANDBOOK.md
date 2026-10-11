# Lumen technical handbook

## Detailed reading paths

This handbook provides the system overview. [MODULE_REFERENCE](MODULE_REFERENCE.md) explains workflows, mechanisms and boundaries for each module. [MODULE_TUTORIAL](MODULE_TUTORIAL.md) gives source integration steps. [API_REFERENCE](API_REFERENCE.md) indexes 60 IPC signatures and 12 domain model groups. [ENVIRONMENT_AND_CONFIGURATION](ENVIRONMENT_AND_CONFIGURATION.md) covers prerequisites, isolation and packaging. Begin with [USER_GUIDE](USER_GUIDE.md), explore [EXTENSION_IDEAS](EXTENSION_IDEAS.md) and see [LOCALIZATION](LOCALIZATION.md) for English UI coverage.


Version 0.1.0 candidate · Documentation and localization edition, PPT v4 revision · 2026-10-11

## Product and support

Lumen connects tasks, roadmaps, focus, calendar, journals, search, insights, coding practice, music, portfolio and profile in a local desktop workspace. A previous Windows x64 unsigned candidate passed local native launch, installation, upgrade and uninstall-with-data-retention acceptance. The current localized package passed build and browser regression checks without repeated native installation acceptance. This PPT revision does not rebuild the app. macOS Apple Silicon/Intel and Linux x64 are experimental build targets. Compilation and browser tests do not establish native feature parity.

Manual operation is the default. AI planning has exchange schemas only: no actual model calls, training or background data upload. The community marketplace, full WASM runtime and mobile apps are deferred.

## Architecture and source map

React pages call typed IPC clients. Tauri commands validate inputs and call Rust Kernel/domain services. Parameterized repositories own SQLite access. The frontend never opens core SQLite directly.

| Layer | Responsibility | Boundary |
| --- | --- | --- |
| Kernel | Security, data, backup, IPC, permissions, lifecycle, platform services | Security cannot be uninstalled |
| Official Modules | Existing product functionality and the first registry migration | Reuse stable services; migrate incrementally |
| Community Modules | Manifest, capability and future runtime contracts | No arbitrary third-party native execution |
| Theme Packs | Tokens and visual resources | No database, filesystem or network access |

`src/app` and `src/pages` contain the shell and pages. `src/kernel/modules` owns registry/navigation configuration; `src/kernel/plan` owns planning exchange validation. `src/lib` contains clients and shared utilities. `src-tauri/src/lib.rs` initializes Tauri plugins/services and registers commands. `src-tauri/src/commands` contains the IPC boundary. `src-tauri/src/kernel` owns storage, permissions, export and desktop services. Platform selection stays in `kernel/platform`. `src-tauri/src/database` and `src-tauri/migrations` own repositories and schema upgrades. `docs/contracts` contains JSON schemas and future host WIT.

Sidebar state comes from Module Registry, including enabled, hidden, pinned and order. Hiding/disabling preserves records. Today exposes a widget contribution point. Appearance colors are managed in settings.

## Storage and migrations

Tauri's AppData API selects the production root; do not hardcode workstation paths. The stable core file remains `lumen.db` in that root. Planned logical areas cover isolated `plugins/<id>`, attachments, backups, exports, extensions, themes, logs and cache; check Kernel storage code for currently active paths. Directory planning does not imply a complete plugin runtime.

Read schema version from `MAX(migration_history.version)`, currently 10, rather than PRAGMA user_version. Append numbered migrations; never edit released migrations. Back up before upgrading and roll back failed migrations. Use parameterized SQL. Back up a live database consistently; copying a bare file while ignoring WAL is not a backup strategy.

Planning contracts/fields preserve estimates, actual time, capacity, plan versus actual, adjustments and accept/reject history. These retain manual decisions and do not train models. Community plugins own isolated namespaces and must never read core `lumen.db`. Their update, rollback or removal cannot transfer core data ownership. See [DATABASE](DATABASE.md) and the [planning domain](../data/ROADMAP_PLANNER.md).

## Module and plugin evolution

Validate stable module id, module version, Module API version, navigation/routes, contribution points and permission requests before registration. Keep existing service code and use the initial official migration as the example. The planned lifecycle covers install, enable, disable, staged update, rollback and uninstall.

Kernel permission checks are deny-by-default and scoped to identity, capability and user grant. A future Wasmtime/WASM host should expose bounded data APIs with call, result-size, memory, time and resource limits. Arbitrary SQL, filesystem paths and network sockets are outside the proposed host contract. Current WIT and schemas are design interfaces, not an accepted sandbox. Do not enable community execution before runtime isolation, revocation, rollback and security tests are complete.

## Platform behavior

Business modules call the Kernel platform abstraction. Windows implements lock/unlock and suspend/resume handling. macOS/Linux declare session_events=false. Their current Today Service initialization returns `NATIVE_HOOK_UNAVAILABLE`; native Today/focus commands therefore cannot be described as supported. Browser mocks passing are unrelated to native session safety.

Media monitoring returns `MEDIA_UNSUPPORTED` on these targets; manual music records remain the intended fallback. Attachment/data-directory opening, preview browser opening and deep-link registration return explicit unsupported errors. Tauri cross-platform tray, notification and autostart APIs exist, but actual OS registration, delivery and permissions remain unverified. Do not enable focus counting without a safe session-event implementation.

## Reproducible build procedure

Use Node 24.15.0, pnpm 11.19.0, Rust 1.98.1 and both committed lockfiles. Install the target OS's [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings
node --experimental-strip-types scripts/check-contracts.mjs
```

Windows E2E defaults to Edge. CI sets LUMEN_E2E_BROWSER=chromium, installs Playwright Chromium, and runs pnpm test:e2e at 1280×720, 1440×900 and 1920×1080. Browser fixtures cover themes, keyboard access, form spacing, module transitions and privacy flows, but do not replace native IPC acceptance.

macOS packaging: `pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked`. Linux: replace bundles with `deb,appimage`; Ubuntu 22.04 needs WebKitGTK 4.1, GTK3, AppIndicator, FUSE2 and build libraries. The manually dispatched GitHub workflow uses native runners, contents:read permissions and commit-pinned Actions. It has no signing or publication credentials.

Windows: use scripts/release/build-windows.ps1. The reference workstation had recorded CPU WHEA events; its build workers use an explicitly verified affinity mask. FFFF0000 is machine-specific and must not be copied to unrelated hardware. It does not restrict the normal application process.

## Linux security backport

Tauri's GTK graph requires glib 0.18.5. [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429.html) describes undefined behavior in VariantStrIter. The checked-in vendor source backports the two mutable-output-pointer lines from [upstream PR1343](https://github.com/gtk-rs/gtk-rs-core/pull/1343), preserving API/version. An optimized regression tests forward/backward iteration, nth/nth_back, last and empty exhaustion. See src-tauri/vendor/glib/LUMEN_PATCH.md.

Version-based cargo audit may still report 0.18.5. This is a scoped source patch, not a claim that upstream 0.18.5 is patched. Do not suppress all advisories. Remove the patch only when the maintained Tauri/GTK graph accepts a supported glib >=0.20 and platform acceptance passes.

## Release and publication

Distinguish compilation, unit/browser regression, install, real native window, OS integration, signing and notarization. Current candidates have no commercial signing certificate or Apple notarization. Never ask users to disable Gatekeeper, SmartScreen or other protections. Test experimental builds under an isolated OS user, keeping the only production database safe.

The owner has authorized public source and editable materials. Publication uses a sanitized source-only snapshot. export-source.py allowlists code and technical docs, excluding local Git history, production databases, journals, music history, native personal screenshots, workstation logs and credentials. UPSTREAM_CHECKPOINT.json preserves source provenance and hashes. Original local phase history remains intact. Release packages and SHA256 must identify the producing commit. A Windows Phase16 binary remains labelled Phase16, rather than being presented as a new native CI build.

## Troubleshooting and maintenance

- Build failure: inspect the relevant runner log and toolchain.txt before changing dependencies.
- NATIVE_HOOK_UNAVAILABLE: missing platform adapter, not evidence of database corruption.
- Schema/integrity error: stop writes, preserve the original DB/WAL, use consistent backups and isolated restore. Never reset production data.
- OS installation block: follow the OS's supported approval path or wait for signing; do not disable protections.
- Expired CI artifact: use a verified retained copy or rebuild from the recorded source/toolchain. Rebuilding does not guarantee identical bytes; publish new checksums.
- Issue reports: include app version, OS/architecture, reproduction and error code. Remove private content, private paths and credentials.

The next acceptance sequence is native packages/security regression, real macOS/Linux session events and Today/focus, then media/attachments/preview/deep links, tray/notifications/autostart, install/upgrade/uninstall data retention, and legitimate signing/notarization. Full plugin execution and AI planning remain separate scopes.

See the current [requirements and UI audit](REQUIREMENTS_AUDIT.md) for coverage and remaining gaps.

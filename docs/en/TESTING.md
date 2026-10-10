# TESTING

Run the explicit commands in BUILD_FROM_SOURCE.md for lint/format/typecheck/unit/build/Rust formatting/tests including migrations/contracts. Windows also runs `cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings`. For Chromium E2E, install `pnpm exec playwright install --with-deps chromium`, set LUMEN_E2E_BROWSER=chromium and run pnpm test:e2e; Windows defaults to Edge. UI gates cover three sizes, light/dark themes, drag/keyboard ordering, enabled/hidden/pinned, reload and form spacing. Phase17 Windows has35 frontend/72 Rust/60 Edge E2E PASS. macOS library tests have71 pass and1 ignored; Linux adds a separate glib integration regression in debug/release. Counts/results come from actual logs. Original verify scripts remain local-only.

Negative Kernel tests cover required-module enforcement, unknown/duplicate IDs, stale revisions, disabled IPC, unsupported API versions, forbidden SQL/files/network/native permissions, closed runtime, invalid paths/proposals, capacity overflow and append-only observations. Existing migration rollback and Windows message cleanup tests remain.

verify-packages.yml checks read-only DMG mount/Bundle/Mach-O on macOS and deb/AppImage metadata/extraction/ELF on Linux, requiring complete packages and exact hashes. Extraction does not establish native launch or installation. macOS/Linux deferred adapters currently produce dead-code warnings; Windows Clippy PASS does not imply those platforms passed -D warnings. Native window, lock/sleep, tray/notification/autostart, attachment and install/upgrade/uninstall acceptance stay separate. Compare production columns/rows by SHA256 without private bodies. Full Wasm isolation, actual AI, macOS/Linux native launch/install/signing/notarization are NOT RUN. Session events/Today/focus are unsupported there.

Matching Chinese document：[TESTING](../zh-CN/TESTING.md)。


## Phase17 distribution update

Native macOS ARM64/Intel and Linux x64 workflows now live in `.github/workflows/native-build.yml`. Build results must come from the actual run; compilation does not establish OS integration support. See [current handbook](TECHNICAL_HANDBOOK.md) and [installation matrix](INSTALLATION.md) for complete commands, platform gaps, the glib backport, private source publication and retained local evidence. Historical Phase16 reports/build scripts are local acceptance references; they are not all included in the allowlisted GitHub source snapshot.

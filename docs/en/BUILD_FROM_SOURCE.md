# BUILD_FROM_SOURCE

Use Node24.15.0, pnpm11.19.0 (packageManager), Rust1.98.1 (rust-toolchain.toml) and committed lockfiles. Install the target OS's [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/). Codex is not required. In a standard installed Node/pnpm/Rust environment:

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --locked
node --experimental-strip-types scripts/check-contracts.mjs
```

For development, use a dedicated CARGO_TARGET_DIR, CARGO_BUILD_JOBS=1 and pnpm tauri build --debug --no-bundle. Windows needs WebView2 and MSVC SDK. QA requires an isolated app identifier/profile and must never write production storage.

macOS14+: `pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked`. Ubuntu22.04 x64: replace bundles with `deb,appimage`; install WebKitGTK4.1/GTK3/AppIndicator/OpenSSL/FUSE2/patchelf/librsvg and build libraries. Windows: `pnpm tauri build --config src-tauri/tauri.release.conf.json --bundles nsis -- --locked`. CI records opt2/codegen1, jobs2 and incremental0; Windows reference uses jobs1. Development launches must use an isolated OS user or QA identifier/profile because debug alone does not isolate AppData. Source is MIT; dependencies retain their licenses.

Matching Chinese document：[BUILD_FROM_SOURCE](../zh-CN/BUILD_FROM_SOURCE.md)。

Phase16 packaging: Rust minimum1.90; project-pinned compiler1.98.1. `scripts/release/build-windows.ps1` produces an optimized current-user installer with the verified opt2/codegen1 profile and locked dependencies. Base config remains unbundled for development. Missing WebView2 uses Microsoft bootstrapper; no security policy bypass. Signing needs an owner-supplied certificate; unsigned candidates are labeled.

The optional owner wrapper scripts/release/build-windows.ps1 defaults to D:\LumenToolchain (override LUMEN_TOOLCHAIN_ROOT), and -TargetDirectory must match your machine. Standard builds above do not require that layout. On the reference i9-14900HX, FFFF0000 constrains only the worker to its verified efficiency cores after a WHEA warning; omit this host-specific mask elsewhere. Historical Phase16 toolchain/acceptance reports and verify scripts are retained locally, not part of the source-only GitHub snapshot.


## Phase17 distribution update

Native macOS ARM64/Intel and Linux x64 workflows now live in `.github/workflows/native-build.yml`. Build results must come from the actual run; compilation does not establish OS integration support. See [current handbook](TECHNICAL_HANDBOOK.md) and [installation matrix](INSTALLATION.md) for complete commands, platform gaps, the glib backport, private source publication and retained local evidence. Historical Phase16 reports/build scripts are local acceptance references; they are not all included in the allowlisted GitHub source snapshot.

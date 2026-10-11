# Environment and configuration

This guide targets a standard developer workstation. Codex, an AI account, API keys and a cloud backend are not required. The app works manually and locally. See [installation](INSTALLATION.md) if you only want to run a package.

## Supported toolchain

| Component | Project pin | Purpose / check |
| --- | --- | --- |
| Node.js | 24.15.0, package engine >=24.15.0 <25 | Frontend/build tools, `node --version` |
| pnpm | 11.19.0 | Lockfile installation, `pnpm --version` |
| Rust | 1.98.1 in rust-toolchain.toml | Rust/Tauri, `rustc --version` |
| TypeScript / React / Tauri | package.json + lockfiles | Do not replace with unpinned latest versions |
| Native SDK | Depends on host | Compiler, system libraries and WebView |
| Python | For distribution verification scripts | Not needed for ordinary app use |

Install official Node and Rust/rustup, then `npm install --global pnpm@11.19.0` if pnpm is absent. Rustup reads the project toolchain file. Dependencies install with `pnpm install --frozen-lockfile`; Cargo commands use `--locked`. Keep both lockfiles in contributions.

## Native prerequisites

Use the maintained [Tauri prerequisite guide](https://v2.tauri.app/start/prerequisites/) for the exact OS/compiler setup.

- **Windows x64:** Microsoft C++ Build Tools with Desktop development with C++, MSVC and Windows SDK, plus WebView2 Runtime. Rust uses the MSVC target. Installing an IDE without its C++ workload is insufficient.
- **macOS:** Xcode Command Line Tools and the target architecture's Rust toolchain. The current experimental configuration sets minimum macOS 14. Apple Silicon and Intel packages build on their native runners separately. Signing/notarization needs legitimate owner credentials and is not part of the current candidate.
- **Ubuntu 22.04 x64 reference:** build-essential, pkg-config, libwebkit2gtk-4.1-dev, libgtk-3-dev, libayatana-appindicator3-dev, librsvg2-dev, libssl-dev, patchelf and libfuse2. Distribution package names can differ. See native-build.yml for the exact reference build recipe.

Native compilation does not imply feature parity. macOS/Linux currently lack session hooks, making native Today/Focus unavailable. Media and several OS-opening helpers also return explicit unsupported errors.

## First verification

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings
node --experimental-strip-types scripts/check-contracts.mjs
```

`pnpm dev` starts a loopback browser preview at port 1420. It has no production database and shows explicit preview/empty states. It is useful for layout, not native persistence acceptance.

## Isolated native development

Debug mode alone does not isolate AppData. Prefer an isolated OS user. Alternatively create a local Tauri override file outside the published source:

```json
{ "identifier": "com.example.lumen.dev" }
```

Then run `pnpm tauri dev --config /path/to/qa.local.json`. Verify the resolved AppData path belongs to the development identifier before creating fixtures. Do not open the only copy of real records in an experimental fork. Production uses the configured identifier `com.lumen.desktop` and Tauri's `app_data_dir()`.

## Test configuration

| Setting | Default | Scope |
| --- | --- | --- |
| LUMEN_E2E_BROWSER | msedge | Playwright channel, use chromium on CI |
| LUMEN_E2E_REPORT | docs/testing/e2e-results.json | Browser JSON evidence |
| CARGO_TARGET_DIR | Cargo default | Build cache only, not app data |
| CARGO_BUILD_JOBS | Cargo default | Optional memory/CPU control |
| LUMEN_TOOLCHAIN_ROOT | D:\\LumenToolchain | Optional owner's PowerShell wrapper only |
| lumen.locale.v1 | zh-CN | Renderer localStorage UI language |
| lumen.appearance.v1 | system / sage | Renderer appearance preference |

For CI-like browser testing, set LUMEN_E2E_BROWSER=chromium, run `pnpm exec playwright install chromium`, then `pnpm test:e2e`. Test projects cover 1280×720, 1440×900 and 1920×1080. Keep screenshots synthetic and isolated.

The optional Windows owner script `scripts/environment.ps1` changes process-local toolchain paths. It is not a universal prerequisite and must not hardcode another user's disk layout. The reference workstation's FFFF0000 affinity mask is hardware-specific, never a general build requirement.

## App configuration and storage

- `src-tauri/tauri.conf.json`: identifier, base window, local frontend and CSP. Do not broaden connect-src to implement an unrelated feature.
- `tauri.release.conf.json`: Windows release packaging. `tauri.experimental.conf.json`: macOS/Linux experimental packaging.
- `src-tauri/capabilities/default.json`: minimal `core:default`. Native dialogs/shortcuts are invoked from Rust, without arbitrary renderer filesystem permissions.
- `src/kernel/modules/catalogue.json`: trusted module inventory. Module preferences persist separately with revisions.
- `src/styles/tokens.css`: built-in appearance tokens. Remote fonts/CSS URLs are outside the future theme contract.
- AppData `lumen.db`: core facts. Existing DB remains at the root for compatibility. `core/` is reserved, not a silent relocation.
- AppData `plugins/<id>`, `attachments`, `backups`, `exports`, `extensions`, `themes`, `logs`, `cache`: separate responsibilities. Community execution is closed despite directory preparation.

There is no supported `.env` AI token configuration and no hidden cloud URL to fill in. Do not add credentials to manifests, source files, screenshots or issue logs.

## Packaging

```sh
# Windows
pnpm tauri build --config src-tauri/tauri.release.conf.json --bundles nsis -- --locked
# macOS on native host
pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked
# Linux on native host
pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles deb,appimage -- --locked
```

Record producing commit, toolchain, package architecture and SHA256. Separate build success from install, real-window, OS integration, upgrade/uninstall and signing acceptance. Keep original releases unchanged.

## Failure diagnosis

| Error / symptom | Resolution |
| --- | --- |
| MSVC/linker/WebView absent | Install the official workload/runtime and reopen the terminal |
| Port 1420 busy | Stop your previous preview, then retry. Do not kill unrelated services |
| DATABASE_BUSY | Close the other initialization/write window; keep DB/WAL intact |
| STATE_CONFLICT / stale basis | Reload and explicitly reapply changes or generate another proposal |
| NATIVE_HOOK_UNAVAILABLE | Missing OS adapter, not corrupt data. Use the accepted platform |
| MEDIA_UNSUPPORTED | Use manual music entry |
| PLUGIN_RUNTIME_DEFERRED | Expected boundary. Do not bypass the Kernel |
| Backup or attachment verification failed | Preserve originals, inspect space/permissions and rehearse a verified restore |
| OS blocks unsigned app | Use legitimate OS approval/signing paths. Never disable protections |

Report version, OS/architecture, error code, reproducible steps and synthetic examples. Omit private bodies, full data directories and credentials.

# Lumen

Local-first personal growth workspace · 本地优先个人成长工作台

MIT · Everything is a Module · Privacy by Architecture · Manual First / AI Optional

Windows x64 0.1.0 is a locally verified unsigned release candidate. macOS (Apple Silicon / Intel) and Linux x64 packages are experimental; successful compilation alone does not establish feature parity. Native session events, media integration and attachment opening still have platform limitations. No actual AI invocation, arbitrary community native plugins or plugin marketplace is enabled.

## Start here / 从这里开始

- [中文技术手册](docs/zh-CN/TECHNICAL_HANDBOOK.md) · [English handbook](docs/en/TECHNICAL_HANDBOOK.md)
- [中文文档索引](docs/zh-CN/README.md) · [English index](docs/en/README.md)
- [安装与支持范围](docs/zh-CN/INSTALLATION.md) · [Installation](docs/en/INSTALLATION.md)
- [架构](docs/zh-CN/ARCHITECTURE.md) · [Module system](docs/en/MODULE_SYSTEM.md)
- [安全回补记录](src-tauri/vendor/glib/LUMEN_PATCH.md)
- [MIT license](LICENSE) · [Security](SECURITY.md)

## Build

Install Node 24.15.0, pnpm 11.19.0, Rust 1.98.1 and the native prerequisites for Tauri 2. Use the checked-in lockfiles.

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm typecheck
pnpm test
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml --locked
pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked
```

The last command is for macOS. Linux: replace `app,dmg` with `deb,appimage`. Windows: use `pnpm tauri build --config src-tauri/tauri.release.conf.json --bundles nsis -- --locked`. Read [complete prerequisites and build conditions](docs/en/BUILD_FROM_SOURCE.md). CI uses native GitHub-hosted runners and is started manually in Actions → Native experimental packages. Artifacts expire after30days; keep a checksum-verified local copy. [Build/verification results](docs/distribution/BUILD_RESULTS.md) · [Release candidate](https://github.com/wangziwen2794928256/Lumen/releases/tag/v0.1.0-distribution-rc1).

Production data lives under Tauri AppData and is never included here. This repository begins with an allowlisted source snapshot; `UPSTREAM_CHECKPOINT.json` records local upstream provenance. Historical local screenshots, machine logs and private acceptance records are intentionally excluded. This preserves the owner's original phase history without uploading personal records.

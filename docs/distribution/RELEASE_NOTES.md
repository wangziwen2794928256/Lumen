# Lumen 0.1.0 — Distribution candidate / 发行候选

Windows x64: locally accepted Phase16 unsigned candidate. The executable and installer are unchanged from checkpoint bcee0483ae5c072e49307dada2bee479a830ef99. They are not newly compiled by the macOS/Linux workflow.

macOS ARM64/Intel and Linux x64: experimental native build targets. Actual CI results and producing commit are recorded in BUILD.json and the Phase17 distribution report. Runtime parity, real-machine install/upgrade/uninstall, OS event delivery, signing/notarization are not established by compilation.

All four native packages passed build and static image/package/architecture/hash validation. macOS build commit94e89b4d8deaeb5f7f1e09808acbd7ee3005ccaf; Linux build commit06c2c4aebdc7a301ed776cdb2705b32507006c70. The release tag identifies the final publication snapshot, which also includes later documentation/CI helper changes. BUILD_MANIFEST.json preserves each binary's actual provenance. [Build and verification evidence](https://github.com/wangziwen2794928256/Lumen/blob/main/docs/distribution/BUILD_RESULTS.md) lists the native runs and hashes.

Included archives: Lumen-Source-<publication>.zip, Lumen-Technical-Documentation.zip and Lumen-Native-CI-Evidence.zip. The source archive excludes .git/private phase history. Windows35 frontend/72 Rust/60 Edge E2E PASS; native macOS71 Rust per architecture; Linux71 Rust plus debug/release glib and60 Chromium E2E PASS. One ten-year benchmark is ignored in default suites and was separately run in Phase16. Original failed CI runs and their repairs are retained in the report.

Important limitation: macOS/Linux session_events=false; current native Today/focus initialization returns NATIVE_HOOK_UNAVAILABLE. Media capture and attachment/directory/preview/deep-link platform integrations have explicit unsupported paths. Browser tests use isolated fixtures. Do not use an experimental package with the only production database.

Linux's GTK glib0.18.5 source includes the minimal upstream two-line fix for RUSTSEC-2024-0429, with provenance and optimized iterator regression. Version-based audit can still report the advisory; no blanket suppression is configured.

Includes bilingual technical handbook/installation docs and a 12-slide editable Chinese product/architecture deck. No private SQLite, journals, music histories, native personal captures or credentials are uploaded. No actual AI calls, arbitrary native community plugins, full marketplace or mobile app is added.

Verify each downloaded file against SHA256SUMS. Windows is unsigned; macOS candidates have no notarization. Follow supported OS approval processes; do not disable system protections. Full original phase history and machine-specific acceptance evidence stay on the owner workstation; the GitHub source snapshot records upstream provenance.

当前仓库默认私有。Windows 候选已经本机验收，macOS/Linux 属于实验范围；各平台的构建、原生运行和系统集成分别验收。数据目录由 AppData API 决定，升级前创建一致性备份。具体支持缺口见中英文 INSTALLATION 与 TECHNICAL_HANDBOOK。

# Distribution build results / 构建结果

Candidate0.1.0 · owner-authorized private staging · 2026-10-11.

This document separates source/build, static package acceptance and real native integration. Final package hashes and producing commits are recorded in BUILD_MANIFEST.json and SHA256SUMS. The final publication commit may include documentation/packaging changes after a native build; it must not be confused with the binary's producing commit.

| Target | Build | Static package acceptance | Native integration |
| --- | --- | --- | --- |
| Windows x64 | Retained Phase16 installer + portable | Original SHA256 unchanged | Phase16 local installation/window/upgrade/uninstall-data-retention PASS; unsigned |
| macOS ARM64 | DMG PASS | DMG checksum/mount/Bundle/Mach-O PASS | NOT RUN; experimental |
| macOS Intel | DMG PASS | DMG checksum/mount/Bundle/Mach-O PASS | NOT RUN; experimental |
| Linux x64 | deb + AppImage PASS | Metadata/extraction/AppRun/ELF64/SHA256 PASS | NOT RUN; experimental |

Windows source checkpoint: bcee0483ae5c072e49307dada2bee479a830ef99. macOS source:94e89b4d8deaeb5f7f1e09808acbd7ee3005ccaf; [native run](https://github.com/wangziwen2794928256/Lumen/actions/runs/38080545648). Both macOS jobs succeeded; the overall run failed in Linux AppImage packaging. [macOS package verification](https://github.com/wangziwen2794928256/Lumen/actions/runs/38082923199) passed both architectures.

Linux repair source:06c2c4aebdc7a301ed776cdb2705b32507006c70; [Linux run](https://github.com/wangziwen2794928256/Lumen/actions/runs/38082828409). The earlier run compiled the application and passed tests but failed AppImage packaging because no square PNG was declared. The experimental config now declares the existing PNG/icns/ico resources. Target selection permits a Linux-only retry; optimized security tests and the native build now share the same explicit Cargo target.

Verified local regression:8 Windows gates,35 frontend/72 Rust library tests,60 Edge browser tests at3sizes, JSON contract drift and120-file glib provenance. Native macOS library tests:71 PASS plus1 ignored benchmark per architecture. The Linux repair run independently passed71 library tests plus the glib debug/release regression and60 Chromium browser tests (5.7min). [Linux static verification](https://github.com/wangziwen2794928256/Lumen/actions/runs/38084743119) passed both packages. The ten-year benchmark was run separately in Phase16, not on every Phase17 runner.

| Verified native file | Bytes | SHA256 |
| --- | ---: | --- |
| Lumen_0.1.0_aarch64.dmg | 8147855 | 265172dd4b84e6912c5b3d1454aaa2a34d65feb1c51b2312b9dfa78756c19f59 |
| Lumen_0.1.0_x64.dmg | 8479626 | 2bd337b004e852d4b618bf2a771189b456ab49fed3a4fd21090ac99b0e0e1c53 |
| Lumen_0.1.0_amd64.deb | 9780024 | 274ae1e6040e0133d1bc964cb1f01e9380e332a29bc471b811fa1fc06a31a6fc |
| Lumen_0.1.0_amd64.AppImage | 87894520 | a3fb9028dc62585908a44222dd8d61579214d7bd472f7df561f3b6c0cf2a4b50 |

Linux's scoped glib0.18.5 vendor backport changes only the upstream mutable C-output pointer lines. The optimized iterator test guards the actual dependency path. Version-based audits may still report RUSTSEC-2024-0429; there is no blanket suppression. See [patch record](../../src-tauri/vendor/glib/LUMEN_PATCH.md).

The Chinese PPT has12 editable slides and4 native tables;2 screenshots use synthetic browser fixtures. Final package/layout/font/native-table/first-party-import checks passed with zero findings; all12 final renders were inspected. Microsoft PowerPoint native open/edit was NOT RUN. SHA256:47a3f41b02c611ccdfa65434249b72ab6458e6fc2f65faf1c0e9d7b7765909e8.

macOS/Linux session_events=false; native Today/focus initialization returns NATIVE_HOOK_UNAVAILABLE. Media capture, attachment/directory opening, preview browser and protocol registration have explicit unsupported boundaries. Real OS lock/sleep safety, notifications/tray/autostart, native install/upgrade/uninstall and macOS notarization remain unverified. No arbitrary community native execution, full WASM runtime, actual AI, mobile or website deployment is added. Experimental builds must use isolated test data.

当前仓库保持私有；构建、包结构、真实运行和系统集成分别验收。失败 run 保留，不用成功的单个 job 掩盖整体失败。源码白名单不包含生产数据库、日记、音乐历史、私人原生截图、凭证和原始本地 Git 历史。Windows 原候选/桌面快捷方式不变；无新增 migration。

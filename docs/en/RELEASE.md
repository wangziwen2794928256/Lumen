# RELEASE

Phase16 prepares a Windows0.1.0 release candidate after verified Phase0–15 checkpoints. Actual acceptance is recorded in docs/reports/PHASE_16_REPORT.md and LUMEN_V1_FINAL_REPORT.md. A pending/failed check is not a completed release.

Future lifecycle: offline manifest/API/hash/signature-policy validation stages a package; install leaves it disabled; enable needs grants/runtime preflight; disable revokes/stops; update backs up isolated data and requires new permissions; rollback needs compatible snapshots and keeps the old package on failure; uninstall removes disabled code while retaining data by default. The pure state machine is testable; installer/runtime are absent.

scripts/release/build-windows.ps1 generates the locked optimized NSIS current-user installer (jobs1, incremental0, codegen1, optimization2 after documented compiler crashes). Missing WebView2 uses Microsoft's bootstrapper; no security-policy bypass. Signing requires an owner-controlled certificate; unsigned candidates are explicitly labeled. Lifecycle tests use a unique QA product/identifier/data/install path and never uninstall production. Path isolation is not clean Windows VM acceptance. Default uninstall retains AppData.

After verified commit, scripts/release/package.ps1 produces installer/portable/screenshots/database-schema/test-reports/docs/source/checksums, exact Git source archive and verified SHA256 manifest. Explicit inclusion excludes production data. Portable binaries still use Tauri AppData for user data. Public publishing, auto-update, marketplace, full runtime, actual AI and mobile are excluded. Do not alternate incompatible binaries on the same core database.

Matching Chinese document：[RELEASE](../zh-CN/RELEASE.md)。

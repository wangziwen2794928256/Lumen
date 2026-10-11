# RELEASE

Current edition: public source, detailed bilingual docs, editable 24-slide decks and English frontend. See [release notes](../distribution/PHASE18_RELEASE_NOTES.md). Earlier Phase17 statements below describe its historical private release.

Phase17 adds a private distribution prerelease to the accepted Phase16 Windows0.1.0 candidate: unchanged installer/portable binary, experimental macOS ARM64/Intel DMGs and Linux x64 deb/AppImage, bilingual docs, editable Chinese PPT, source archive, CI evidence and SHA256 manifest. BUILD_MANIFEST.json records native producing commits separately from the final publication commit. See [build results](../distribution/BUILD_RESULTS.md). Original Windows reports remain local. Pending/failed checks are never labelled PASS.

Future lifecycle: offline manifest/API/hash/signature-policy validation stages a package; install leaves it disabled; enable needs grants/runtime preflight; disable revokes/stops; update backs up isolated data and requires new permissions; rollback needs compatible snapshots and keeps the old package on failure; uninstall removes disabled code while retaining data by default. The pure state machine is testable; installer/runtime are absent.

scripts/release/build-windows.ps1 generates the locked optimized NSIS current-user installer (jobs1, incremental0, codegen1, optimization2 after documented compiler crashes). Missing WebView2 uses Microsoft's bootstrapper; no security-policy bypass. Signing requires an owner-controlled certificate; unsigned candidates are explicitly labeled. Lifecycle tests use a unique QA product/identifier/data/install path and never uninstall production. Path isolation is not clean Windows VM acceptance. Default uninstall retains AppData.

The original scripts/release/package.ps1 retains the Phase16 local package. Phase17 scripts/distribution/assemble-delivery.py verifies downloaded CI packages/receipts, stable Windows/PPT hashes and source blobs, then produces source/docs/evidence ZIPs and BUILD_MANIFEST.json/SHA256SUMS. Upload authorized private prerelease assets, compare GitHub digests, verify tag commit, and retain an external local copy. No production SQLite/journals/music histories/private native captures/credentials/local Git history are uploaded. Portable binaries still use AppData. Native workflows only read repository contents; release upload is separate. Website deployment, visibility changes, auto-update, marketplace, full runtime, actual AI and mobile are excluded.

Matching Chinese document：[RELEASE](../zh-CN/RELEASE.md)。


## Phase17 distribution update

Native macOS ARM64/Intel and Linux x64 workflows now live in `.github/workflows/native-build.yml`. Build results must come from the actual run; compilation does not establish OS integration support. See [current handbook](TECHNICAL_HANDBOOK.md) and [installation matrix](INSTALLATION.md) for complete commands, platform gaps, the glib backport, private source publication and retained local evidence. Historical Phase16 reports/build scripts are local acceptance references; they are not all included in the allowlisted GitHub source snapshot.

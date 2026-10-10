# REPRODUCE_LUMEN

The retained local Phase16 checkpoint bcee0483ae5c072e49307dada2bee479a830ef99 produces the Windows candidate. GitHub has a separate source-only history; its publication commit and UPSTREAM_CHECKPOINT.json identify the exported source. Do not fetch local-only checkpoint tags from that repository. Verify source/package SHA256 using BUILD_MANIFEST.json and SHA256SUMS, then use BUILD_FROM_SOURCE.md and TESTING.md. Original phase16 scripts/reports stay on the owner workstation; browser fixtures are distinct from native integration.

Large performance fixtures are generated in TempDir, never production AppData. The ten-year benchmark was independently run in Phase16, not automatically run on each Phase17 target. No training or model download occurs. UTC/local-date conventions and host timezone are documented in the technical handbook.

Delivery includes full commit/tag, SHA256 manifest, optimized installer/portable binaries and evidence. Reproducible means traceable source/dependencies/commands/fixtures, not a current promise of bit-identical binaries across compilers, SDKs and linker timestamps.

Matching Chinese document：[REPRODUCE_LUMEN](../zh-CN/REPRODUCE_LUMEN.md)。

Native CI records tools, actual producing commit, tests and BUILD.json. verify-packages.yml accepts separate macOS/Linux build run IDs and enforces complete image/package/architecture coverage. The owner assembler scripts/distribution/assemble-delivery.py consumes downloaded verified receipts and the publication checkout to make source/documents/CI-evidence archives and manifest/checksums; ordinary source builds do not require it. Reference release uses opt2/codegen1. CI artifacts expire after30days; release/local copies retain deliverables. SHA256 verifies delivered identity, not cross-machine bit-identical reproduction.


## Phase17 distribution update

Native macOS ARM64/Intel and Linux x64 workflows now live in `.github/workflows/native-build.yml`. Build results must come from the actual run; compilation does not establish OS integration support. See [current handbook](TECHNICAL_HANDBOOK.md) and [installation matrix](INSTALLATION.md) for complete commands, platform gaps, the glib backport, private source publication and retained local evidence. Historical Phase16 reports/build scripts are local acceptance references; they are not all included in the allowlisted GitHub source snapshot.

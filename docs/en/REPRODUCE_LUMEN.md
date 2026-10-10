# REPRODUCE_LUMEN

Use checkpoint/15-release for the final candidate source/lockfile snapshot. Source archives exclude personal data. scripts/verify/phase16.ps1 emits phase16-gates.json, named logs and three-viewport E2E evidence; browser fixtures are explicitly distinct from native integration.

Large performance fixtures are generated in TempDir; Phase9 reproduction and short-body limits remain in INSIGHTS.md. No training or model download occurs. UTC/local-date conventions and host timezone are documented.

Delivery includes full commit/tag, SHA256 manifest, optimized installer/portable binaries and evidence. Reproducible means traceable source/dependencies/commands/fixtures, not a current promise of bit-identical binaries across compilers, SDKs and linker timestamps.

Matching Chinese document：[REPRODUCE_LUMEN](../zh-CN/REPRODUCE_LUMEN.md)。

Phase16 evidence: final reports/checkpoints record exact commit; scripts/verify/phase16.ps1 and scripts/release/build-windows.ps1 define current commands. Reference release uses opt-level2/codegen1 after documented LLVM failures; SHA256 verifies delivered identity, not cross-machine bit-for-bit reproduction.

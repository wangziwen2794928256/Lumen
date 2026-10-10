# BUILD_FROM_SOURCE

Windows reference: load scripts/environment.ps1, run pnpm install --frozen-lockfile and scripts/verify/phase16.ps1. package.json records Node/pnpm requirements; Cargo.lock fixes Rust dependencies. See environment decisions for prerequisites.

For development, use a dedicated CARGO_TARGET_DIR, CARGO_BUILD_JOBS=1 and pnpm tauri build --debug --no-bundle. Windows needs WebView2 and MSVC SDK. QA requires an isolated app identifier/profile and must never write production storage.

Other-platform preparation is documented separately, not verified by a Windows build. Source is MIT; dependencies retain their licenses. Exclude private databases, credentials and generated dependencies from archives.

Matching Chinese document：[BUILD_FROM_SOURCE](../zh-CN/BUILD_FROM_SOURCE.md)。

Phase16 packaging: Rust minimum1.90; project-pinned compiler1.98.1. `scripts/release/build-windows.ps1` produces an optimized current-user installer with the verified opt2/codegen1 profile and locked dependencies. Base config remains unbundled for development. Missing WebView2 uses Microsoft bootstrapper; no security policy bypass. Signing needs an owner-supplied certificate; unsigned candidates are labeled.

On the reference i9-14900HX host, use scripts/release/build-windows.ps1 -AffinityMask FFFF0000 and scripts/verify/phase16.ps1 -AffinityMask FFFF0000. This constrains only the worker to its observed efficiency cores after a WHEA warning; other machines must inspect CPU topology or omit that host-specific mask. It does not modify BIOS/global security policy. See PHASE_16_TOOLCHAIN_DECISION.md for full evidence.

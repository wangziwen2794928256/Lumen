# SOURCE_MAP

src/kernel/modules contains the shared catalogue, validators, provider, ordering, route gate and widget extension. src/modules/official/insights owns the migrated entry and Today widget. Existing features/pages retain stable implementations.

Rust kernel owns modules, permissions, storage, plan and platform boundaries. Rust modules coordinates official startup; database owns SQLx/migrations; commands owns controlled IPC. Existing domains remain trusted bundled services.

docs/contracts stores WIT/JSON Schema; zh-CN and en contain matching technical documents; execution, testing/phase95 and reports store tasks, verification and phase evidence.

Matching Chinese document：[SOURCE_MAP](../zh-CN/SOURCE_MAP.md)。

Phase12: Official Profile at src/modules/official/profile and src/pages/profile; Kernel public_export/deep_link/preview; commands/profile; standalone website/ and scripts/preview-website.mjs. Details: PUBLIC_WEBSITE.md.

Phase13: database/backup.rs and backup_schedule.rs own complete snapshots/retention; kernel/data_exchange.rs and privacy.rs own private exchange/recovery/privacy; commands/privacy.rs exposes controlled IPC; features/privacy supplies Settings panels. See BACKUP_PRIVACY.md.

Phase14: kernel/desktop.rs, commands/desktop.rs, features/desktop and settings groups; official tray/single-instance/window-state/autostart/notification adapters. See DESKTOP.md.

Phase15–16: skills/*/SKILL.md; scripts/verify/phase15-skills.ps1, phase16.ps1, phase16-installer.ps1; scripts/release/build-windows.ps1 and package.ps1; rust-toolchain.toml; database/backup_schedule.rs resolves redirected AppData backup roots.

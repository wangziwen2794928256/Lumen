# SOURCE_MAP

src/kernel/modules：共享 catalogue、校验、Provider、排序、路由Gate和Widget边界。src/modules/official/insights：模块入口及Today widget贡献。src/features、src/pages：稳定业务代码保留。

src-tauri/src/kernel：modules/permissions/storage/plan/platform；src-tauri/src/modules：官方模块启动协调。database拥有SQLx与编号迁移，commands拥有受控IPC，insights/journal/coding/today仍是可信内置domain services。

docs/contracts：WIT及JSON Schema；docs/zh-CN 与 docs/en：对应17份技术文档；docs/execution：原子任务；docs/testing/phase95：门禁/原生证据；docs/reports：阶段报告。

本文件对应英文版本：[SOURCE_MAP](../en/SOURCE_MAP.md)。

Phase12：Official Profile 位于 src/modules/official/profile、src/pages/profile；Kernel public_export/deep_link/preview；commands/profile；独立 website/ 与 scripts/preview-website.mjs。边界见 PUBLIC_WEBSITE.md。

Phase13: database/backup.rs and backup_schedule.rs own complete snapshots/retention; kernel/data_exchange.rs and privacy.rs own private exchange/recovery/privacy; commands/privacy.rs exposes controlled IPC; features/privacy supplies Settings panels. See BACKUP_PRIVACY.md.

Phase14：kernel/desktop.rs、commands/desktop.rs、features/desktop 与设置分组；官方托盘、单实例、窗口状态、自启动和通知适配。见 DESKTOP.md。

Phase15–16: skills/*/SKILL.md; scripts/verify/phase15-skills.ps1, phase16.ps1, phase16-installer.ps1; scripts/release/build-windows.ps1 and package.ps1; rust-toolchain.toml; database/backup_schedule.rs resolves redirected AppData backup roots.

Phase17：.github/workflows 管理原生构建/静态包验收；scripts/distribution 管理源码白名单/blob 核验、vendor provenance、构建 receipt 和交付汇总；src-tauri/vendor/glib 为最小上游回补。docs/distribution 保存发行策略/结果；双语技术手册是当前维护入口。上文历史 execution/testing/reports 和 verify 脚本保留本地，不全部上传；GitHub 是白名单源码快照，不是私人阶段完整历史。

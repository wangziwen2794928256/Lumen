# Lumen — 双语文档与英化版本 / Bilingual documentation and English UI

## 已审核的 PPT v4 修订 / Approved PPT v4 revision

2026-10-11：用户已审核通过的中英文 r6 原样发行为 v4，各24页。新增概念封面，优化场景、工作流、模块接入、开源衍生创作与双语排版。同步更新源码快照、技术文档 ZIP、BUILD_MANIFEST.json 和 SHA256SUMS；v3 重建输入保留在技术文档 ZIP 的 docs/presentations 内。此修订只更新资料，不重新构建 Windows/英化包；原始 tag 仍指向原程序发布提交，当前资料提交见 BUILD_MANIFEST.json 的 publicationCommit。未重跑原生 PowerPoint 打开编辑验收。

The accepted r6 decks are released byte-for-byte as v4, 24 slides per language. The source snapshot, documentation ZIP and checksum manifests are updated. Original v3 authoring inputs remain inside the documentation ZIP. Application and language-pack binaries are unchanged. The original release tag keeps its application provenance; the documentation revision commit is recorded as publicationCommit in BUILD_MANIFEST.json. Native PowerPoint open/edit validation remains unrun.

## 中文

本地优先、开源的个人成长工作台：考研、考公、语言、编程、科研、职业转型都可以用计划、真实时间、日记和作品记录。Everything is a Module：从源码接入自己需要的组件。

本版增加 905 条静态英文资源及动态单位、图表标签、导出文案。设置 → 外观 → 界面语言选择 English，保存编辑后应用并重新载入。用户记录保持原文和本地存储。系统对话框跟随 OS；部分 Rust 托盘、通知、底层诊断仍为中文。

中英文资料包括 12 组功能原理与使用流程、60 个 IPC 签名、12 组模型、已编译验证的模块教程、权限隔离、环境配置和 14 类扩展方向。PPT 各 24 页，文字、代码、7 张原生表格可编辑，内容 JSON 和作者脚本公开。

Windows EXE/NSIS 为本版新构建的未签名候选。本次不包含重编译的 macOS/Linux 包；此前实验包保留在前一发行页，不包含本次英化。macOS/Linux Today/Focus 仍不可用。社区 WASM Runtime、完整市场、AI 调用和移动端未实现。

MIT 源码与自制资料允许修改、分发和创作衍生作品，保留许可和署名。欢迎 Fork、贡献模块，觉得好用请点 Star 支持创作者。发布仅含白名单源码和合成演示资料，不含生产库、日记、私人日志、备份、凭据或完整本机历史。下载后核对 SHA256SUMS。

## English

A local-first, open-source workspace for sustained growth: exams, languages, coding, research, projects and career transitions. Everything is a Module: build the source components your workflow needs.

This edition adds 905 static English resources plus dynamic units, accessible chart labels and export copy. Select English under Settings → Appearance → Interface language, save edits and apply/reload. User records remain unchanged and local. OS dialogs follow the OS; some Rust tray, notification and diagnostic copy remains Chinese.

Detailed bilingual documentation covers 12 feature groups, 60 IPC signatures, 12 model groups, a compiled module tutorial, permissions, environment/configuration and 14 contribution directions. Two editable 24-slide decks contain seven native tables each; content JSON and authoring scripts are public.

Windows EXE/NSIS are freshly built unsigned candidates. Rebuilt macOS/Linux packages are not included in this edition. Previous experimental packages remain in the earlier release without this localization; Today/Focus remain unavailable there. Community WASM execution, a full marketplace, AI calls and mobile are deferred.

MIT source and project-created editable materials welcome adaptations with license and attribution retained. Fork, contribute and give a Star if Lumen helps you. Publication excludes production databases, journals, private logs, backups, credentials and full workstation history. Check SHA256SUMS after download.

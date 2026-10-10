# ARCHITECTURE

四层：Kernel → Official Modules；Community Modules 经未来 capability host 与 Kernel 交互；Theme Packs 只提供校验后的设计 token。Kernel 拥有安全、数据库、迁移/备份、注册表、平台、权限、计划交换；不允许卸载安全服务。

此前是 React 硬编码导航/路由直接调用 Tauri domain commands，平台分支位于 Today/Journal。现在增加受控注册表与平台边界，稳定 domain services 原位保留。洞察是首个已迁移 Official Module，其余为 legacy-bridge 或 planned。Everything is a Module 指职责可组合，不意味着所有 Kernel 安全服务可卸载。

原则：Local-first、Open Source、Privacy by Architecture、Manual First / AI Optional、Cross-platform、Reproducible by Design。无账号和云端前置；MIT 许可，尚未公开发布。

本文件对应英文版本：[ARCHITECTURE](../en/ARCHITECTURE.md)。

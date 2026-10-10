# MODULE_SYSTEM

Manifest源为src/kernel/modules/catalogue.json：id、双语name、SemVer version、api_version=1.0、layer、route、icon、required、status、permissions。id/route唯一；trusted官方目录编译进程序，不接受用户文件冒充official或kernel。

Registry持久化enabled/hidden/pinned/order；启用控制官方入口，hidden仅侧栏，pinned优先且各组内稳定order。设置拖动/上下按钮显式保存；侧栏拖动直接保存。失败显示错误，revision冲突不覆盖。Today required不能关闭，设置与安全始终可达。关闭模块不删业务数据。

洞察的route、widget、所有IPC与启动封存任务都检查启用状态。旧模块Gate提供兼容UI禁用，但旧核心专注安全监听等仍运行，这是渐进边界，不能称作全部模块已完全隔离。Widget slot=today.after-summary，错误边界防止组件崩溃破坏整个首页。

本文件对应英文版本：[MODULE_SYSTEM](../en/MODULE_SYSTEM.md)。

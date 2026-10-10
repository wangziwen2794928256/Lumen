# PRIVACY

默认private、离线可用、没有遥测与插件自动网络。Kernel不将lumen.db/路径/文件句柄交给Community Modules。官方内置模块在可信进程中共享数据，属于当前信任边界，不声称互相沙箱隔离。

插件自己的AppData/plugins/<id>/data.db或KV由未来host管理，禁止读取core。卸载默认保留隔离数据，另行明确删除请求才清理；缓存/日志不存正文。备份覆盖core与受控附件，未来插件备份策略需逐插件授权及version标记。

本机SQLite/备份当前不加密；OS账号、磁盘权限和磁盘加密属于宿主保障。默认私密导出不等于脱敏，词语/标签仍敏感；JSON/PNG仅用户显式导出，本阶段不新增AI分享。

本文件对应英文版本：[PRIVACY](../en/PRIVACY.md)。

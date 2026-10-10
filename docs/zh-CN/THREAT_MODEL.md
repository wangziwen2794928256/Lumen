# THREAT_MODEL

信任主体：用户、Kernel、编译内置官方代码；不信任社区包、其manifest声明、Theme Pack输入、LLM输出和远程响应。资产：core数据库、附件、秘密、规划选择、权限状态与可用性。

攻击/缓解：路径穿越或symlink逃逸→无guest文件路径，canonical namespace拒绝越界（未来文件代理还需抗TOCTOU句柄）；伪造official→编译期catalogue；旧grant重用→hash/version绑定与撤销；恶意widget→声明式数据及host渲染；无限循环/内存→未来worker限额；prompt injection→提案视为数据、禁止执行/自动接受。

升级/回滚需包hash、权限重同意、隔离数据库备份、兼容检测；不可逆数据升级时拒绝二进制回滚。宿主OS被攻破、官方代码恶意或用户已授权公开导出不在Wasm沙箱可解决范围。本阶段runtime关闭，不将设计策略冒称已有沙箱。

本文件对应英文版本：[THREAT_MODEL](../en/THREAT_MODEL.md)。

# CROSS_PLATFORM

Platform Service集中于kernel/platform：Adapter声明name、session_events、attachment_viewer，host()集中OS选择。业务仅调用install_system_events/open_verified_attachment，不能散落cfg或OS字符串分支。

Windows保持WTS锁屏/电源消息、窗口subclass释放、ShellExecute经校验附件；原生安全测试移至platform/windows，不删除。macOS边界准备NSWorkspace/session通知与系统查看器；Linux准备logind/portal；当前明确false并返回Unsupported，专注不冒称已具备锁屏保障。

下一步需macOS/Linux本机build/锁屏/睡眠/快捷键/附件/目录权限测试，不从跨平台库推断已通过。桌面外壳和SQLite设计可移植，移动端完全不在本阶段范围。

本文件对应英文版本：[CROSS_PLATFORM](../en/CROSS_PLATFORM.md)。

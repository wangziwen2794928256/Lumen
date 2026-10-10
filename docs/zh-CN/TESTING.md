# TESTING

执行scripts/verify/phase16.ps1：lint、format、typecheck、unit、build、rust-fmt、rust-test（含migration）、clippy、完整E2E。三尺寸浅深色、导航拖动/键盘、启停/隐藏/置顶/重载与表单间距是UI门禁。Phase16证据记录前端35项、Rust72项、E2E60项；默认忽略的十年性能测试单独执行。

Kernel负面测试：required模块不可停用、未知/重复ID拒绝、陈旧revision、关闭模块IPC、unsupported API、任意SQL/文件/网络/native权限、runtime始终关闭、非法路径、非法提案与容量越界、append-only观察记录。迁移回滚与Windows消息释放测试延续。

真实原生操作与正式升级另记录NATIVE_QA；生产按升级前原表原列SHA256比较，不保存正文。macOS/Linux构建、完整Wasm隔离、AI调用本阶段NOT RUN/延期，不可标PASS。

本文件对应英文版本：[TESTING](../en/TESTING.md)。

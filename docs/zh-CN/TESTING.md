# TESTING

按 BUILD_FROM_SOURCE 显式执行 lint/format/typecheck/unit/build/Rust格式/含迁移测试/契约检查。Windows 另运行 `cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings`。Chromium E2E：`pnpm exec playwright install --with-deps chromium`，设置 LUMEN_E2E_BROWSER=chromium 后 pnpm test:e2e，Windows 不设置默认 Edge。三尺寸浅深色、导航拖动/键盘、启停/隐藏/置顶/重载与表单间距是 UI 门禁。Phase17 Windows35前端/72Rust/60Edge E2E PASS；macOS71 library pass、1 ignored，Linux 另有 glib debug/release integration 回归。计数以日志为准，原 verify 脚本留在本地主机。

Kernel负面测试：required模块不可停用、未知/重复ID拒绝、陈旧revision、关闭模块IPC、unsupported API、任意SQL/文件/网络/native权限、runtime始终关闭、非法路径、非法提案与容量越界、append-only观察记录。迁移回滚与Windows消息释放测试延续。

verify-packages.yml 在 macOS 只读挂载 DMG、校验 Bundle/Mach-O，在 Linux 解包 deb/AppImage、校验 metadata/ELF，并要求完整包与 SHA256。解包不代表启动或安装。macOS/Linux 延期适配器仍有 dead-code 警告，不套用 Windows Clippy PASS。原生窗口、锁屏/睡眠、托盘/通知/自启、附件、安装/升级/卸载另验；生产比较只保存 hash 不保存正文。完整 WASM、真实 AI、macOS/Linux 原生启动/安装/签名/公证为 NOT RUN；其 session events、Today/focus 当前不支持。

本文件对应英文版本：[TESTING](../en/TESTING.md)。


## Phase17 发行更新

已新增 `.github/workflows/native-build.yml`，覆盖 macOS ARM64/Intel 与 Linux x64 原生目标。构建结果以实际 run 为准，编译不能代替系统集成验收。最新完整命令、平台缺口、glib 安全回补、私有源码发行与本地证据策略见 [技术手册](TECHNICAL_HANDBOOK.md) 和 [安装矩阵](INSTALLATION.md)。Phase16 历史报告/脚本属于本地验收参考，不全部进入 GitHub 白名单源码快照。

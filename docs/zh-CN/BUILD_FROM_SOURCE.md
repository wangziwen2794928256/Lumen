# BUILD_FROM_SOURCE

Windows参考：scripts/environment.ps1；pnpm install --frozen-lockfile；scripts/verify/phase16.ps1。package.json声明Node与pnpm版本，Cargo.lock固定Rust依赖；环境决策见docs/environment。

开发构建设置独立CARGO_TARGET_DIR、CARGO_BUILD_JOBS=1；pnpm tauri build --debug --no-bundle。WebView2与MSVC SDK为Windows前置。QA必须独立identifier/profile，不能写正式库。

macOS/Linux工具链准备与适配测试见CROSS_PLATFORM；本阶段没有这些系统的构建证据。源码MIT，第三方按自身许可证；不把数据库、密钥、node_modules和target打包进源码。

本文件对应英文版本：[BUILD_FROM_SOURCE](../en/BUILD_FROM_SOURCE.md)。

Phase16打包：Rust最低1.90，项目固定编译器1.98.1。`scripts/release/build-windows.ps1`按已验证opt2/codegen1和锁定依赖生成优化版当前用户安装包；基础配置保留开发不打包。缺少WebView2时使用微软bootstrapper；不绕过安全策略。签名需用户证书，未签名候选明确标记。

参考i9-14900HX设备使用scripts/release/build-windows.ps1 -AffinityMask FFFF0000及scripts/verify/phase16.ps1 -AffinityMask FFFF0000，仅将构建worker限制到已确认能效核心，应对已记录WHEA警告。其他机器应检查拓扑或省略此设备专用mask，不修改BIOS或全局安全策略。完整证据见PHASE_16_TOOLCHAIN_DECISION.md。


## Phase17 发行更新

已新增 `.github/workflows/native-build.yml`，覆盖 macOS ARM64/Intel 与 Linux x64 原生目标。构建结果以实际 run 为准，编译不能代替系统集成验收。最新完整命令、平台缺口、glib 安全回补、私有源码发行与本地证据策略见 [技术手册](TECHNICAL_HANDBOOK.md) 和 [安装矩阵](INSTALLATION.md)。Phase16 历史报告/脚本属于本地验收参考，不全部进入 GitHub 白名单源码快照。

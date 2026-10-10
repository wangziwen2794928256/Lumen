# BUILD_FROM_SOURCE

使用 Node24.15.0、packageManager 固定的 pnpm11.19.0、rust-toolchain.toml 固定的 Rust1.98.1 和两个 lockfile。安装 [Tauri 系统前置依赖](https://v2.tauri.app/start/prerequisites/)。源码构建不依赖 Codex；在标准 Node/pnpm/Rust 环境执行：

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --locked
node --experimental-strip-types scripts/check-contracts.mjs
```

开发构建设置独立CARGO_TARGET_DIR、CARGO_BUILD_JOBS=1；pnpm tauri build --debug --no-bundle。WebView2与MSVC SDK为Windows前置。QA必须独立identifier/profile，不能写正式库。

macOS14+：`pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked`。Ubuntu22.04 x64 将 bundles 换为 deb,appimage，安装 WebKitGTK4.1、GTK3、AppIndicator、OpenSSL、FUSE2、patchelf、librsvg 与开发库。Windows：`pnpm tauri build --config src-tauri/tauri.release.conf.json --bundles nsis -- --locked`。CI 记录 opt2/codegen1、jobs2、incremental0；Windows 参考 jobs1。debug 不隔离 AppData，启动开发包须使用独立 OS 用户或 QA identifier/profile。源码 MIT，第三方保留许可证。

本文件对应英文版本：[BUILD_FROM_SOURCE](../en/BUILD_FROM_SOURCE.md)。

Phase16打包：Rust最低1.90，项目固定编译器1.98.1。`scripts/release/build-windows.ps1`按已验证opt2/codegen1和锁定依赖生成优化版当前用户安装包；基础配置保留开发不打包。缺少WebView2时使用微软bootstrapper；不绕过安全策略。签名需用户证书，未签名候选明确标记。

可选所有者包装脚本 scripts/release/build-windows.ps1 默认工具链目录 D:\LumenToolchain，可通过 LUMEN_TOOLCHAIN_ROOT 覆盖，-TargetDirectory 也需按本机指定。以上通用命令不依赖该目录。FFFF0000 仅适用于已核验拓扑且出现 WHEA 的参考 i9-14900HX，只限制 worker，其他设备省略。本地 Phase16 报告、工具链决策与 verify 脚本不全部进入白名单源码快照。


## Phase17 发行更新

已新增 `.github/workflows/native-build.yml`，覆盖 macOS ARM64/Intel 与 Linux x64 原生目标。构建结果以实际 run 为准，编译不能代替系统集成验收。最新完整命令、平台缺口、glib 安全回补、私有源码发行与本地证据策略见 [技术手册](TECHNICAL_HANDBOOK.md) 和 [安装矩阵](INSTALLATION.md)。Phase16 历史报告/脚本属于本地验收参考，不全部进入 GitHub 白名单源码快照。

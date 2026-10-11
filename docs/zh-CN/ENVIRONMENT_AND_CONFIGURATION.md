# 环境安装与程序配置

普通开发工作站即可构建；不需要 Codex、AI 账户、API Key 或云后端。应用默认手工、本地运行。只使用安装包请先读 [安装说明](INSTALLATION.md)。

## 固定版本与用途

| 组件 | 项目固定版本 | 检查与用途 |
| --- | --- | --- |
| Node.js | 24.15.0，engine >=24.15.0 <25 | `node --version`，前端与构建 |
| pnpm | 11.19.0 | `pnpm --version`，锁定依赖 |
| Rust | rust-toolchain.toml 中 1.98.1 | `rustc --version`，Tauri/Rust |
| React / TypeScript / Tauri | package.json 和 lockfiles | 不替换成未固定 latest |
| 原生 SDK | 随 OS 变化 | 编译器、系统库、WebView |
| Python | 发行校验脚本使用 | 日常使用应用不需要 |

从官方安装 Node、Rust/rustup。没有 pnpm 时执行 `npm install --global pnpm@11.19.0`。Rustup 在项目内读取固定工具链。依赖使用 `pnpm install --frozen-lockfile`，Cargo 带 `--locked`。提交修改时保留两份锁文件。

## 原生依赖

具体步骤以 [Tauri 官方前置条件](https://v2.tauri.app/start/prerequisites/) 为准。

- **Windows x64：** Microsoft C++ Build Tools 的“使用 C++ 的桌面开发”工作负载、MSVC、Windows SDK 和 WebView2 Runtime。仅安装 IDE 而没有 C++ 工作负载不够。Rust 使用 MSVC target。
- **macOS：** Xcode Command Line Tools 和对应架构 Rust。当前实验配置最低 macOS 14，Apple Silicon 与 Intel 分别在原生 runner 构建。签名/公证需要合法所有者凭据，现有候选未签名公证。
- **Ubuntu 22.04 x64 参考环境：** build-essential、pkg-config、libwebkit2gtk-4.1-dev、libgtk-3-dev、libayatana-appindicator3-dev、librsvg2-dev、libssl-dev、patchelf、libfuse2。其他发行版包名可能不同，准确参考配方见 native-build.yml。

能编译不代表功能齐全。macOS/Linux 缺少 session hooks，原生 Today/Focus 当前不可用；媒体监听及部分打开助手也返回显式不支持。

## 首次门禁

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --lib --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings
node --experimental-strip-types scripts/check-contracts.mjs
```

`pnpm dev` 只在 1420 端口提供 loopback 浏览器预览，没有生产数据库。它适合布局检查，不代表原生存储验收。

## 隔离原生开发

debug 不会自动隔离 AppData。优先用独立 OS 用户；也可在发布源码之外创建 Tauri 本地覆盖文件：

```json
{ "identifier": "com.example.lumen.dev" }
```

执行 `pnpm tauri dev --config /path/to/qa.local.json`。创建 fixture 前确认 AppData 已解析到开发 identifier。不要用唯一真实数据副本试验 fork。正式标识为 `com.lumen.desktop`，根目录由 Tauri `app_data_dir()` 选择。

## 配置表

| 名称 | 默认值 | 影响 |
| --- | --- | --- |
| LUMEN_E2E_BROWSER | msedge | Playwright 浏览器，CI 用 chromium |
| LUMEN_E2E_REPORT | docs/testing/e2e-results.json | E2E JSON 证据位置 |
| CARGO_TARGET_DIR | Cargo 默认 | 编译缓存，不影响应用数据 |
| CARGO_BUILD_JOBS | Cargo 默认 | 可限制编译内存/CPU |
| LUMEN_TOOLCHAIN_ROOT | D:\\LumenToolchain | 仅所有者 PowerShell 包装器 |
| lumen.locale.v1 | zh-CN | localStorage 界面语言 |
| lumen.appearance.v1 | system / sage | localStorage 外观偏好 |

模拟 CI 时设 LUMEN_E2E_BROWSER=chromium，执行 `pnpm exec playwright install chromium` 后 `pnpm test:e2e`。默认三个窗口尺寸为 1280×720、1440×900、1920×1080。截图只能用隔离合成数据。

`scripts/environment.ps1` 是所有者可选包装器，仅设置进程环境；其他开发者无需建立相同 D 盘目录。参考机器 FFFF0000 编译亲和掩码是硬件专用处理，不能照搬成通用构建条件。

## 程序配置与数据

- `src-tauri/tauri.conf.json`：identifier、窗口、本地 frontend、CSP。不要为了无关功能扩大 connect-src。
- `tauri.release.conf.json`：Windows 打包；`tauri.experimental.conf.json`：macOS/Linux 实验打包。
- `src-tauri/capabilities/default.json`：最小 core:default。原生选择器/快捷键从 Rust 调用，不给 renderer 任意文件系统权限。
- `src/kernel/modules/catalogue.json`：可信模块清单；用户模块偏好单独通过 revision 保存。
- `src/styles/tokens.css`：内置主题；未来主题契约拒绝远程字体和 CSS URL。
- AppData `lumen.db`：核心事实，保持原根路径兼容；`core/` 预留但不静默搬库。
- `plugins/<id>`、attachments、backups、exports、extensions、themes、logs、cache 分别隔离职责。目录就绪不等于社区代码可执行。

没有需要填写的 `.env` AI Token 或隐藏云端地址。不要把凭据放入 manifest、源码、截图或问题日志。

## 打包

```sh
# Windows
pnpm tauri build --config src-tauri/tauri.release.conf.json --bundles nsis -- --locked
# macOS 原生主机
pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked
# Linux 原生主机
pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles deb,appimage -- --locked
```

记录产出 commit、工具链、架构与 SHA256。编译、安装、真实窗口、OS 集成、升级/卸载、签名各自验收，保留旧发行不覆盖。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| MSVC/linker/WebView 缺失 | 安装官方工作负载/运行时并重开终端 |
| 1420 被占用 | 停止自己的旧预览，不终止无关服务 |
| DATABASE_BUSY | 关闭其他初始化/写入窗口，保留 DB/WAL |
| STATE_CONFLICT / 依据过时 | 重新载入、明确重做选择或再生成建议 |
| NATIVE_HOOK_UNAVAILABLE | OS adapter 缺失，不是数据库损坏 |
| MEDIA_UNSUPPORTED | 使用手动音乐录入 |
| PLUGIN_RUNTIME_DEFERRED | 预期安全边界，不要绕过 Kernel |
| 备份/附件校验失败 | 保留原件，检查空间/权限并隔离恢复演练 |
| OS 阻止未签名应用 | 合法签名或受支持批准路径，不关闭防护 |

报告问题时提供版本、OS/架构、错误码、可复现步骤和合成示例，去除私人正文、完整目录和凭据。

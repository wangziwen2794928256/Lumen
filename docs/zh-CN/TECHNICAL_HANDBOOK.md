# Lumen 技术手册

## 详细阅读路线

本手册给出系统全景。逐模块的工作流、技术原理与边界见 [MODULE_REFERENCE](MODULE_REFERENCE.md)；可执行源码接入步骤见 [MODULE_TUTORIAL](MODULE_TUTORIAL.md)；60 个 IPC 命令签名与 12 组领域模型见 [API_REFERENCE](API_REFERENCE.md)；环境、配置、隔离开发和打包见 [ENVIRONMENT_AND_CONFIGURATION](ENVIRONMENT_AND_CONFIGURATION.md)。用户起步见 [USER_GUIDE](USER_GUIDE.md)，可贡献方向见 [EXTENSION_IDEAS](EXTENSION_IDEAS.md)。当前 English UI 包见 [LOCALIZATION](LOCALIZATION.md)。


版本：0.1.0 候选；双语文档与英化版，PPT v4 资料修订；2026-10-11。

## 1. 产品范围与支持级别

Lumen 是本地个人成长工作台，使用任务、路线规划、专注、日历、日记、搜索、洞察、刷题、音乐、作品库与个人档案连接计划和实际活动。此前 Windows x64 候选完成本地安装、升级、卸载保留数据与原生窗口验收；当前英化包完成构建与浏览器回归，未重复原生安装验收。本次 PPT 修订不重新构建程序。macOS ARM64 / Intel、Linux x64 属于实验构建：CI 成功只证明指定环境中的测试和打包，不代表真实设备上的全部集成可用。

默认手动操作。AI Planner 只有数据交换契约，没有实际模型调用、训练或后台数据上传。社区插件市场、完整 WASM Runtime、移动端均未实现。

## 2. 运行结构

```text
React 页面 → TypeScript IPC 契约 → Tauri Commands
                                  ↓
                         Rust Kernel 与领域 Service
                                  ↓
                     参数化 Repository → SQLite / 受控文件
```

前端不直接读取 SQLite。Rust 管理数据库、备份、导出、权限、平台接口和生命周期。UI 路由按模块加载；Module Registry 负责 enabled / hidden / pinned / order，侧栏读取注册表而不是重复维护模块列表。首页 Widget Extension Point 接受受控贡献。颜色在设置统一管理。

四层边界：

| 层 | 职责 | 约束 |
| --- | --- | --- |
| Kernel | 安全、数据、IPC、备份、模块/权限与平台边界 | 安全系统不可卸载 |
| Official Modules | 官方功能与第一个迁移验证模块 | 通过 Kernel 服务访问数据；保留已有稳定实现 |
| Community Modules | Manifest、权限、版本与未来运行时接口 | 本版不执行任意第三方 native code |
| Theme Packs | 主题 token 与资源扩展 | 不授予数据库、文件系统或网络权限 |

## 3. 源码入口

| 位置 | 阅读目的 |
| --- | --- |
| `src/app` / `src/pages` | 应用壳、页面和路由 |
| `src/kernel/modules` | Manifest 验证、Registry、侧栏状态 |
| `src/kernel/plan` | Plan Context / Exchange 验证 |
| `src/lib` | IPC 客户端与共享工具 |
| `src-tauri/src/lib.rs` | Tauri 插件、Service 初始化与命令注册 |
| `src-tauri/src/commands` | IPC 输入/输出边界 |
| `src-tauri/src/kernel` | 权限、数据交换、存储、桌面与导出安全 |
| `src-tauri/src/kernel/platform` | Windows/macOS/Linux 唯一平台分支入口 |
| `src-tauri/src/database` | 连接、迁移、Repository、完整性检查 |
| `src-tauri/migrations` | 编号 SQL migration；已发布文件不可修改 |
| `docs/contracts` | JSON Schema 与未来 capability host WIT |
| `tests/unit` / `tests/e2e` | 前端逻辑与三尺寸浏览器回归 |
| `src-tauri/vendor/glib` | Linux GTK 安全修复的最小上游回补 |

## 4. 数据与迁移

生产根目录由 Tauri AppData API 决定，不能把工作站路径写进业务代码。当前核心数据库仍为根目录 `lumen.db`，不为整齐而移动已稳定数据。逻辑目录规划包括 `plugins/<id>`、`attachments`、`backups`、`exports`、`extensions`、`themes`、`logs`、`cache`；具体已启用路径以 Kernel storage 实现为准。规划目录不等于已经开放完整插件运行时。

Schema 版本读取 `migration_history` 最大版本；当前为 10。不要用 `PRAGMA user_version` 判断本项目版本。每次修改只能追加编号 migration，迁移前备份，失败回滚。Repository 使用参数化 SQL；用户输入不能拼接为 SQL。运行中数据库必须使用一致性备份，不可直接复制裸文件忽略 WAL。

未来 Planner 所需的估计/真实耗时、计划与实际、容量、调整历史、accept/reject 等已有字段或数据契约准备；它们用于保存人工决策证据，未训练模型。详细表和语义见 [DATABASE](DATABASE.md) 与 [领域数据说明](../data/ROADMAP_PLANNER.md)。

社区插件的数据属于隔离命名空间，不能直接读取 `lumen.db`。插件删除、更新和回滚也不得改变核心数据库所有权。

## 5. 模块与未来插件

模块提供稳定 id、版本、Module API 版本、路由/导航信息、贡献点与请求权限，先经过 Manifest 验证再进入 Registry。显隐不删除记录，禁用保留数据；固定与排序需要保存到用户配置。新增模块优先遵循现有迁移示例，不一次性重写所有页面。

插件生命周期设计覆盖 install → enable → disable、update staging、rollback、uninstall。权限默认拒绝，由 Kernel 校验 capability、插件身份、作用域与用户授权。未来 Wasmtime/WASM host 仅暴露有界数据 API，配合调用预算、结果长度、内存、时间和资源限额；不得开放任意 SQL、任意磁盘路径或任意网络 socket。

当前 WIT/Schema 是设计契约，不是已验收的沙箱。第三方代码只有在未来运行时、权限撤销、升级回滚、隔离测试和安全审查全部完成后才能开放。参见 [PLUGIN_SECURITY](PLUGIN_SECURITY.md)、[PERMISSION_MODEL](PERMISSION_MODEL.md)。

## 6. 平台边界

业务模块只调用 Kernel Platform Service。Windows 实现会话锁定/解锁、挂起/恢复监听；已有锁屏停止记录验收继续适用。macOS/Linux 的 session_events=false，当前 Today Service 初始化会报告 `NATIVE_HOOK_UNAVAILABLE`，因此依赖 Today/focus 的原生命令不可视为已支持。不能用 UI 浏览器 mock 的通过掩盖这个限制。

macOS/Linux 媒体监听返回 `MEDIA_UNSUPPORTED`；可使用手动记录。附件/目录打开、本地预览浏览器、deep link 注册尚无平台实现，返回明确不支持错误。Tauri 官方托盘、通知、自动启动插件的跨平台 API 存在，但实际 OS 投递/注册和权限未验收。平台适配必须另加真实设备测试，不能通过启用计时而失去锁屏/休眠保护。

## 7. 构建与回归

固定 Node 24.15.0、pnpm 11.19.0、Rust 1.98.1；两个 lockfile 都必须提交。目标系统需要 Tauri 2 原生前置，参见 [官方 prerequisites](https://v2.tauri.app/start/prerequisites/)。

```sh
pnpm install --frozen-lockfile
pnpm lint
pnpm format:check
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings
node --experimental-strip-types scripts/check-contracts.mjs
```

Windows 默认 E2E 使用 Edge；CI 设置 `LUMEN_E2E_BROWSER=chromium`，安装 Playwright Chromium 后执行 `pnpm test:e2e`。三尺寸 1280×720、1440×900、1920×1080，测试涉及主题、键盘、表单间距、模块切换与隐私交互。浏览器测试使用隔离 mock，不能代替原生 IPC 验收。

macOS：`pnpm tauri build --config src-tauri/tauri.experimental.conf.json --bundles app,dmg -- --locked`。
Linux：将 bundles 改为 `deb,appimage`；使用 Ubuntu 22.04 WebKitGTK 4.1、GTK3、AppIndicator、FUSE2 等构建前置。CI 三个目标在各自原生 runner 上运行，权限为 contents:read，Actions 固定到 commit SHA，仅手动触发，不配置发布密钥。

Windows 参考主机曾发生处理器 WHEA 警告，其构建 worker 使用专用能效核心 affinity；该值不适用于其他设备。此主机使用 `scripts/release/build-windows.ps1 -AffinityMask FFFF0000`，其他 Windows 主机省略 mask。

## 8. 安全依赖与发布

Linux Tauri GTK 依赖要求 glib 0.18.5，其 `VariantStrIter` 存在 [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429.html)。本项目保留版本/API，回补 [上游 PR1343](https://github.com/gtk-rs/gtk-rs-core/pull/1343) 的两行 mutable output pointer 修复，并在优化配置下测试 iterator 的各条路径。`cargo audit` 依然可能按版本报告此 advisory；不要忽略整个 advisory 类别，也不要声称该上游版本已修复。完整来源与移除条件见 vendor 的 LUMEN_PATCH.md。

正式发布应区分编译、自动测试、安装、真实窗口、OS 集成、签名/公证。当前候选未提供商业签名证书与 Apple notarization，不能建议用户关闭 Gatekeeper、SmartScreen 或其他安全机制。实验包不得覆盖唯一生产数据；优先在独立 OS 用户中验证。

GitHub 按所有者最新授权公开源码与可编辑资料。发布导出脚本采用文件白名单，不上传完整本地 Git 历史、生产库、日记、音乐历史、原生用户窗口截图、本机日志或凭据。`UPSTREAM_CHECKPOINT.json` 记录上游 checkpoint 与文件 hash；本地完整历史保留。发行页与 SHA256 对应同一构建 commit，候选先使用 prerelease/draft，不能把本机 Windows 旧包标为新 CI 编译结果。

## 9. 维护与故障排查

- 构建失败：先读对应 runner 原始日志和 toolchain.txt，核对锁文件/原生依赖，不盲目升级所有库。
- `NATIVE_HOOK_UNAVAILABLE`：当前目标的会话事件 adapter 未实现，不是用户数据库损坏。
- Schema 异常：停止写入，保留原库和 WAL，使用应用一致性备份/隔离恢复流程；禁止重置生产库。
- 安装被系统阻止：使用系统提供的受控审批路径或等待签名版本，不关闭系统保护。
- GitHub artifact 过期：从已保留的校验副本获取，或按同一源码与锁定工具链重建；重建字节不一定完全相同，需生成新 checksum。
- 反馈问题：提供版本、OS/架构、操作步骤、错误码与脱敏日志，删除私人正文、绝对私人目录和密钥。

## 10. 后续验收顺序

先完成实验包原生编译与依赖安全回归，再在真实 macOS/Linux 上实现会话事件与故障关闭策略、验证 Today/focus，随后验证媒体/附件/预览/deep link、托盘/通知/自动启动、安装/升级/卸载保留数据。签名与公证最后使用合法证书执行。完整插件运行时和 AI Planner 保持独立后续范围。

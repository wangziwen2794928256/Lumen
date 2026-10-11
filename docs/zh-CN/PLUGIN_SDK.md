# 社区插件 SDK 契约

**当前状态：只有接口设计、校验器与纯状态测试。** 没有可执行社区 SDK、市场、安装器或完整 Wasmtime host。manifest 通过校验不授予执行权。要现在做出能运行的模块，请按 [官方模块教程](MODULE_TUTORIAL.md) 修改可信源码。

## 选择接入方式

| 需求 | 当前路径 | 信任层级 |
| --- | --- | --- |
| 新页面、可信数据工作流 | fork 后接入 Official Module | 审核后的应用源码 |
| Today 面板 | 静态 widget 描述器 | 官方 renderer 同级 |
| 配色与布局 | 修改源码内置 theme tokens | 审核 CSS，无运行时包加载 |
| 分发不可信插件 | 按下方契约准备 | 执行入口明确关闭 |

## Community Manifest 1.0

```json
{
  "id": "org.example.revision",
  "version": "0.1.0",
  "api_version": "1.0",
  "runtime": "wasm-component",
  "entry": "plugin.wasm",
  "permissions": ["storage.own", "widget.contribute"]
}
```

完整定义见 [community-manifest.schema.json](../contracts/community-manifest.schema.json)。[permissions.rs](../../src-tauri/src/kernel/permissions.rs) 额外拒绝保留的 `lumen.*`。ID 最多 80 字节，小写分段以点连接；version 三段数字；API 只能 `1.0`，runtime 只能 `wasm-component`，entry 只能 `plugin.wasm`。最多三个唯一已知 capability。额外字段、native 入口、任意路径、网络/文件系统/SQL 权限均拒绝。

它不同于带双语导航和可信 route 的 bundled Manifest。不能把社区 manifest 塞入官方 catalogue 或靠自称 official 提权。

## 未来 Host API

[lumen-plugin.wit](../contracts/lumen-plugin.wit) 定义草案 ABI `lumen:plugin@1.0.0`：

| 接口 | 参数与返回 | 预期能力 |
| --- | --- | --- |
| storage.get | key → optional bytes / error | 仅自己的 opaque namespace |
| storage.put | key、bytes → result | 仅自己的空间，受限写入 |
| widgets.contribute | declarative JSON → result | Host 验证并渲染内容 |
| plans.propose | Plan Exchange JSON → result | 供审阅的建议，不写任务 |
| initialize（导出） | 无参数 → result | 初始化 |
| shutdown（导出） | 无参数 | 清理 |

接口不包含 host 路径、原始 SQL、Tauri invoke、任意 HTML/script 或默认网络句柄。当前未引入 Wasmtime/WASI 依赖。未来需拒绝 ambient WASI 文件/Socket，逐调用验证 grant，约束内存、fuel/deadline、调用数、响应大小、存储额度和并发 worker。具体资源额度尚未定稿，不能宣传已强制执行。

## 授权与撤销

声明仅为请求。Kernel `Grant` 绑定 plugin_id、version、content_hash、capabilities、revoked。`check_capability` 检查身份/版本、64 位十六进制 loaded hash、授权 hash、已声明且已授予的能力、未撤销。版本或字节变化不能复用旧 grant。即使检查通过，`authorize_community_execution` 仍返回 `PLUGIN_RUNTIME_DEFERRED`。当前没有授权 UI。

未来 opaque handle 应短期、最小范围。停用/卸载必须先终止 worker 和撤销句柄。更新增加权限需重新明确同意，不能默认继承。

## 生命周期

以下是已测试的纯状态逻辑，不是实际文件安装或升级执行器：

| 当前 | 操作 | 结果 |
| --- | --- | --- |
| Staged | install | Disabled |
| Disabled | enable | PLUGIN_RUNTIME_DEFERRED |
| Enabled | disable | Disabled |
| Disabled | update | UpdateStaged |
| UpdateStaged | commit / rollback | Disabled |
| Disabled | rollback | RollbackStaged |
| RollbackStaged | commit | Disabled |
| Disabled | uninstall | Uninstalled |
| 其他组合 | 任意 | 拒绝 |

未来 staging 先验证身份/hash/API 与包路径约束，再激活。更新保留已验证旧版本并设计插件隔离库 migration/rollback。卸载包与删除插件数据应分别选择，不触碰核心记录。不要直接改状态表模拟授权。

## 数据所有权

现有核心库保持 AppData/lumen.db。插件空间仅解析 plugins/<id>；`plugin_namespace` 校验 ID、canonicalize 既有目录并拒绝链接逃逸，不代表开放运行时或任意路径创建。extensions/<id>/<version> 是计划中的代码包区域，与数据分离。themes、attachments、backups、exports、logs、cache 各自承担独立职责。

未来插件数据库归插件，不能 ATTACH/query 核心库。确需核心事实时另行设计有界、经授权的 host projection，不能暴露内部 SQL。

## API 版本

App 版本、模块版本、API 版本各自独立。当前仅接受 `1.0`，其他版本 fail closed。未来 additive minor 协商必须显式声明 capability/schema 兼容；破坏性变更新 major，并给出弃用与兼容 adapter 窗口。同步更新 TS/Rust fixtures、JSON Schema 和 WIT，不凭 app 版本相同推断兼容。

## 规划交换

使用 [plan-exchange.schema.json](../contracts/plan-exchange.schema.json) 和 [exchange.ts](../../src/kernel/plan/exchange.ts)。方案含 schema_version、request_id、basis_hash、source、capacity_minutes、items；item 为 task_id/date/minutes。最多 500 项，每项 1–1440 分钟，task/date 唯一，总和不超容量。Rust 接受前还检查关系、当前 basis、日期、剩余工作和每日可行性。JSON Schema 本身不足以证明跨字段有效。

PlanContext 的 optional label 可能包含私人任务正文；当前仅为契约，无 External LLM/Local Model 调用、不自动外发、不自动接受。

## 未来 Runtime 必须通过的安全测试

包路径遍历/链接逃逸、伪造官方 ID、未知权限、陈旧 hash/grant、调用中撤销、资源耗尽、恶意声明式 widget、超大值、跨 namespace、错误升级和回滚失败。证明插件不能读核心库、备份、导出或凭据。独立进程与能力校验仍需实际 sandbox 验证，完整运行时验收尚未进行。

当前执行 `node --experimental-strip-types scripts/check-contracts.mjs`、模块前端测试和 Rust Kernel 测试。另见 [权限](PERMISSION_MODEL.md)、[威胁模型](THREAT_MODEL.md)、[测试](TESTING.md)。

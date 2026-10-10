# 模块扩展操作指南

这是官方模块接入及未来社区插件契约的维护指南。仅通过 Manifest 验证不构成第三方执行授权。

## 官方模块

从 `src/modules/official/insights` 阅读第一个迁移示例，再阅读 `src/modules/official/profile`。保留原 Service/Repository，新增受控模块入口。`src/kernel/modules/catalogue.json` 保存 bundled Manifest；`validateCatalogue` 只接受 `lumen.*`、kernel/official，拒绝重复 id/route。

以下示例用于说明字段，不能直接安装到运行中的应用：

```json
{
  "id": "lumen.example",
  "name": {"zh-CN": "示例", "en": "Example"},
  "version": "0.1.0",
  "api_version": "1.0",
  "layer": "official",
  "route": "/example",
  "icon": "Leaf",
  "required": false,
  "status": "module",
  "permissions": []
}
```

按照现有 router/provider 代码注册入口与 Widget 贡献；不要给业务组件复制平台判断。Registry 状态含 revision 与 items；每项为 id/enabled/hidden/pinned/order。保存时遵守 revision 冲突规则，禁止静默覆盖另一个窗口的状态。置顶组优先显示；拖动只在同一置顶分组内排序，并保留键盘上下按钮。

接入验收至少包括 Manifest 不合法/重复路由拒绝、禁用页面 gate、显隐/置顶/拖动持久化、重启保留原记录、Widget 不请求额外数据权限。新增数据仅通过新编号 migration。

## 社区 Manifest 与 capability

社区契约独立于 bundled catalogue：

```json
{
  "id": "org.example",
  "version": "0.1.0",
  "api_version": "1.0",
  "runtime": "wasm-component",
  "entry": "plugin.wasm",
  "permissions": ["storage.own", "widget.contribute"]
}
```

现有验证允许最多三项唯一权限：`storage.own`、`widget.contribute`、`plan.propose`。`lumen.*` 是保留命名空间。未知字段、路径、native runtime、任意网络/文件/SQL 权限均不被接受。Grant 绑定 plugin_id/version/content_hash/capabilities/revoked；更新内容或版本不能继承失效授权。

`authorize_community_execution` 始终返回 `PLUGIN_RUNTIME_DEFERRED`。因此示例是数据契约，不提供安装器或可运行 SDK。生命周期状态机也会拒绝当前 enable。不要用修改状态表绕过 Kernel。

## 未来 Host API

`docs/contracts/lumen-plugin.wit` 定义 storage.get/put 的 opaque key、widgets.contribute 的声明式 JSON、plans.propose 的数据提案。它不暴露 SQL、宿主路径、HTML脚本或 Tauri IPC。未来实现须增加调用频率、key/value 长度、资源/内存/时间预算和撤销后的停止保证。

Plan Exchange schema1.0 限制最多 500 项、同一 task/date 不重复、每项 1–1440 分钟、总量不超过 capacity，basis_hash 用于检测过时上下文。外部或本地提案必须人工接受，不直接写任务或自动排期。JSON Schema 之外的跨字段检查仍须执行 TS/Rust validator。

验证命令：`node --experimental-strip-types scripts/check-contracts.mjs`、前端 modules 单元/E2E 与 Rust kernel tests。完整 WASM host、安全沙箱、安装/更新实际执行器和 AI 调用不在本版范围。

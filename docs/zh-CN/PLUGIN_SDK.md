# PLUGIN_SDK

本阶段仅提供CommunityManifest数据模型、默认拒绝策略、生命周期状态机、WIT draft，不提供安装器、插件市场或可执行SDK。CommunityManifest字段见docs/contracts/community-manifest.schema.json；不接受native入口。

未来Wasm Component ABI 1.0：storage.get/put只提供opaque key/value到自己的命名空间；widgets.contribute只接受宿主校验的声明式JSON；plans.propose只提交Plan Exchange建议。禁止HTML/script/Tauri invoke、core数据库路径或主机文件句柄。

插件API与程序版本独立。当前只认1.0，未知major/minor拒绝；未来同major additive minor需显式协商能力，上版本保留旧适配器直到弃用窗口结束。变更字段要schema升级及双语言校验用例，不按应用版本猜能力。

本文件对应英文版本：[PLUGIN_SDK](../en/PLUGIN_SDK.md)。

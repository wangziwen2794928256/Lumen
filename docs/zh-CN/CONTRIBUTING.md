# 贡献指南

欢迎使用者、开发者、设计者和文档作者参与。无需特定编辑器、Codex 或 AI 服务。入口见 [仓库贡献说明](../../CONTRIBUTING.md)，对应英文版见 [Contributing](../en/CONTRIBUTING.md)。

## 选择切入口

可以报告问题、补充使用示例、修正文案、翻译、改进可访问性，也可以贡献模块、主题设计和公开可编辑 PPT。考研、考公、语言、科研、编程和职业成长等场景均欢迎。可拓展方向见 [EXTENSION_IDEAS](EXTENSION_IDEAS.md)。

源码模块接入请先阅读 [MODULE_TUTORIAL](MODULE_TUTORIAL.md)、[MODULE_SYSTEM](MODULE_SYSTEM.md) 和 [API_REFERENCE](API_REFERENCE.md)。当前官方模块采用可信源码集成；社区 WASM Runtime、插件市场和运行时主题包加载并未实现。设计契约不等于已开放执行能力。

## 开发与验证

先查重 Issues/PR；较大功能先讨论问题、用户流程和数据边界。Fork 后在独立分支小步修改稳定代码，保留锁文件。环境固定版本、原生依赖、隔离 AppData 和完整命令见 [ENVIRONMENT_AND_CONFIGURATION](ENVIRONMENT_AND_CONFIGURATION.md)。浏览器预览不能替代原生功能验收。

| 改动 | 适用检查 |
| --- | --- |
| 文档、翻译、演示资料 | 检查链接、术语、中英一致性；演示逐页检查，不把概念图当真实产品证据 |
| 前端逻辑与界面 | lint、format:check、typecheck、相关 unit、build；受影响的 E2E、键盘与布局检查 |
| Kernel / Rust | fmt、相关 unit、Clippy；受影响的 IPC、权限和原生集成验证 |
| 数据库与备份 | 新编号 migration、迁移失败恢复、备份一致性和已有记录保留测试，仅使用隔离 fixture |
| 模块 API 或权限 | 同步 Schema/类型/契约、兼容性测试与权限拒绝用例 |

界面改动检查 1280×720、1440×900、1920×1080，兼顾明暗主题、键盘焦点、ARIA、对比度和 reduced motion。默认语言保持 zh-CN；新用户可见文案应支持英文。macOS/Linux 有明确实验边界，不得把 mock 检查写成原生平台支持。

## PR 应说明什么

写清问题和复现步骤、修改后的行为、相关 Issue、兼容性/数据影响及实际执行的检查。文档改动可以说明业务检查 N/A；未运行写 NOT RUN 和原因，不能写 PASS。截图使用合成数据。维护者会审查范围、实现和证据，并决定合并；不要求贡献者取得主仓库写权限或使用特定 AI 工具。

## 隐私、安全和许可

不要上传生产 SQLite、私人正文、附件、备份、密钥或私人日志。debug 不会自动隔离 AppData，必须验证开发数据目录。已发布 migration 不可修改，不可删除或重置真实用户数据。参数化 SQL、受控文件路径、权限撤销和 Kernel 授权边界必须保留。不得直接授予社区代码读取 lumen.db、任意磁盘或网络的能力。

你须有权提交内容，并保留 MIT 及第三方许可/版权声明；可以 Fork、修改和分发自己的衍生作品。向本项目提交的代码或自制资料应与其 MIT 许可兼容，第三方素材说明来源和许可。使用 AI 辅助不免除对正确性、许可和隐私的责任。发行签名、包发布和维护者凭据不属于普通 PR 所需权限。

发现安全问题时不要在公开 Issue 附带秘密、真实数据或可直接利用的敏感细节，参见 [Security](../../SECURITY.md)。

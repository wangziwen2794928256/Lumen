# 参与贡献 / Contributing to Lumen

欢迎所有开发者和使用者参与 Lumen。你可以提交代码，也可以改进文档、翻译、使用体验和演示资料。**不需要使用 Codex 或任何特定 AI 工具。**

Everyone is welcome to contribute code, documentation, translations, usability feedback and editable presentation materials. **No Codex account or particular AI tool is required.**

## 中文

Lumen 是 MIT 开源、本地优先的个人成长工作台。Everything is a Module：欢迎 Fork 成自己的作品，编写需要的组件，也欢迎把通用改进贡献回来。修改和分发时保留许可证、版权及第三方声明。

产品坚持简约易用，额外需求优先通过 Issues 讨论或独立模块探索，不默认增加核心复杂度。你可以自行开发，也可以借助 AI 等工具；请使用合成数据并审阅、验证生成结果。[移动端 App](docs/zh-CN/EXTENSION_IDEAS.md#重要展望移动端-app) 是重要展望，欢迎需求讨论与原型贡献，当前尚未实现。

### 可以从哪里开始

- **功能模块：** 错题复盘、考研/考公专题训练、阅读进度、语言练习、实验日志或作品迭代。参考 [扩展方向](docs/zh-CN/EXTENSION_IDEAS.md)。
- **源码接入：** 按 [模块教程](docs/zh-CN/MODULE_TUTORIAL.md) 编写页面、Manifest、路由和 Today widget。当前接入方式是可信源码集成；社区 WASM 执行与插件市场尚未开放。
- **界面与可访问性：** 修复布局、键盘操作、焦点、对比度和动效问题。主题扩展先参考 [Theme SDK](docs/zh-CN/THEME_SDK.md)，运行时主题包加载仍待实现。
- **文档与英化：** 补充示例、修正文案、完善中英文资料，或编辑公开 PPT。技术入口见 [文档中心](docs/zh-CN/README.md)。
- **问题反馈：** 提供版本、操作系统、复现步骤、预期与实际行为；截图使用合成数据并移除私人信息。

### 提交一个改进

1. 先查看现有 Issues 和 Pull Requests。较大的功能或架构调整先开 Issue 讨论；小范围修复和文档改进可以直接提交 PR。
2. Fork 仓库，在自己的分支开发。环境与隔离开发说明见 [环境配置](docs/zh-CN/ENVIRONMENT_AND_CONFIGURATION.md)。
3. 保持改动聚焦，复用现有模块与 Kernel 服务。代码变更同步相关测试、契约和中英文说明。
4. 运行与改动相关的检查，在 PR 中说明问题、修改后的行为、兼容性/数据影响和实际测试结果。未运行的检查写明原因。
5. 提交 PR，由维护者审查并决定是否合并。贡献者不需要主仓库写权限。

### 数据与安全边界

不要提交生产 SQLite、任务或日记正文、附件、密钥、私人日志或真实用户截图。原生开发使用独立 AppData，禁止用唯一真实数据副本测试。已发布 migration 不可修改；schema 变化应新增编号迁移并验证备份、失败恢复和数据保留。安全和权限属于 Kernel，不能依靠前端隐藏按钮实施隔离，也不能通过贡献绕过社区运行时尚未开放的限制。

更详细的验证要求见 [中文贡献指南](docs/zh-CN/CONTRIBUTING.md)。安全相关说明见 [Security](SECURITY.md)。感谢你的贡献；如果 Lumen 对你有帮助，欢迎点一个 Star。

## English

Lumen is an MIT-licensed, local-first personal growth workspace. Everything is a Module: fork it into your own work, build the components you need and contribute reusable improvements. Preserve license, copyright and third-party notices when adapting or distributing it.

Keep the product simple and approachable. Discuss additional needs through Issues or explore a focused module without adding default core complexity. Build it yourself, optionally using AI tools, with synthetic data and reviewed, validated results. A [mobile app](docs/en/EXTENSION_IDEAS.md#major-outlook-a-mobile-app) is a major future direction; needs and prototypes are welcome, but it is not implemented.

### Ways to contribute

- **Modules:** exam review, topic practice, reading progress, language exercises, experiment logs or portfolio iterations. See [extension ideas](docs/en/EXTENSION_IDEAS.md).
- **Source integration:** follow the [module tutorial](docs/en/MODULE_TUTORIAL.md) for pages, manifests, routes and Today widgets. Current integration is trusted source code; community WASM execution and a marketplace are not available yet.
- **UI and accessibility:** improve layout, keyboard navigation, focus, contrast and motion. Consult the [Theme SDK](docs/en/THEME_SDK.md); runtime theme-pack loading remains deferred.
- **Documentation and localization:** improve examples, Chinese/English copy and editable presentations. Start at the [documentation hub](docs/en/README.md).
- **Bug reports:** include the version, OS, reproduction steps, expected and actual behavior. Use synthetic screenshots and remove private information.

### Contribution workflow

1. Check existing Issues and Pull Requests. Discuss substantial features or architecture changes in an Issue first; focused fixes and documentation improvements can go straight to a PR.
2. Fork the repository and work on your own branch. Follow the [environment and isolation guide](docs/en/ENVIRONMENT_AND_CONFIGURATION.md).
3. Keep changes focused and reuse existing modules and Kernel services. Update relevant tests, contracts and bilingual documentation.
4. Run applicable checks. Explain the problem, resulting behavior, compatibility/data impact and actual validation in the PR. Explain checks that were not run.
5. Open a PR for maintainer review and a merge decision. Contributors need neither write access to the main repository.

### Privacy and security

Never submit production SQLite files, task/journal bodies, attachments, secrets, private logs or real user screenshots. Isolate AppData for native development. Released migrations are immutable; schema changes require new numbered migrations and backup, failure-recovery and preservation checks. Kernel authority must enforce security. Contributions must not bypass the closed community runtime.

See the [detailed English contribution guide](docs/en/CONTRIBUTING.md) and [Security](SECURITY.md). Thank you for contributing. A Star is welcome if Lumen helps you.

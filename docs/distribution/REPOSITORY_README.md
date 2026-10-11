# Lumen

**为一段专心成长的时间，建立属于自己的工作台。**  
**A personal workspace for a season of focused growth.**

[可编辑演示 / Editable decks](docs/presentations/README.md) · [中文技术文档](docs/zh-CN/README.md) · [English documentation](docs/en/README.md) · [模块开发教程 / Module tutorial](docs/zh-CN/MODULE_TUTORIAL.md) · [发行候选 / Releases](https://github.com/wangziwen2794928256/Lumen/releases)

## 中文

### 不限于一场考试，也不限于一张待办清单

Lumen 将长期目标、每日任务、实际专注、日历、日记、编程练习、音乐回忆与作品库放在同一台设备上。它面向一段认真提升自己的过程：考研、考公、语言学习、资格考试、科研写作、转行准备，或从零完成一个自己的项目。你可以先手工记录，再逐步建立适合自己的方法。

考研时，把数学、英语和专业课拆成目标与任务，比较计划和实际；考公时，分别记录行测模块、申论练习和错题复盘；科研或作品冲刺时，用日记记录推导，用 Vault 保存成果、证书和项目链接。模板是起点，字段与路线都可以按自己的情况修改。程序不预测录取概率，不用连续打卡惩罚休息。

### 简约使用，按需扩展

Lumen 主打简约、易上手，不以堆叠功能取胜。默认体验保持清晰，需要时再启用相应模块；额外需求可以通过 Issues 反馈，也可以参考教程 Fork 源码，自行编写组件，或借助 AI 等开发工具实现。AI 可选，日常使用无需 AI 账户；开发时请使用合成数据，并审阅、验证生成代码。

### Everything is a Module：缺少的组件，可以自己写

Lumen 采用 Kernel、Official Modules、Community Modules、Theme Packs 四层结构。核心负责安全、本地数据、迁移、备份和平台服务；功能通过模块组织。侧栏来自 Module Registry，可启用、隐藏、置顶和拖动排序。Today 提供 widget 贡献点。Insights、Music、Vault、Profile、每日运势已有独立官方模块入口，其他稳定模块通过兼容桥逐步迁移。

想要错题面板、阅读进度、论文实验日志、考公专题训练、作品迭代记录，或者自己的配色？你可以从 MIT 源码 fork，在官方模块教程的基础上编写入口、声明 manifest、接入路由和贡献点，复用现有服务与组件。文档提供真实代码路径、参数模型、环境要求和验证步骤，无需从头推导整个应用。

**当前能力说明：** 可直接修改源码接入可信官方模块。社区插件的 manifest、权限、生命周期与 WASM host API 已有契约，但本版不执行第三方插件、不提供完整插件市场。完整 WASM Runtime、AI Planner 和移动端属于后续方向。

### Local-first，隐私从架构开始

日常任务、日记、专注、音乐与附件保存在本机 AppData；核心数据库是 SQLite。手工使用不需要在线账户，当前没有后台模型调用或训练，也不自动上传任务内容。音乐监听默认关闭。安全属于不能卸载的 Kernel，未来社区插件不能直接读取 `lumen.db`、任意文件系统或网络。

公开作品是明确选择：先在 Vault 标为公开，在 Profile 检查白名单投影，再手工导出静态网站。私有备份和公开网站相互独立。你仍需保护自己的 OS 账户、磁盘和导出文件；本地存储并不意味着设备被攻破或主动分享时仍有绝对安全保证。

### 开源，也欢迎改成你的作品

项目源码采用 [MIT License](LICENSE)。欢迎学习、修改、分发、制作自己的衍生作品，并诚实说明自己的改动，保留原许可和作者声明。中文/英文 Markdown 技术文档与 PowerPoint 发布演示均提供可编辑材料；演示源数据和生成方法也有说明。第三方依赖保留各自许可。

源码与可编辑资料按所有者授权公开发布。欢迎阅读、Fork、改造和贡献；如果 Lumen 对你有帮助，请点一个 Star 支持创作者。

### 接下来，你可以把它变成什么？

**移动端 App 是重要的未来展望。** 希望探索轻量 iOS / Android 体验，先从离线 Today 概览与快速记录入手，保持本地优先和简约界面。移动安装包与设备同步尚未实现，也没有承诺发布日期。详见 [移动端展望](docs/zh-CN/EXTENSION_IDEAS.md#重要展望移动端-app)。

| 方向 | 可以先做的模块 | 接入基础 | 需要额外设计的边界 |
| --- | --- | --- | --- |
| 移动端 App | 离线 Today 概览、快速记录原型 | 领域模型、数据交换契约 | 尚未开发；平台行为、冲突与隐私需独立验证 |
| 考研 / 考公 | 错题复习队列、专题计时、阶段模板 | Tasks、Journal、Roadmap | 题目来源许可，不能自动推测分数 |
| 语言与阅读 | 词汇复习、书籍进度、口语练习日志 | Journal、Today widget | 音频附件类型与容量策略 |
| 科研与创作 | 实验日志、论文节点、作品版本记录 | Calendar、Vault | 大文件和实验数据导入校验 |
| 职业成长 | 面试题单、技能树、证书作品集 | Coding、Roadmap、Profile | 简历公开字段必须主动选择 |
| 无障碍与个性化 | 高对比主题、键盘流程、更多语言 | CSS tokens、语言资源 | 双主题对比度、焦点与布局回归 |
| 本地智能辅助 | 人工审阅的规划建议、可解释复盘 | Plan Exchange 1.0 | 当前无 AI；未来需最小上下文与授权 |
| 互操作 | ICS/Markdown 导入、手工迁移助手 | 私有导出与模型 | 预览、去重、时区和回滚 |

这些是贡献方向，不是已经发布的功能清单。详细切入口见 [扩展路线](docs/zh-CN/EXTENSION_IDEAS.md)。

### 从这里开始

- **使用者：** [安装与平台支持](docs/zh-CN/INSTALLATION.md)、[使用指南](docs/zh-CN/USER_GUIDE.md)。Windows 是已验证的主平台；macOS/Linux 是实验包，原生 Today/Focus 尚不可用。
- **开发者：** [环境与配置](docs/zh-CN/ENVIRONMENT_AND_CONFIGURATION.md)、[逐模块原理](docs/zh-CN/MODULE_REFERENCE.md)、[可照做的模块教程](docs/zh-CN/MODULE_TUTORIAL.md)、[API 参考](docs/zh-CN/API_REFERENCE.md)。
- **插件作者：** [Plugin SDK](docs/zh-CN/PLUGIN_SDK.md)、[权限与安全](docs/zh-CN/PERMISSION_MODEL.md)。先读“当前可用”和“未来契约”的区别。
- **语言：** 设置 → 外观 → 界面语言 → English，保存当前编辑后应用。自己的记录保持原文。

如果 Lumen 帮到了你，欢迎在 [GitHub](https://github.com/wangziwen2794928256/Lumen) 点一个 **Star**，也欢迎提交问题、改进文档和贡献模块。

---

## English

### For a focused period of growth

Lumen brings long-term goals, daily tasks, actual focus, calendar, journals, coding practice, music memories and a personal vault into one local desktop workspace. Use it for postgraduate or civil-service exams, language learning, certifications, research, career changes or your next independent project. Begin with manual records and shape the workflow around your own goals.

Break subjects into journeys and tasks, review estimates against actual focus, keep reasoning in journals and collect real outcomes in Vault. Templates are editable starting points. Lumen does not predict admission probability or punish rest with streak pressure.

### Simple to use, extensible when needed

Lumen prioritizes an approachable, simple experience over accumulating features. Keep the default workflow clear and enable modules when needed. Request additional capabilities through Issues, or fork the source and build them yourself with optional AI-assisted development. Everyday use needs no AI account. Develop with synthetic data and review and validate generated code.

### Everything is a Module

Four layers organize the system: Kernel, Official Modules, Community Modules and Theme Packs. Kernel owns security, local data, migrations, backup and platform services. Module Registry generates navigation with enable/hide/pin/reorder preferences. Today exposes a widget extension point. Insights, Music, Vault, Profile and Daily fortune have dedicated official entrypoints, while stable legacy features migrate incrementally.

Missing a mistake-review queue, reading tracker, experiment notebook, exam-practice panel or project log? Fork the MIT source and build your component using the official-module tutorial. It explains manifests, routes, contributions, typed services and verification with real source paths. You can reuse the foundation instead of reverse-engineering the application.

**Available now:** trusted official modules integrated through source changes. **Prepared interfaces:** community manifests, permissions, lifecycle and WASM host contracts. Third-party execution, a full marketplace, a complete WASM runtime, AI Planner and mobile apps are not implemented.

### Local-first. Privacy by architecture

Tasks, journals, focus, music and attachments stay in local AppData, with SQLite at the core. Manual use needs no online account. The current app has no background model calls, training or automatic task uploads. Media observation defaults to off. Security remains a non-removable Kernel responsibility. Future community plugins must not access `lumen.db`, arbitrary files or networks directly.

Sharing is deliberate: mark works public in Vault, inspect the allowlisted projection in Profile, then export a static website manually. Private backups and public exports are separate. Protect your OS account, disk and exported files. Local storage does not guarantee safety after device compromise or voluntary disclosure.

### Source you can make your own

The application source uses the [MIT License](LICENSE). Learn from it, modify it, redistribute it and build your own derivative work. Preserve the license and attribution and describe your changes honestly. Chinese/English Markdown documentation and PowerPoint presentations are editable, with presentation content and reproduction instructions. Dependencies retain their own licenses.

Source and editable materials are published with the owner’s authorization. Read, fork, adapt and contribute. If Lumen helps you, please give the project a Star.

### Extension opportunities

**A mobile app is an important future direction.** A lightweight iOS / Android experience could provide an offline Today overview and quick capture while preserving local-first storage and a simple interface. Mobile apps and device sync are not implemented; there is no promised release date. See [the mobile outlook](docs/en/EXTENSION_IDEAS.md#major-outlook-a-mobile-app).

Build exam revision queues, reading progress, experiment logs, interview plans, skill trees, certificates, accessible themes or additional languages. Future interoperability could add reviewed ICS/Markdown imports. Future local planning could use Plan Exchange 1.0 for proposals that users explicitly accept. These are contribution opportunities, not shipped features. Each needs its own validation, privacy and compatibility decisions. See [extension ideas](docs/en/EXTENSION_IDEAS.md).

### Start here

- **Users:** [installation](docs/en/INSTALLATION.md) and [user guide](docs/en/USER_GUIDE.md). Windows is the primary accepted platform. macOS/Linux packages are experimental, with native Today/Focus unavailable.
- **Developers:** [environment and configuration](docs/en/ENVIRONMENT_AND_CONFIGURATION.md), [module reference](docs/en/MODULE_REFERENCE.md), [hands-on tutorial](docs/en/MODULE_TUTORIAL.md) and [API reference](docs/en/API_REFERENCE.md).
- **Plugin authors:** [Plugin SDK](docs/en/PLUGIN_SDK.md) and [permission model](docs/en/PERMISSION_MODEL.md), including the current execution boundary.
- **English UI:** Settings → Appearance → Interface language → English. Save your work before applying. Your records retain their original language.

If Lumen helps your work, a **Star** on [GitHub](https://github.com/wangziwen2794928256/Lumen) supports the creator. Issues, documentation improvements and module contributions are welcome.

[当前需求覆盖与差距 / Coverage and remaining gaps](docs/zh-CN/REQUIREMENTS_AUDIT.md) · [English audit](docs/en/REQUIREMENTS_AUDIT.md)

# 可拓展方向与贡献切入口

Everything is a Module 的价值，是让不同成长方式共享安全底座。下面是设计方向，不是已经实现的功能。优先选择可以离线、手工、只读验证的小模块，逐步扩展。

| 方向 | 第一个可交付版本 | 复用位置 | 关键验收 |
| --- | --- | --- | --- |
| 错题复习 | 从用户手工标记的日记生成复习列表 | Journal tags、Today widget | 不复制正文到日志，稳定 entry ID |
| 考公专题 | 行测专题计时与申论复盘模板 | Tasks、Focus、Journal | 复用实际会话，不重复计时 |
| 考研路线 | 可编辑科目与阶段配置 | Roadmap entities | 年度资料分开，不宣称招生预测 |
| 阅读工作台 | 手工书目、进度、摘录与阅读任务 | Journal、Roadmap | 书籍版权、页码与进度语义 |
| 语言训练 | 手工听说读写记录和复习提示 | Tasks、Journal | 音频不默认上传，新增格式需设计 |
| 论文与实验 | 实验条目模板、里程碑与成果关系 | Calendar、Vault | 数据来源可追溯，大附件有界 |
| 技能成长 | 手动技能树与练习证据 | Roadmap、Coding、Vault | 不把活动量伪装能力评级 |
| 作品迭代 | 项目版本日志和个人贡献页面 | Vault、Profile | 公开字段白名单和旧快照管理 |
| 无障碍主题 | 高对比/色觉友好内置方案 | tokens.css、Appearance | 双主题、可见焦点、reduced-motion |
| 更多语言 | 新 locale 与逐页翻译 | src/locales、locale.ts | 用户内容不翻译、日期/标签布局 |
| 互操作导入 | 用户审阅的 ICS/Markdown 导入 | Kernel data exchange | 去重、时区、演练和回滚 |
| 本地 Planner | 数据建议的解释面板 | Plan Exchange | 最小上下文、显式接受，无自动训练 |
| OS 适配 | macOS/Linux 会话 adapter | kernel/platform | 锁屏/休眠/恢复，实际账本不虚增 |
| 安全工具 | 插件 manifest/lifecycle 静态审查 | Kernel permissions/contracts | 不能借审查工具开放 native 执行 |

## 如何选第一项

先写一页问题说明：谁在什么步骤受阻、需要什么最少数据、用户如何撤销。选择一个入口和一个可观察结果。用合成 fixture 证明它有用，再决定是否需要新表或新能力。纯页面/只读 widget 不应先新增迁移。

## 官方模块与未来社区插件

现在 fork 可信源码、按 [模块教程](MODULE_TUTORIAL.md) 接入，能获得完整应用开发体验。未来分发不可信插件则受 [SDK 契约](PLUGIN_SDK.md) 限制；不能直接把 React 源码、native DLL 或 Node 包投放 extensions 文件夹来运行。想贡献 WASM runtime，应先设计资源限制、撤销、更新回滚和攻击测试，而不是仅把一个 Wasm 函数跑通。

## 贡献作品与署名

MIT 允许修改、分发与商用，保留许可/版权声明并遵守依赖许可。你可以把 fork 和自己的模块作为作品，说明哪些来自 Lumen、哪些由自己实现。文档和 PPT 可编辑，适合继续补教程、翻译、演示与真实案例。欢迎 Star 支持创作者，也欢迎比 Star 更具体的问题复现和小范围 PR。

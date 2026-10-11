# 发布资料设计参考与取舍

研究日期：2026-10-11。以下借鉴是结构与设计方法，不复制品牌视觉、商标、页面或文案，也不暗示相关公司认可 Lumen。

- [Apple WWDC25 Platforms State of the Union](https://developer.apple.com/videos/play/wwdc2025/102/)：先建立产品/平台整体，再进入具体技术与开发入口。Lumen 演示据此组织为成长场景、产品工作流、模块架构、贡献方式与可验证边界。
- [VS Code Extension API](https://code.visualstudio.com/api/) 与 [Extension Anatomy](https://code.visualstudio.com/api/get-started/extension-anatomy)：把概念、入门样例、贡献点和参考分开。Lumen 采用上手指南、可照做教程、API 索引与安全契约的文档层次；不把 VS Code API 当作 Lumen API。
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)：配置说明引用官方原生依赖，把使用安装包与从源码开发分开。

## 本次视觉方向

保留 Lumen 暖白、深森林绿与鼠尾草色，用大字、单页单主题、充足留白和真实合成界面说明核心价值。深色章节页建立节奏；产品页让界面占主视觉；技术页采用少量可编辑表格和代码摘录。中文和英文各自成套，不把两种语言挤进同一页。详细实现放在 Markdown，演示保留讲者备注与明确当前/未来状态。

原生可编辑文本、表格和注释保留修改能力；截图为隔离合成状态，无用户隐私。没有虚构用户量、速度提升、考试成绩或平台支持。可复现不等于不同机器二进制必然逐字节相同，平台候选签名和实机限制仍保留。

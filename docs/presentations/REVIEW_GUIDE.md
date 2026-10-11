# PPT 审核指南 / Presentation review guide

本轮中英文各 24 页，r6 已获用户审核通过，按字节一致映射发行 v4。旧版本保留在所有者工作站用于回溯。

Both approved r6 decks contain 24 slides and map byte-for-byte to the v4 release. Previous editions remain on the owner's workstation for traceability.

| 页码 / Slides | 审核重点 / Review focus |
| --- | --- |
| 1–3 | 封面、适用人群、Everything is a Module / Cover, audience and modularity |
| 4–9 | Today、Focus、计划反馈、Journal、Insights、Vault / Core workflows |
| 10–11 | 本地隐私与主动公开范围 / Local privacy and explicit sharing |
| 12–18 | 四层架构、官方模块、源码接入、Widget、权限、生命周期 / Architecture and extension boundaries |
| 19–20 | 可拓展方向、MIT、Fork 与可编辑资料 / Contribution ideas, licensing and editable materials |
| 21–23 | 环境、平台限制、双语技术文档 / Prerequisites, platform scope and documentation |
| 24 | 项目地址与 Star 支持 / Project link and Star invitation |

文字、代码和七张原生表格可直接编辑。四张产品截图是隔离合成数据，封面为 AI 生成的概念静物图；图片为位图。完整实现步骤在技术文档，PPT 代码页仅为摘录。WASM、AI、移动端与跨平台限制明确标注，不代表已实现。

Text, code and seven native tables are editable. Four product images use synthetic fixtures; the AI-generated cover is conceptual artwork. Images are raster assets. Code slides are excerpts; detailed instructions are in the technical documentation. Deferred runtime, AI, mobile and platform limitations are explicit.

建议审核时记录“语言 + 页码 + 希望修改的内容”。Microsoft PowerPoint 原生打开、编辑和字体替换测试未运行；已完成包结构、重新导入及逐页渲染检查。字体为 Microsoft YaHei / Consolas，替换字体后请重新检查换行。

For feedback, specify language, slide number and requested change. Native Microsoft PowerPoint open/edit/font-substitution tests are not run. Package, re-import and rendered-slide inspections are complete. Recheck wrapping after changing Microsoft YaHei / Consolas.

审核内容源：[双语 JSON](../distribution/PRESENTATION_REVIEW_CONTENT.json)。作者脚本：[build-presentation-review.mjs](../../scripts/distribution/build-presentation-review.mjs)，环境要求与旧版一致，见本目录 README。再次生成前必须选择新的修订输出与校验回执路径；不得覆盖已验收文件。

Editable content and authoring script are linked above. The environment requirements match the previous edition in this directory's README. Use fresh output and validation receipt paths for subsequent revisions.

# Lumen — Editable presentations / 可编辑演示资料

用户已通过 r6 审核，发行文件命名为 Lumen-Launch-Developer-zh-CN-v4.pptx / Lumen-Launch-Developer-en-v4.pptx，字节与审核版一致。见 [审核指南 / Review guide](REVIEW_GUIDE.md) 与 [发行映射](PRESENTATION_RELEASE.json)。This approved v4 edition is byte-identical to the accepted r6 review decks.

中文与英文各 24 页、16:9，覆盖成长场景、工作流、隐私、模块架构、接入代码、Widget、未来权限、环境及平台边界。

Download / 下载：[公开发行页](https://github.com/wangziwen2794928256/Lumen/releases/tag/v0.1.0-docs-i18n-rc1)。文件：Lumen-Launch-Developer-zh-CN-v4.pptx、Lumen-Launch-Developer-en-v4.pptx。

文字、代码、7 张原生表格及页面对象可编辑。4 张功能截图来自隔离浏览器合成数据，为位图，不能逐字编辑，不代表原生验收；新增封面为 AI 生成概念静物图。备注包含来源和实现边界。没有使用真实任务、日记或私人窗口截图。

Text, code excerpts, seven native tables and slide objects are editable. Four raster screenshots are feature excerpts from synthetic browser fixtures. Speaker notes document sources and limitations. No production records are included.

直接在 PowerPoint 编辑副本，无需开发工具。保持 Microsoft YaHei / Consolas，替换字体后检查换行。源码与自制资料遵循根目录 MIT 许可，保留许可和署名；第三方依赖保留各自许可。

Edit a copy in PowerPoint without developer tools. Keep Microsoft YaHei / Consolas or recheck wrapping after font substitution. Project-created materials follow the root MIT license with attribution retained; dependencies retain their own licenses.

重生成输入 / Rebuild inputs：[双语内容 JSON](../distribution/PRESENTATION_CONTENT.json)、[作者脚本](../../scripts/distribution/build-presentation.mjs)。合成 PNG 在 docs/distribution/presentation-assets 中公开。脚本需要环境提供 @oai/artifact-tool，在仓库根目录执行 node scripts/distribution/build-presentation.mjs。该作者工具来自配套 Codex 环境，不属于应用构建依赖；普通 Node 环境不保证具有该工具。可选最终校验需要 LUMEN_PRESENTATION_SKILL、LUMEN_PRESENTATION_PYTHON、RUNTIME_NODE_MODULES。为修改版本选择新输出与回执路径，拒绝覆盖已验收文件。

v4 内容源为 docs/distribution/PRESENTATION_REVIEW_CONTENT.json，作者脚本 scripts/distribution/build-presentation-review.mjs，概念封面位于 docs/distribution/presentation-review-assets。作者脚本导入原 v3；v3 仅作为重建输入保留在双语技术文档 ZIP 的 docs/presentations 目录中，解压到仓库根目录后使用。PPT 不进入源码 ZIP，源码 ZIP 和技术文档 ZIP 配合使用。再次生成前改用新的输出文件与校验回执路径。

Rebuilding requires an environment-provided @oai/artifact-tool. Run the authoring script from the repository root. The v4 review script imports the original v3 decks; these authoring inputs are preserved under docs/presentations in the bilingual documentation ZIP. Extract that ZIP into the source checkout. Source ZIPs exclude PPT files. This authoring tool is separate from app prerequisites. Optional finalization uses the environment variables above. Select new output and receipt paths for revisions. The new cover is AI-generated conceptual artwork.

两份最终包通过结构、几何、字体策略、原生表格、重新导入校验，逐页渲染检查完成。Microsoft PowerPoint 原生打开/编辑/字体渲染测试未运行。

Both final packages passed integrity, geometry/font-policy, native-table and first-party import validation. All rendered slides were inspected. Native Microsoft PowerPoint open/edit/font-rendering tests remain unrun. Community WASM execution, AI calls and full macOS/Linux integration are deferred.

# 公开导出与本地网站

Phase12 新增 Official Profile；白名单导出、预览服务器、深链校验由 Kernel 管理。本阶段不部署网站。GitHub 连接不代表授权上传个人数据。

## 操作顺序

1. 在作品库中明确选择公开作品。已归档或删除的作品不导出。
2. 个人档案中编辑介绍与 HTTP/HTTPS 联系链接，默认全部私有；勾选公开并保存后才进入投影。不读取 users 表推断姓名。
3. 检查已保存的公开资料。导出/预览校验内容指纹；其他窗口变更公开资料后必须重新检查。
4. 手动导出到 AppData `exports/website-UUID/`。生成 index.html、site.js、site.css、public-profile.json，最后写 COMPLETE（JSON SHA-256）。不复制 SQLite、附件或其他私有文件。
5. 在应用中打开本地预览；或独立运行 `node scripts/preview-website.mjs <导出目录>`，Ctrl+C 停止。独立预览不需要应用或主数据库。

## Public Schema 1.0

顶层仅 schema_version/exported_at/profile/artifacts。profile 可以为 null；公开档案仅名称、简介、正文、联系链接、public=true、revision=0（固定兼容值，不导出编辑历史）。作品仅 id/title/description/kind/date/role/organization/skills/technologies/links/public=true。明确公开的作品描述按原文导出，请在导出前检查正文。私有 Journey/Competition 的关联与标题、附件路径、附件 manifest、编辑版本和数据库内部时间状态不会导出。

Journal/Mood/Quick Capture、音乐历史、原始 Focus 和私人笔记在1.0版本不支持公共导出，即使 visibility 存在也不会纳入。网站的比赛/项目/研究/奖项是明确公开的相应类别作品。网站拒绝未知字段、schema、private 实体和危险 URL。

## 隐私与运行边界

导出是不可变快照。撤销公开影响后续导出，不会删除旧文件；用户自行管理旧快照。IO 失败可能留下无 COMPLETE 的不完整目录，不能视作成功导出。无自动上传、CDN、遥测或第三方脚本。

内置预览只绑定127.0.0.1随机端口，内存中只服务五个精确 GET 路径。Host 防 DNS rebinding，无 CORS/数据库端点/路径遍历/SQL查询。同机其他进程可能读取已明确公开的预览内容；退出应用即结束。网站以 textContent 显示正文，不执行 HTML；外链 noopener/noreferrer，设置 CSP。磁盘 JSON 也会重新校验。

## Deep Link

手动注册 HKCU 当前用户的 lumen 协议，指向当前 exe；移动 exe 后需要重新注册。仅接受 lumen://today、lumen://calendar、lumen://journal/YYYY-MM-DD、lumen://artifact/规范小写UUID、lumen://journey/规范小写UUID。拒绝 query/fragment/百分号编码/遍历和其他协议，不授予文件读取或向网站返回本地内容。作品打开本地有效记录编辑器，日记筛选日期，Journey 高亮相应本地卡片；禁用模块仍受 guard 保护。

Windows 协议启动后由 Kernel 单实例回调校验路由，异步恢复/聚焦已有窗口再导航（Phase14）。macOS/Linux 注册/浏览器 adapter 暂未实现；不绕过浏览器协议提示。联系外链由用户提供，不验证目标真实性。

## 复现

按 BUILD_FROM_SOURCE、TESTING 中显式命令运行；当前60项 E2E 包含三尺寸、网站脱离数据库与私有哨兵，Rust 覆盖迁移备份回滚。原 Phase12 验收脚本/原生报告保留本地。原生合成测试只使用隔离 QA；schema10、001–010 migration 不变。

Windows Shell may append a single root slash to authority-only links: `lumen://today/` and `lumen://calendar/` are also accepted. Double slashes and all other additional paths remain rejected. Windows 系统可能给无参数入口补一个末尾斜杠；仅 Today、Calendar 兼容这一形式。

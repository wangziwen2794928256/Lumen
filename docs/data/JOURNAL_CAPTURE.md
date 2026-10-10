# Journal / Quick Capture 数据契约

Phase 7，仅新增 migration 005，不修改 001–004。Journal 默认私密，无种子，无自动上传。

## 数据与无限记录

journal_entries 保存 Markdown、日期、八种类型、草稿状态、revision 与 Journey/Task/Music 外键。journal_tags 参数化去重，编辑采用软删除再恢复；记录没有每天数量限制。分页 30 条，以日期/创建时间/id 稳定排序并单独 count，不把全部正文载入内存。

FTS5 trigram 支持三字及以上中文子串与英文短语。关键词始终为转义后的字面短语，不当作搜索运算符；1–2 字、草稿和回收站使用转义 LIKE。类型、标签、日期范围在本机过滤，搜索索引不包含已删除记录或草稿。全应用其他模块仍保留既有能力，后续模块接入统一搜索时复用此本地契约。

## 崩溃、安全与冲突

每次输入同步写 renderer localStorage，按窗口类别和 client UUID 分键，多份未确认草稿不会互相覆盖。停止输入 600ms 后串行提交完整 Save；日记正文不进入日志。SQLx SQLite synchronous FULL/WAL；成功回执后才清除对应本机副本。未发送/未确认草稿下次启动提供恢复；也可在草稿分页中继续已落库草稿。

服务先以独立事务持久化 journal_recovery，再校验 revision/关联并写条目、标签和 receipt。冲突不回滚恢复副本，也不覆盖另一个窗口。UUID + sequence + payload SHA256 防止重试重复；receipt 只存哈希，不复制历史全文。旧 sequence 不能覆写更新的 recovery。

编辑已保存记录继续保持已保存；新条目自动保存为草稿，点击保存才进入已保存列表。恢复为新记录保留原 id 的正文和 revision，不静默覆盖。无网络情况下仍运行。磁盘或 localStorage 故障显示失败提示；任何尚未持久化且没有本机备份的输入无法保证在系统掉电后恢复。

软删除/恢复校验 revision，回收站保留标签与附件；撤销按钮恢复刚删除的版本，遇后续更新拒绝覆盖。本阶段没有永久删除功能。

## 附件

文件仅从 Rust 官方文件选择器返回路径导入，renderer 不提供任意源路径。仅支持 PNG/JPEG/WebP/PDF，按内容签名验证，最大 20 MB。随机 UUID 目标名、create_new、写入 fsync 后才发布 metadata；路径/hash/大小/mime 在 attachments，关联在 journal_attachment_links，无 blob。

受控目录为 Tauri AppData/attachments。验证拒绝 ..、绝对路径、目录/文件链接逃逸；逐次检查大小及 SHA256，显示完整/缺失/变化状态。只有通过校验的受控文件可以交给 Windows 默认查看器。记录删除不物理删除文件。磁盘写入完成但事务未提交时可能残留未关联文件；保守保留，不自动清理用户附件。

链接仅 HTTP(S) 且须具有 host。Markdown 禁用原始 HTML 和危险 URL，不自动下载远程图片；图片通过附件查看。

## 独立小窗口和快捷键

默认 Control+Shift+Space。Tauri 官方 global-shortcut 2.4.0 在进程运行时注册；独立 capture 窗口 520×560、置顶，与主窗口共享同一个库及 IPC。保存完整正文、标签、类型；正文第一行可生成标题，成功后开始下一条。原生标题栏直接关闭的未确认内容保留本机草稿。

快捷键冲突显示提示，按钮仍可打开。设置替换先注册新组合，再保存设置并注销旧组合；冲突保留原组合，存储失败撤销新注册。程序退出后不注册；无额外后台服务或自启动设置。QA 与正式实例会争用相同组合，应顺序运行。

依赖决策：使用 [官方快捷键插件](https://v2.tauri.app/plugin/global-shortcut/) 和 [官方对话框插件](https://v2.tauri.app/plugin/dialog/)，仅 Rust 调用，不授予 renderer 任意文件/快捷键插件权限。Markdown 使用 react-markdown 10.1.0，无 raw HTML 扩展。

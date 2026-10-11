# 模块技术与使用参考

本卷对应当前源码，不把历史数据字典中的阶段限制当作最新发行结论。每节包括实际工作流、存储/服务原理、边界与可扩展方向。API 形状见 [API 参考](API_REFERENCE.md)，接入步骤见 [模块实作教程](MODULE_TUTORIAL.md)。

## 今日与任务

用途：把当天最值得完成的事、容量与随手记放在同一工作面。考研可按科目建任务，考公可按行测/申论拆分，语言学习与作品冲刺同样适用。任务名称完全由用户决定。

使用：新建任务，填写标题、本地日期、1–1440 分钟估时和优先级。开始专注前确认当前任务。完成、跳过、取消、移走与移入回收站具有不同语义，误删可恢复。未来日期任务到期后进入 Today；容量超额提示建议减少或移走任务，不强制加班。

原理：前端 useToday 获取 typed snapshot，操作通过 today_action 到 TodayService。Rust writer 锁与事务协调任务、会话和规划写入；UUID 请求回执与 payload hash 使相同请求重试不重复创建。revision 拒绝陈旧覆盖。估时属于计划，实际来自有效专注区间，两者不能混写。

扩展：增加题库待办、论文写作清单或阅读任务应复用任务语义。首页 widget 只能贡献界面，不能私自把一次渲染当作完成任务。与目标关联时保留 task UUID，不把标题当主键。

验收：同一请求重复提交、陈旧 revision、跨窗口忙状态、回收站恢复、当天容量与未来任务边界。浏览器预览无数据库，看到空状态不是数据丢失。

源码 / Source: [src/pages/today/TodayPage.tsx](../../src/pages/today/TodayPage.tsx) · [src/features/today/model.ts](../../src/features/today/model.ts) · [docs/data/TODAY_CORE.md](../../docs/data/TODAY_CORE.md)

## 专注与会话安全

使用：从已有任务开始，按需要暂停、继续或中断。完成并结束才计入有效成果；取消会话不算成果，任务重新安排。崩溃后选择继续、按最后活动结束或取消，离线间隔不补计。

原理：会话状态机区分运行、暂停、中断及终态。实际账本保存确认活动片段，跨日片段按本地日期归属。锁屏/休眠事件通过 Kernel Windows adapter 进入统一服务，停止累计；恢复不会自行开始专注。checkpoint 支持重启恢复，系统时间变化也不能凭墙钟推造离线努力。

隐私与边界：没有窗口标题、浏览器地址或私人正文采集。暂停时间、休眠时间和离线时间不算专注。macOS/Linux session adapter 尚未实现，Today 初始化显式报 NATIVE_HOOK_UNAVAILABLE，因此这些原生实验包的 Today/Focus 当前不可用，不能用浏览器 mock 的通过代替实机支持。

扩展：番茄钟、复习间隔提示、实验计时面板应消费已有会话 API，不能另外叠加第二套实际时长。适配新 OS 必须在 kernel/platform 内完成系统事件处理并验证锁屏、断连、休眠、时钟跳变和崩溃恢复。

源码 / Source: [src/pages/focus/FocusPage.tsx](../../src/pages/focus/FocusPage.tsx) · [src/features/today/model.ts](../../src/features/today/model.ts) · [docs/data/FOCUS_CALENDAR.md](../../docs/data/FOCUS_CALENDAR.md)

## 日历与固定占用

使用：切换日、周、月或议程；分别查看计划与实际。固定事件可设全天或开始/结束时间，支持学习、课程、比赛、考试、截止日期、论文及个人安排。定时事件最多七天，结束必须晚于开始。误删事件从事件回收站恢复。

原理：date-only 表示本地日历日期，UTC 时间戳表示有时刻的事件。计划虚线与实际实线使用不同来源，未指定时刻的任务只按日期展示，不补造一个精确时段。规划器按本地日切分固定事件，合并重叠时段后扣减容量；全天事件保守占满当日容量。

扩展：考试倒计时、学期日程或科研节点可以先作为固定事件编排。ICS 导入属于未来扩展，需要显式导入预览、时区策略、重复事件和去重规则，当前不能宣称已支持。固定事件变化后重新生成方案，旧接受方案保留并提示容量风险。

验收：跨日、夏令时与设备时区差异，重叠扣减不重复，全天与定时互斥，日期导航不越界，事件回收站恢复。浏览器表单验证并不能替代 Rust 校验。

源码 / Source: [src/pages/calendar/CalendarPage.tsx](../../src/pages/calendar/CalendarPage.tsx) · [src/features/calendar/model.ts](../../src/features/calendar/model.ts) · [docs/data/FOCUS_CALENDAR.md](../../docs/data/FOCUS_CALENDAR.md)

## 目标与确定性规划

使用：建立 Journey、阶段、学科和支线；设置每周最低/偏好/上限分钟、优先级、父关联及手动或任务比例进度。工作日/周末容量和每日覆盖值分别管理。任务可以配置最早开始、deadline、剩余估时、单次最小/偏好分钟、是否可拆与固定时间。

原理：先分配能满足的最低量，再处理偏好量，再按截止日期、slack 和优先级填充。每天不超过扣除固定占用后的 available，每任务不超过 remaining，遵守目标与支线上限。不可拆任务放不下时明确保留未安排，不声称全局最优。精力要求是人工标签，不推断用户精力。

确认：Generate 只写 proposed 审计，Accept/Modify 重新验证 basis hash 和所有跨字段约束。before/after/reason 和 accept/reject 保留调整历史。Undo 仅能回撤当前最新被接受版本，且不能覆盖后续修改。容量变化不会静默改写旧计划。

模板与扩展：Fudan 2028 是主动创建的可编辑学习模板，不是招生预测。年度规则仅保存用户提供的官方来源文本，哈希验证该文本。可扩展考公科目模板、语言等级路线或技能树。External LLM/Local Planner 只能准备数据交换方案，本版没有真实模型调用或训练。

源码 / Source: [src/pages/roadmap/RoadmapPage.tsx](../../src/pages/roadmap/RoadmapPage.tsx) · [src/features/roadmap/model.ts](../../src/features/roadmap/model.ts) · [docs/data/ROADMAP_PLANNER.md](../../docs/data/ROADMAP_PLANNER.md)

## 日记、搜索与快速捕获

使用：新建 Markdown 记录，选择八种类型、日期、标签与目标/任务/音乐关联。编辑器展示本机自动保存状态；新记录自动保存为草稿，明确保存后进入正式列表。Quick Capture 可用 Ctrl+Shift+Space 或按钮打开，快捷键冲突不阻止手动操作。分页与本地日期/类型/标签/正文搜索可组合。

原理：每次输入同步保留 renderer localStorage 副本，停止输入约 600ms 后串行提交 Save。journal_recovery 先独立持久化，随后检查 revision 与关联，再写条目、标签和 receipt。UUID/sequence/hash 防重复与旧输入覆盖；成功 ACK 后才清除对应副本。冲突允许恢复为新记录并保留原文。

搜索：FTS5 trigram 支持三字及以上中文子串与英文短语，1–2 字、草稿和回收站走转义 LIKE。查询按字面短语处理，索引排除草稿和删除记录。搜索只在本机执行。

附件：Rust 文件选择器导入 PNG/JPEG/WebP/PDF，签名验证与 20 MB 上限，UUID 文件名与 fsync 后提交元数据。逐次校验路径归属、大小和 SHA256。Markdown 不开放原始 HTML 或自动远程图片。

扩展：错题复盘、实验日志、阅读摘录或写作提示可复用条目类型和关联。磁盘与本地存储同时失败时不能保证未持久化输入恢复，界面会保留错误提示而不假报成功。

源码 / Source: [src/pages/journal/JournalPage.tsx](../../src/pages/journal/JournalPage.tsx) · [src/features/journal/model.ts](../../src/features/journal/model.ts) · [docs/data/JOURNAL_CAPTURE.md](../../docs/data/JOURNAL_CAPTURE.md)

## 编程练习与真实用时

使用：手动添加题目、平台/题号、难度、算法标签及竞赛。每次尝试记录日期、语言、结果、笔记和额外手工分钟。可关联已完成专注，但同一会话只能关联一条尝试。平台链接由用户点击打开，不自动抓取账号历史。

原理：题目与尝试是独立实体，平台/题号用于检测重复题目，尝试保留真实过程。manual_seconds 表示专注之外的额外时间；focus_seconds 从关联的实际区间读取。两者不能再次把同一会话加入别的尝试。周统计按尝试日期归属；练习题数按不同题目计，首过率以首次尝试为分母。

边界：未标注难度与未填写主题保留空语义，不自动推断题目质量或能力排名。最近 200 条尝试的展示限制不限制统计。删除/恢复采用软状态，未确认输入保存在本机。

扩展：手动题单、算法复习提醒、面试计划、代码片段关联都可基于既有记录。OJ API 导入需单独设计显式授权、频率限制和去重，本版没有自动爬虫。验收重点是重复题目、重复会话关联、陈旧 revision、周边界和回收站。

源码 / Source: [src/pages/coding/CodingPage.tsx](../../src/pages/coding/CodingPage.tsx) · [src/features/coding/model.ts](../../src/features/coding/model.ts) · [docs/data/CODING.md](../../docs/data/CODING.md)

## 洞察与回顾

使用：查看实际专注趋势、计划与实际、年度热图、词云、周回顾和月快照。特殊日可标休息、生病、考试或旅行，不因此抹掉记录。心得和下周关注由用户写入，与系统事实分开。

原理：六项真实信号按可调权重求和，再与四个递增阈值比较得到 0–4 强度。特殊日从 7/28 日一致性分母排除，不修改规划容量。编码时间包含专注关联，与 deep focus 有交集，不能直接相加。任务完成是当前计划日状态，不冒充历史完成事件。

分词：jieba-rs 本机词典与中英停用词，按正文 hash 维护可重建缓存，每批 250 条；词云最多显示 50 词，每词至多 20 来源。音乐 Song/Artist/Album 云与日记词云分开，不混入用户正文。

快照：结束月份封存生成时事实，当前月读取时更新；旧非 JSON 快照保留。往年今日严格取相同月日，闰日不替换。私密 JSON/PNG 导出白名单排除正文和心得，但统计词语仍可能敏感，应自行保管。

架构：Insights 是首个 Official Module，Today 的 on-this-day widget 由静态可信贡献描述器接入。扩展可增加透明的复习分析或阅读统计，必须说明分母、重叠和来源，不能制造道德评分或虚假精准预测。

源码 / Source: [src/pages/insights/InsightsPage.tsx](../../src/pages/insights/InsightsPage.tsx) · [src/features/insights/model.ts](../../src/features/insights/model.ts) · [docs/data/INSIGHTS.md](../../docs/data/INSIGHTS.md)

## 音乐与可选媒体观察

使用：手工录入歌曲、艺术家、专辑、播放时间、实际聆听秒数与日记关联，保存/修改/回收站操作均可离线进行。默认不监听。Windows 用户可明确开启来源 ID 精确匹配的媒体观察，并设置 5–600 秒计入阈值，默认 30 秒。

原理：手动编辑建立独立 track，避免改写其他历史播放的元数据。自动事件使用 identity/event key 去重，达到阈值只增加一次播放，之后单调更新时长。暂停不计时，缺元数据回落手动；不会控制播放器或调用网络。系统没有统一广告标志，未识别广告可能需要手动移除。

查询：按本机 localtime 归属日期，日期间隔最多 366 天。列表最新 100 条、云最多 500 组，显示前 50；总数/秒数查询全量范围。音乐不进入公开网站导出。Spotify 仅为关闭状态的未来接口，没有 OAuth 流程。

扩展：专注歌单备注、艺术家回顾、手动唱片日志和歌词学习记录。联网元数据、歌词与播放器控制均需要独立权限设计及许可评估，当前没有。macOS/Linux 媒体监听返回 MEDIA_UNSUPPORTED，手动录入为可用路径。

源码 / Source: [src/pages/music/MusicPage.tsx](../../src/pages/music/MusicPage.tsx) · [src/features/music/model.ts](../../src/features/music/model.ts) · [docs/data/MUSIC.md](../../docs/data/MUSIC.md)

## 作品库与附件

使用：添加项目、奖项、证书、课程项目、开源、研究或演示，填写日期、角色、组织、技能/技术与仓库/项目/论文链接。默认私密，只有主动设为公开才可进入档案导出。归档、回收站与恢复保留附件。

原理：010 迁移扩展既有 artifacts/artifact_files，public_visibility 管理公开状态，缺少可见性行仍视为私密。save/state 通过 BEGIN IMMEDIATE、revision 和请求回执实现一致性。最新 1000 项用于列表与预览，不能声称无限分页。

附件：先保存作品，再用原生选择器导入 20 MB 以内图片/PDF。受控 attachments 目录保存 UUID 文件与 SHA256，metadata 保存原名、大小、MIME；查看前再次校验归属和完整性。签名验证不能替代恶意文件扫描。文件写完而事务失败可能留下未关联文件，程序保守保留，避免误删。

扩展：作品可以承载考试阶段成果、课程设计、论文资料或个人项目。关联比赛/Journey 仍为私有关联，公开预览不带原始附件和关联名称。自定义作品分类需要同步 Rust 枚举、前端模型、校验与迁移兼容，不能只改按钮文字。

源码 / Source: [src/pages/vault/VaultPage.tsx](../../src/pages/vault/VaultPage.tsx) · [src/features/vault/model.ts](../../src/features/vault/model.ts) · [docs/data/VAULT.md](../../docs/data/VAULT.md)

## 个人档案与可控公开

使用：填写介绍和联系链接，决定是否公开；先保存，再检查 public projection，最后手动导出静态网站和本地预览。作品的公开状态在 Vault 设置。修改公开范围后应重新导出，并自行处理旧快照。

原理：Kernel public_export 只读取白名单字段，筛选明确公开、未归档、未删除作品。日记、心情、音乐历史、专注原始区间、附件与私有关系从投影中排除，前端不能请求“导出全部”绕过边界。网站是独立快照，没有后台自动发布。

链接：HTTP(S) 输入验证和渲染限制共同减少危险 URL。lumen:// 只接受预定义路由，不接受 arbitrary 文件或 SQL。Windows 重复启动转发由 Kernel 校验并聚焦现有窗口。macOS/Linux 注册和预览打开目前显式不支持。

扩展：用户可把 MIT 代码 fork 成自己的作品网站，保留许可与作者声明，明确列出自己的修改贡献。作品介绍应来自真实输入，不生成虚假学历、成绩或经验。公开导出是用户主动分享路径，不能把 local-first 宣称为“用户永远无法泄露任何内容”。

源码 / Source: [src/pages/profile/ProfilePage.tsx](../../src/pages/profile/ProfilePage.tsx) · [src/features/profile/model.ts](../../src/features/profile/model.ts) · [docs/en/PUBLIC_WEBSITE.md](../../docs/en/PUBLIC_WEBSITE.md)

## 隐私、备份与恢复

使用：隐私中心查看本地目录、断开集成、私有 JSON/CSV/Markdown 导出、审阅导入和隔离恢复。完整备份另含数据库、设置和附件，便携 JSON 不包含附件文件、设置、凭据或授权。恢复演练不覆盖当前库。

原理：数据库采用 WAL、synchronous FULL、外键和参数化查询，迁移验证历史 checksum，写入前创建一致性备份。每日调度运行时约每五分钟检查，当日只保留一个有效成功快照；按日/周/月桶去重保留，只有新快照验证后清理过期 daily。手动、迁移和异常备份不自动删。

导入：先解析和事务演练后回滚，展示新增/更新/冲突/无效数量；用户确认同一文件快照且当前 basis 未变后才备份并写入。冲突和无效记录阻止确认。恢复演练检查 COMPLETE、校验和、schema、外键和样本，再在隔离路径实际打开。

边界：Kernel 安全不可停用/卸载。OS 账户、磁盘加密、用户分享和第三方应用仍是信任边界，当前 SQLite 不宣称应用层加密。扩展插件不得直接接触 lumen.db、备份或 exports，读取敏感上下文必须另行设计最小能力。

源码 / Source: [src/features/privacy/PrivacyCenter.tsx](../../src/features/privacy/PrivacyCenter.tsx) · [src/features/privacy/centerText.ts](../../src/features/privacy/centerText.ts) · [docs/en/BACKUP_PRIVACY.md](../../docs/en/BACKUP_PRIVACY.md)

## 桌面、外观与语言

使用：设置按通用、外观、学习、计时、日历、音乐、隐私、备份、数据、快捷键、高级和关于组织。主题有浅色/深色/跟随系统，强调色统一在此修改。模块支持启用、隐藏、置顶、拖动和上下按钮，停用不删记录。

语言：外观里选 English 或简体中文，保存当前编辑后应用并重新载入。lumen.locale.v1 仅保存界面偏好，缺省或异常值回落中文。905 条现有前端静态文案由本机英语包映射，导航从 Manifest 双语 name 读取，日期格式使用当前 locale。用户正文、目标、题目与文件名不翻译。新增文案必须补充语言包并回归；原生系统对话框跟随 OS，现有 Rust 托盘/通知正文仍为中文，完整原生壳层国际化列入后续工作。

原理：外观在 localStorage 持久化，通过 HTML data-theme/data-accent 与 CSS tokens 驱动。模块配置使用数据库 revision，置顶分组内排序，键盘按钮与拖动等价。Tauri 管理托盘、单实例、自启动、通知和 capture 窗口；通知默认关闭且不含私人正文。

扩展：主题作者修改 tokens.css 与 appearance 类型并验证浅/深色、焦点、对比度、reduced-motion 和三个窗口尺寸。当前没有从任意目录加载 Theme Pack 的运行时，不能把主题文件夹当已开放 CSS 插件加载器。

源码 / Source: [src/pages/settings/SettingsPage.tsx](../../src/pages/settings/SettingsPage.tsx) · [src/features/desktop/model.ts](../../src/features/desktop/model.ts) · [docs/en/DESKTOP.md](../../docs/en/DESKTOP.md)

## 官方模块清单与状态

这些是随源码捆绑的可信官方功能，不是可以在线安装的第三方插件。manifest 的 status 描述迁移状态，不等于每个平台都已原生验收。

| ID | Route | Layer | Status |
| --- | --- | --- | --- |
| `lumen.today` | `/` | kernel | stable |
| `lumen.focus` | `/focus` | official | legacy-bridge |
| `lumen.calendar` | `/calendar` | official | legacy-bridge |
| `lumen.roadmap` | `/roadmap` | official | legacy-bridge |
| `lumen.journal` | `/journal` | official | legacy-bridge |
| `lumen.coding` | `/coding` | official | legacy-bridge |
| `lumen.insights` | `/insights` | official | module |
| `lumen.music` | `/music` | official | module |
| `lumen.vault` | `/vault` | official | module |
| `lumen.profile` | `/profile` | official | module |

已拆出官方入口：insights、music、vault、profile。其余 official legacy-bridge 仍复用稳定页面与 Service，后续逐步迁移，不推倒重写。Kernel Today 必须保留；安全系统无可卸载插件入口。

# Insights 与回顾数据字典

版本 Phase9 / schema7。旧迁移 001–006 不变；仅新增 007。所有资料、分词和导出在本机，默认 private。

## 日指标与日期

范围闭区间，支持 7/30/90/365 天、自定义最多 3661 个日格；缺日补零，不填假记录。年度热图以客户端当前年的 1 月 1 日至 12 月 31 日显示，每列一周。未来格不计入近 7/28 日一致性。

| 信号            | 来源与单位                                                                      | 去重/边界                                                            |
| --------------- | ------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| deep_minutes    | 有效 focus_sessions 的 focus_intervals.elapsed_ms / 60000，按已记录 local_date  | 不用 planned_seconds；保留 legacy_total 的原历史归属，不伪造精确区间 |
| coding_minutes  | 有效题目/尝试关联专注的实际跨日区间，加 attempted_on 当天的 manual_seconds / 60 | Focus 唯一引用由 Phase8 约束；与 deep 有交集，不能相加为总用时       |
| tasks_completed | 有效 tasks 中 lifecycle=done，按 due_date                                       | 当前计划日状态，不声称历史完成时间；无日期任务不虚构归属             |
| journal_count   | 有效且已发布 Journal，按 entry_date                                             | 含用户 Quick Capture 与 Reflection；排除草稿                         |
| problem_count   | 当天有效尝试的 DISTINCT problem_id                                              | 一题多次尝试当天只计一次，不限是否通过                               |
| music_count     | 有效播放且 track 有效，按 SQLite localtime(played_at)                           | UTC 源时间保留；本机时区改变会影响音乐/作品按日展示                  |

任务计划预算使用 due_date 下未取消有效任务 estimated_minutes；完成比例分母也是同范围未取消计划任务。移动/未完成单独显示。这里只能从当前任务状态计算 rolling completion，未保存完整历史完成事件，绝不预测考试分数。

每日容量读取 capacity_days，再读取 planning_preferences 工作/周末值，最后兼容 daily_capacity_minutes（默认 210）。活动特殊日状态独立存储，不修改这些容量或任务。Journey 图表表示范围内计划任务的完成比例，不冒充总体人生进度；科目按实际区间归属任务的 subject。

## 强度与 No-Guilt

六信号依次乘 weights，默认 `[1,1,10,10,10,1]`；与递增四阈值 `[1,30,90,180]` 比较，结果 0–4。权重 0–100，允许零；阈值正数、递增、有限。主题颜色来自 Design System，跟随浅深色及用户强调色。

status 支持 normal/rest/sick/exam/travel/paper_deadline/special；兼容旧 capacity_days 的 competition/paper_sprint/other 展示。显式 insights_days 优先。特殊日从近 7/28 日一致性分母剔除，不清零实际数据；活跃定义为任一真实信号大于零，独立于用户强度权重。无合格样本展示“暂无样本”。无 streak 惩罚或道德分。

## 表与写入

| 表                   | 契约                                                                                                                             |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| insights_preferences | id=1；JSON preferences（weights/thresholds/stop_words/memories）、revision、updated_at；缺省在代码中，不注入用户记录             |
| insights_days        | date 主键，status、revision、updated_at；只影响展示与一致性                                                                      |
| insights_receipts    | UUID 主键、payload SHA256、created_at；与业务写入同 BEGIN IMMEDIATE 事务，完全相同请求重试返回成功，内容变化或陈旧 revision 拒绝 |
| insights_words       | (kind,id) 主键，date、正文/标题 hash、分词 JSON；可重建衍生缓存，不作为事实来源                                                  |
| weekly_reviews       | 旧 body 为 My Reflection；新增 next_focus 与 revision；week_start_date 归一到周一，UUID 保持，自动数据与用户心得分离             |
| monthly_snapshots    | 旧 summary 保持；新版本 summary 为事实 JSON，revision 增长；已封存月保持生成时版本，进行中月读取更新                             |
| music_tracks         | 新 album 默认空，无音乐种子；播放接入与管理在 Phase10 完善                                                                       |

用户输入参数化 SQL，UUID、标准日期、长度、枚举和算法参数在 Rust 检查。保存失败/冲突保持待确认稿；UUID 与内容在稿中持久化，只有 ACK 后清理对应稿。周回顾冲突可重新读取版本后保留输入再显式保存。

## 分词与来源

jieba-rs 0.7.4 嵌入词典，默认分词、关闭 HMM，中文/拉丁字符小写归一；排除基础中英停用词、单字符、纯数字与无意义符号。自定义停用词最多 500 个，每词 100 字节。用户文本从已发布日记/快速记录/Reflection、weekly_reviews.body/next_focus 读取；系统总结、任务和音乐标题不混入 Journal 词云。

按日期/UUID 游标每批 250 条；正文指纹变化重分词。删除/草稿源不参与统计，恢复后重新读取，不依赖旧缓存的可见性。批内事务减少写盘，缓存批量读取避免每条 roundtrip。词云前 50 词，频数计入全部有效源，每词列最多 20 个来源；日记链接跳到准确 entry，回顾链接跳到准确周。音乐维度可选 Song/Artist/Album，计数或收听分钟单独聚合，缺 album 不推测。

## Day Detail、往年今日与快照

当日显示专注/科目、计划与实际、任务、编码题目、日记、音乐、日程、竞赛、成就、项目及照片。有记录才显示，没有则真实空状态；元数据来源最多 500 条，聚合统计不受截断影响。

往年今日取精确月日、早于当天的 Journal/Song/Study/Achievement/Photo，最多五条；闰日不替换成 2 月 28 日。可关闭 Today 卡片。

应用启动后及每小时检查有真实来源且未封存的已结束月份，自动生成快照；当前月份打开时生成/更新。后台失败不会重置原数据，可在月视图重试并显示错误。旧非 JSON 快照保持，不盲目覆盖。封存后追加/修改源不会改写历史快照，生成时间明确保留。

月快照包含深度专注、科目、编码/主题、音乐、词语、计划任务与竞赛/成就/项目/照片计数，以及最多 20 条日记元数据摘选（无正文）。统计计数独立全量查询，不从 500 条摘选数推断。

本地 JSON 导出采用字段白名单，排除日记摘选/源 UUID/正文/心得，含统计标签、词语与特殊日状态，标为 private；PNG 是 1200×800 的统计摘要。两个按钮显式触发，不自动发布。PDF 留给后续 minor 版本。

## 性能复现

`scripts/environment.ps1` 后设置 `CARGO_TARGET_DIR=D:\LumenToolchain\target-phase5`、`CARGO_BUILD_JOBS=1`、`LUMEN_PERF_REPORT=D:\Lumen\docs\testing\phase9\performance.json`，执行：

`cargo test --manifest-path src-tauri/Cargo.toml --lib insights_ten_year_performance_baseline -- --ignored --nocapture`

TempDir SQLite 自动生成 3650 天、100k focus、100k music、50k journal、10k problems/attempts；断言完整统计及词频，并记录生成/十年冷热/七日热查询。普通门禁忽略此重型测试，独立明确执行，不能把 ignored 当成已测。fixture 为短中文正文，不代表 5 万篇 1MB 正文；数字仅为当前机器 debug 基线。图表绘制所有日期点，数据表每页 90 日，避免一次挂载数万 DOM 行。

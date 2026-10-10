# 数据库核心规范 · Phase 3

## 数据目录

正式入口只接受 `app.path().app_data_dir()`：当前标识 com.lumen.desktop。目录中包含 lumen.db、WAL/SHM、migration.lock、attachments/、backups/migration-UUID/。真实路径由 Tauri 解析，不硬编码用户目录，不放在仓库。

测试使用 TempDir，绝不打开正式数据库。Git 排除 db/WAL/profile。禁止覆盖、重建损坏库或 DROP 生产表。多进程迁移锁冲突返回 DATABASE_BUSY。

## Schema

权威 schema：src-tauri/migrations/001_initial.sql。26 个领域表 + migration_history，另有 FTS5 content_search 与内部 shadow tables。

| 领域 | 表 |
|---|---|
| 用户/偏好 | users、settings |
| 成长计划 | journeys、journey_phases、subjects |
| 任务/时间 | tasks、task_sessions、calendar_events、focus_sessions |
| 挑战/编程 | quests、competitions、coding_problems、coding_attempts |
| 日记/音乐 | journal_entries、journal_tags、music_tracks、music_plays |
| 作品/回顾 | artifacts、artifact_files、weekly_reviews、monthly_snapshots |
| 快照/指标 | exam_rule_snapshots、daily_metrics |
| 隐私/文件/集成 | public_visibility、attachments、integration_accounts |

领域 id 为 UUID TEXT，Rust API 验证规范 UUID。共同 created_at/updated_at 为 RFC3339 UTC `...Z`；archived_at/deleted_at 可空，SQL 检查 UTC 后缀。date-only 独立 TEXT；Task/Journal API 用 NaiveDate 校验真实 YYYY-MM-DD，不进行时区转换。其他领域写入服务开放时同样增加验证。history.version 为内部编号，非业务外部 ID。

外键、唯一约束、状态 CHECK、deleted_at/updated_at 索引及任务截止、日记日期、关联/哈希索引。public_visibility 默认 private；凭证只保存 credential_ref。没有个人 seed。后续领域扩展必须新增 migration。

## Repository / IPC

ContentRepository(Task|Journal) 提供 create/get/update/set_deleted。get 默认不读回收站，恢复清空 deleted_at，更新已删除记录拒绝。所有值 bind，只有编译期枚举控制表/列名。单语句和 FTS 触发器原子执行，软删除/恢复显式事务。无永久删除 API。

FTS5 trigram 字面短语搜索支持中文子串，至少 3 Unicode 字符、最多 200 字节/50 条。双引号转义并 bind，不能执行用户 FTS 运算符。触发器同步 Task/Journal 更新、软删除/恢复；已删除记录不入索引。其他领域搜索及短词 fallback 后续实现。

database_status 只返回 schema_version、integrity、storage 标签、Task/Journal 数量，不返回绝对路径、SQL 或正文。存储失败仍可打开窗口查看诊断，绝不自动重置。外观仍沿用 Phase 2 localStorage，编辑入口仅设置。

## 迁移

1. 获取 migration.lock 排他锁，连接 SQLite。
2. 验证历史连续版本、名称及原始 SQL SHA256。未知版本、checksum 漂移或已有其他表但无 history 的数据库停止。
3. 已有 schema 的每次升级先创建并验证备份；空库首次迁移无需备份。
4. 事务执行 SQL 与 history INSERT；失败显式 rollback，保留备份。多个 migration 逐个事务，先前成功的不会回滚。
5. 启动和诊断执行 quick_check。迁移文件固定 LF，发布后不可修改。

## 备份与恢复边界

骨架包含一致数据库及其中 settings、版本、数据库 SHA256、附件 manifest（id/path/size/checksum）。使用流式哈希；DB 与 manifest flush 后验证。附件清单从备份 DB 读取。

verify 只读校验 SHA256、integrity_check、foreign_key_check、历史/checksum、所有领域表共同列、Task/Journal/settings 真实行可读和附件清单一致。错误或缺文件均拒绝。恢复测试复制到新 TempDir，重开并比对实际 fixture 内容；没有覆盖生产库的 API。

尚未复制附件 payload、实现 daily/weekly/monthly 调度保留或用户确认恢复 UI，均属于 Phase 13；不能把本骨架称为完整附件恢复包。

## 错误

序列化 `{code,message}`，静态消息不含用户内容、SQL、路径或原始异常。错误码：INVALID_INPUT、NOT_FOUND、CONSTRAINT_VIOLATION、DATABASE_BUSY、DATABASE_CORRUPT、DATABASE_ERROR、STORAGE_UNAVAILABLE、MIGRATION_MISMATCH、MIGRATION_FAILED、BACKUP_INVALID。读取失败不得显示 ready，IO 失败不得视为备份有效。

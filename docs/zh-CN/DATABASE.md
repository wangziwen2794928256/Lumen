# DATABASE

唯一core数据库仍为AppData/lumen.db：不复制或移动旧库。core/目录预留给未来明确迁移，当前为空，不能产生第二份权威数据库。只有Kernel及可信内置服务持有SQLx，社区没有连接、路径、SQL API。

008只新增module_preferences和planner_observations及触发器；001–007不改。模块配置整包revision事务更新。观察记录只从迁移后真实变更产生，不回填虚假历史；任务估时/日期/状态、专注实际/计划、规划accept/reject/修改/撤销、容量与规划偏好被保留。无正文/标题。

迁移前一致备份、checksum history、事务rollback和损坏拒绝延续现有机制。core WAL/SHM一同由SQLite管理，禁止把运行中的db直接复制当备份。

本文件对应英文版本：[DATABASE](../en/DATABASE.md)。

Phase12 保持 schema10，001–010 不变。settings 的 profile.v1 保存默认私密的版本化 Profile，revision 防并发覆盖；仅白名单显式查询公开作品，不序列化任意数据库行，无新 migration。

Phase13 retains schema10 and immutable migrations001-010. backup.policy.v1 uses existing settings. Portable exchange uses a versioned fixed 24-table allowlist with bound values, dry-run rollback and pre-import complete backup; derived focus metrics rebuild from intervals. No production schema changes. See BACKUP_PRIVACY.md.

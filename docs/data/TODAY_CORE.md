# Today 数据与操作契约 · Phase 04

状态：已实施。Schema 2，新增 002_today.sql；001_initial.sql 保持原样。业务数据仅在 Tauri AppData，浏览器不模拟生产业务存储。

## 文件职责

| 路径 | 职责 |
|---|---|
| src/features/today/model.ts | Zod 输入、DTO、独立本地日期 |
| src/features/today/TaskForm.tsx | 创建/编辑、字段校验、失败保留草稿与请求 ID |
| src/features/today/api.ts | 仅白名单 today_snapshot / today_action |
| src/features/today/useToday.ts | 防重复提交、丢弃过期响应、读取后端真值 |
| src/pages/today/TodayPage.tsx | Now/Next/Remember、容量、回收站、实际时间线 |
| src-tauri/src/today/mod.rs | 参数化 repository、串行操作、事务与 Rust 时间源 |
| src-tauri/src/today/tests.rs | TempDir 数据与时间源集成验证 |
| src-tauri/migrations/002_today.sql | 增量字段、约束、单个未结束会话、请求回执 |

## 输入和任务

UI 标题 trim 后 1–250 字符、备注最多 100000 字符、预计时长为 1–1440 整数分钟、优先级 0–3、日期为真实 ISO date 或 null。Rust 再校验 UUID、日期、时长、优先级及正文大小；禁止绕过后端约束。操作错误只返回静态 code/message，不输出 SQL 或私人正文。

任务状态 planned / active / done / skipped / moved / cancelled。active 仅开始会话时设置；其它状态通过 task_state 设置。移走作为生命周期标记，改到具体日期通过编辑表单完成。Today 显示到期/逾期/无日期的待办、进行中任务、今日处理的任务；未来任务在日期到达后进入 Today。历史已完成任务不累加今日计划。

编辑、状态、软删除、恢复使用 revision 乐观并发控制。正在进行或等待恢复的会话关联任务不能编辑、移入回收站或手动完成。回收站只软删除，最多显示最近 100 项。Journey 显示 Now 任务实际关联的未删除目标；无关联时明确空状态，目标编辑待 Phase 6。

## 提交与幂等

create / start / capture 的 UUID 请求 ID 和序列化输入 SHA256 与写入处于同一 BEGIN IMMEDIATE 事务。相同 ID + 相同内容重试不会重复写入；相同 ID + 不同内容返回 STATE_CONFLICT。表单和随手记遇到失败保持草稿，重试沿用 ID；新一次操作创建新 ID。finish / cancel 对同一终态重复调用无新增指标。revision 操作的过期重试返回冲突，不默默覆盖记录。

同一 AppData 的 TodayService 生命周期持有文件独占锁，多个进程只有一个 Today 写入者。第二个窗口显示 DATABASE_BUSY；它不能累加另一窗口的计时。进程结束自动释放锁，重开恢复记录。

## 最小计时状态机

| 当前状态 | 允许操作 | 结果 |
|---|---|---|
| running | pause / interrupt / finish / cancel / resume | 对应状态；重复 resume 保持 running |
| paused | resume / interrupt / finish / cancel / pause | 对应状态；重复 pause 保持 paused |
| interrupted | resume / finish / cancel / interrupt | 恢复、最后活动结束、取消或保持 |
| finished | finish | 幂等；其它操作冲突 |
| cancelled | cancel | 幂等；其它操作冲突 |

Rust Instant 累加确认的 elapsed_ms / paused_ms / interrupted_ms；UTC 时间保存启动、最后活动、终止与检查点，session_date 单独保存开始时本地日期。页面每秒读取，不自行 setInterval 累加时间。后台每 5 秒保存，不依赖页面是否可见。

时间差超过 15 秒、墙钟倒退、单调/墙钟差超过 3 秒时转为 interrupted，不把异常间隔计入专注。重启将未结束会话恢复为 interrupted，离线差只归入中断时长。继续从已保存时长开始；最后活动结束保留 last_activity_at；取消保留审计会话但不产生每日专注指标。

正常环境异常退出最多损失一个后台检查点周期的未确认时长；存储故障或调度延迟时不能承诺精确 5 秒。真实锁屏/电源事件及短于阈值的休眠检测尚未接入，Phase 5 必须补充原生验证。系统墙钟人为调整后的中断时长是估计值，不能作为精准工时。

## 指标与快速记录

finish 在同一事务更新会话 actual_seconds=floor(elapsed_ms/1000)、任务完成及 daily_metrics。当日 focus_seconds 由 session_date 匹配的 finished 会话重新求和，取消和手动完成不虚增时长。时间线读取实际会话。跨午夜目前归属开始日，分钟级跨日分段与日历 overlay 待 Phase 5。

每日容量为 settings 中 0–1440 分钟，默认参考 210；计划按今日可见任务预计分钟汇总，跳过/移走/取消不纳入。容量超出只提示可调整，不惩罚。随手记写 journal_entries 的本地日期和正文，显示今日最近三条；全文日记待 Phase 7。

## 验证边界

Rust 测试使用 TempDir 和私有 fixture clock；浏览器 E2E 的 IPC fixture 只位于 tests，不进入产品。真实桌面 CRUD/暂停/异常退出/结束使用独立标识 com.lumen.qa.phase4，fixture AppData 不随源码或可执行文件交付。正式标识 com.lumen.desktop 的数据库诊断确认 schema 2、Task/Journal 0。

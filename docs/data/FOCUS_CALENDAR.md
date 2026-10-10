# Focus / Calendar 数据契约 · Phase 05

Schema 3：只新增 003_focus_calendar.sql；001/002 的 SHA256 不变。升级由现有 migration history/checksum/备份/事务框架执行。生产和 QA 均使用 Tauri app_data_dir，QA 标识 com.lumen.qa.phase5 的数据库不随交付物发布。

## 文件与边界

| 文件 | 职责 |
|---|---|
| today/mod.rs | 单写入锁、串行状态机、计时检查点与事务 |
| today/intervals.rs | 单调时间分配、本地午夜分段、实际指标重建 |
| today/native_events.rs | Windows 同会话通知、休眠通知、UI 线程 subclass 生命周期 |
| today/calendar.rs | 日历区间查询、事件校验、revision、回执、软删除 |
| features/focus / pages/focus | 专注选择、实际/计划/暂停/中断、显式恢复 |
| features/calendar / pages/calendar | 类型/日期计算、轮询、事件表单、四视图 |
| phase5_tests.rs / database/tests.rs | TempDir 状态、跨日、迁移、旧记录精度测试 |

## 计时状态与系统中断

沿用 Phase 4 状态矩阵：running→pause/resume/interrupt/finish/cancel；paused→pause/resume/interrupt/finish/cancel；interrupted→resume/interrupt/finish/cancel；finished 仅 finish 幂等；cancelled 仅 cancel 幂等。任意时刻最多一个未结束会话。恢复操作必须明确选择，解除系统锁定不会自动继续。

后台每 5 秒以及页面读取/操作前检查点。Rust Instant 是累积时长真值；UTC 标记是时间位置，不能以浏览器后台 setInterval 或离线墙钟替代专注。系统事件在 Windows UI 线程同步锁存 mask/epoch，再触发异步落盘。锁屏和休眠分别占位，全部解除后才允许 start/resume；短时间锁屏后立即解锁仍保留 interrupted。

WM_WTSSESSION_CHANGE 的 lock/disconnect 与 unlock/connect，WM_POWERBROADCAST 的 suspend/resume 映射到锁存。SetWindowSubclass 只在所属 UI 线程安装；WM_NCDESTROY 注销 WTS、移除 subclass、释放 Box<Arc>。监听安装失败显示静态错误并关闭专注写入服务，不宣称保护已就绪。

异常时间差超过 15 秒、墙钟倒退、两种时间源差超过 3 秒或后台写入失败恢复时进入 interrupted，丢弃未确认间隔。系统通知到达时，最后检查点之后的片段也保守丢弃：通常最多约 5 秒；磁盘/线程阻塞时无法保证该上限。中断累计时长对系统时间调整只能估计，不能用作精确工时。

## 实际片段与跨日指标

focus_intervals 主键为 session_id + segment_no + local_date；暂停/中断后 resume 递增 segment_no。正常检查点在同一事务追加片段并累加 elapsed_ms。跨本地午夜按墙钟位置分配单调毫秒总量，最后一段承接舍入余数。暂停和中断时间不会生成片段，取消会话的片段留作审计但不进入日历实际或完成指标。

finish 同时更新任务完成、会话实际总秒数及各涉及日期 focus_seconds。日期指标由该日期 finished 的片段毫秒总和整除 1000；会话总秒和各日秒的和在跨日舍入时可能相差 1 秒，毫秒总量严格守恒。重复 finish 不新增指标。

旧版本 elapsed_ms>0 的记录以 segment_no=0、provenance=legacy_total 保留，时长不变；旧总量没有暂停边界，因此日历只显示“历史汇总（含间隔）”明细，不把覆盖范围冒充准确的连续专注条。已存在每日指标升级时不重写历史归属。

## 日历输入和读取

calendar_snapshot(from,until) 使用本地 ISO 日期半开区间，最多 62 天；定时事件以 UTC RFC3339 保存，全天事件使用单独 event_date。定时开始/结束必须同时存在、严格递增、最长 7 天，且不同时填写全天日期。事件类型为 study/class/competition/exam/deadline/paper/personal；实际专注来源是只读 focus_intervals，不允许以编辑计划事件伪造实际。

所有输入 Rust 二次校验，SQL 参数化；创建 UUID + 内容回执防重复；编辑和软删除/恢复使用 revision。已关联任务必须存在且未删除。下拉展示最多 200 个待办任务，回收站展示最近 100 个事件。日历任务按 due_date 显示全天参考，不补造时间。

日/周使用全天数量及 24 小时时间位置，计划在左侧虚线淡色，实际在右侧实色；重叠或短片段的完整文字在下方明细保留。月视图是周一开始的 42 格，前三项摘要与更多入口，下方展示选中日完整明细。议程为当前日期起 14 天。计划/实际显示可独立开关，状态不只靠颜色识别。长标题在完整明细中换行，时间网格短片段通过 title/ARIA 提供完整信息。

## 验证边界

Rust 的真实 hidden HWND + WTS 注册 + SendMessage subclass 分发/销毁测试属于 Windows 消息重放；TempDir clock 测试属于隔离模拟。两者均不等同于真实物理锁屏/休眠。浏览器 E2E IPC fixture 仅在 tests，产品在普通浏览器显示空状态。物理操作需要用户解锁/唤醒，实际结果单独记录在 NATIVE_QA，未进行时保持 NOT RUN。

官方接口依据：[Windows 电源广播](https://learn.microsoft.com/en-us/windows/win32/power/wm-powerbroadcast)、[SetWindowSubclass](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)、[WTS 通知](https://learn.microsoft.com/en-us/windows/win32/api/wtsapi32/nf-wtsapi32-wtsregistersessionnotification)。

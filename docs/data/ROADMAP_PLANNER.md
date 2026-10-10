# Roadmap / Planner 数据契约

## 版本与写入

004 只追加字段与表，001–003 不变。Journey/Phase/Subject/Quest 扩展使用 typed planning_data JSON 与 revision；核心 title/关系及软删除仍沿用已有表。输入拒绝未知字段，SQL 标识符仅来自四种 kind 白名单，值全部参数绑定。Roadmap 与计时共用 TodayService writer 锁、clock Mutex 和 BEGIN IMMEDIATE，跨模块写入串行。

## 实体与模板

最低 ≤ 偏好 ≤ 上限，单位为每周分钟，范围 0–10080。优先级 0–3、进度 0–100，日期严格本地 date-only。parent_id 验证不存在循环；关联必须指向仍可用记录。父目标删除/归档不会硬删除子项，规划时排除不可用父关联，恢复保留子项原状态。预算按任务直接关联的 Journey/Subject/Quest 独立计算；父子层级用于组织，不自动重复叠加预算。

Fudan 2028 是主动选择创建的实例模板：8 阶段、8 学科、408 四子项，共 21 条实体；不自动创建任务、分数或政策快照。日期为可修改的学习安排。400 是 planning target。任务比例进度从未删除且未跳过/取消/移走的任务计算；阶段只提供手动进度。

## 容量与约束

首次工作日和周末均沿用既有 daily_capacity_minutes（未配置时 210 分钟），避免升级静默提高预算；可以在设置分别修改。每周 30–32 小时仅为可选参考。每天覆盖值 0–1440，并保存 Normal/Rest/Exam/Competition/Travel/Sick/Paper Sprint/Other。Today 容量与日期覆盖共用数据；默认值不是硬性完成指标。

固定日历事件按本地日期切分，重叠时段合并，按分钟向上取整并从该日容量扣除；全天事件保守占用该日全部容量。过去日期 available=0，避免追加历史安排。fixed_time 任务不自动安排。day.available=max(0,capacity-fixed)。不会扩大容量或改变固定事件。

任务 earliest_start、deadline、remaining、minimum/preferred session、splittable、energy 保存于 typed options。精力标签供人工调整参考，不推断用户精力；preferred session 是软偏好。不可拆任务必须在一个日期全部放下；不足则明确保留未安排。最后一段可小于 minimum。未完成任务按剩余估时规划，不默默延期。

## 建议、守恒与审计

先循环保留可满足的最低分配，再偏好，再按 deadline/slack/priority 填充剩余。Journey/Subject/Quest 上限及全局 side quest 上限同时约束，冲突时保留未安排量与说明，不声称全局最优。

Propose 只写 proposed 审计；Accept/Modify 校验依据 hash（当前日期、实体/任务/容量/固定占用/原安排）防止 stale。每天总安排≤available，每任务总安排≤remaining，拆分与上限必须满足。Modify 不得压缩原方案已可满足的最低分配。审计保存 before/after/reason/created/updated，不静默修改任务 deadline 或总预算。

planning_blocks 是派生安排，事务替换本周，不硬删任务。Undo 只允许当前最新已接受版本，比较当前 blocks 和 after，恢复 before 并按现行约束重新验证；冲突不覆盖用户后续修改。重复终态操作幂等；proposal/template/create 使用 UUID 与 payload hash 防重试重复。

接受后的安排在 Roadmap 可见，Today 纳入本日已分配任务并按分配分钟计算计划量。Calendar 保留原任务截止日期与固定事件语义，不把按日分钟预算伪装成精确时段。容量变化后原已接受安排保留，显示超额提示并要求重新生成，绝不静默增加时间。

## 年度资料

手动保存 school/college/major/year/written/interview/source_url/body/notes；captured_at 为本机保存 UTC，content_hash=SHA256(UTF-8 body)。源网址白名单 gsao.fudan.edu.cn、gsas.fudan.edu.cn、www.fudan.edu.cn、yz.chsi.com.cn HTTPS。哈希验证用户保存文本，不伪称整份网站原文已自动下载或认证；source_file 留给后续附件能力。新年度新建版本，不复用某年政策为永久规则。资料软删除可恢复，无预测分数或录取概率。

官方来源核对（2026-10-03）：[复旦研究生招生目录](https://gsao.fudan.edu.cn/15015/list.htm)列出 2027 年目录与章程；[年度专业查询](https://gsas.fudan.edu.cn/)为官方链接目标。此核对不将其当作 2028 年规则，也不注入生产数据库。

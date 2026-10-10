# PLAN_ENGINE

Manual First：现有预算守恒Planner与Accept/Modify/Reject/Undo继续负责真实决策。外部LLM或Local AI未来只生成建议，必须经Kernel验证basis_hash、task revision、逐日容量、冲突与用户明确接受后落库。

Plan Context 1.0只允许明确用户选择的任务元数据、估时/实际数值、容量和basis；不默认发送日记、附件、音乐、正文或插件数据。Plan Exchange 1.0含request_id/source/basis_hash/capacity/items，UUID、日期、分钟、数量和未知字段严格校验；没有执行/apply IPC。

008观察表保留后续估时与日期调整、状态、真实专注、decision choices和capacity，旧的planning_decisions before/after/basis与focus_intervals继续权威。记录原始事实，不生成预测分或训练样本标签。External/Local AI都未调用。

本文件对应英文版本：[PLAN_ENGINE](../en/PLAN_ENGINE.md)。

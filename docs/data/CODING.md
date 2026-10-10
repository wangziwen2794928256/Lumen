# Coding 数据字典与分析定义

Phase 8；本地手动录入，无爬虫、账户连接或自动上传。源码契约为 coding/model.rs 与 features/coding/model.ts。

## 题目

coding_problems：id UUID、title、platform（Luogu/NowCoder/Codeforces/LeetCode/AtCoder/Other）、external_id 平台题号、source_url http(s)、difficulty（unknown/easy/medium/hard）、tags_json 去重标签、notes、journey_id、task_id、competition_id、revision、时间戳及软删除字段。

活动记录按 platform + trim(external_id) 不区分题号大小写唯一。相同题目增加 attempt；重复录入返回 CODING_DUPLICATE，输入保留。删除后的相同题号可新增；恢复旧题时若已有相同活动题目则拒绝覆盖。已有空题号的旧记录保留，但再次编辑须填写题号。

状态由未删除 attempts 推导：无尝试为未尝试，有尝试无 accepted 为未解出，任一 accepted 为已解出。attempt_count、first_attempt、solved_at、最新 language 与总用时是查询结果，不维护易漂移缓存。首尝试/首次通过时间分别取有效尝试/accepted 的最早 UTC started_at。

## 尝试与关联

coding_attempts：problem_id、language、outcome（accepted/wrong_answer/time_limit/runtime_error/compile_error/unfinished）、notes、attempted_on 本地日期、started_at UTC RFC3339、manual_seconds 额外手工用时、focus_session_id、revision 和软删除时间。日期与 UTC 时间相差至多一天以容纳本地时区。

只能关联已完成 focus；读取其 actual_seconds，不修改 Focus/Calendar/指标账本。一个 focus_session_id 只能被一条未删除尝试占用，包括父题目暂在回收站时。有关联任务的题目要求 focus.task_id 相同；修改题目的任务时保护既有 focus 关联。手工秒数为专注以外的额外用时；两项分别展示。恢复尝试检查父题目有效与 focus 未占用。

Journey、Task、Competition 写入时须存在且未删除/归档。实体后续删除不级联删除个人题目或尝试。

## 竞赛

competitions：title、platform、event_date、本机 notes、source_url、participated、revision 与软删除时间。参赛标志为用户手工记录，未从题目关联推断参赛，也不推断名次/成绩。支持创建、修改、软删除、恢复。

## 周统计

选定日期归一到周一，区间周一至周日，按 attempted_on 分组。只统计活动题目下未删除且有尝试日期的 attempts；旧版无日期记录不伪造时间，须编辑补充后进入周统计。

| 指标                  | 定义                                                                                                                                |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Problems/week         | 本周至少一次有效尝试的不同 problem_id 数；多次尝试不多计题数                                                                        |
| Solved/week           | 全部有效历史中第一次 accepted 的本地日期落在本周的不同题目数                                                                        |
| Attempts              | 本周有效尝试总数                                                                                                                    |
| First-pass rate       | 分母为第一次有效尝试日期落在本周的题目；分子为其首次尝试 accepted。按 attempted_on、started_at、id 排序；跨周开始题目不纳入本周分母 |
| Difficulty            | 本周练习的不同题目，按人工难度分类；unknown 单列                                                                                    |
| Topic                 | 本周练习的不同题目按去重标签分类；一道题多个主题，因此总和可超过题数                                                                |
| Time/problem          | 本周有效尝试 manual_seconds + 唯一 focus actual_seconds，除以本周不同题目数；用时归属尝试日期，跨日专注不再次拆分该 Coding 分类统计 |
| Contest participation | 未删除、participated=true 且 event_date 在本周的竞赛数                                                                              |

无分母显示“暂无样本”，不显示 NaN、Infinity 或 0% 假样本；计数/总秒数零值仍显示零。软删除或修改会重新计算历史统计，未冻结为永久成功次数。

## 保存、查询与边界

coding_action 在 BEGIN IMMEDIATE 内验证、revision 更新及 coding_receipts(request_id,payload_hash,created_at) 回执写入。相同请求相同内容幂等；复用请求 id 改内容则拒绝。客户端未提交输入按 type/id 独立保存本地，包含 request_id，失败/重启可重试；仅明确保存 ACK 后清理对应稿。保存期间禁用字段，避免提交后编辑丢失。

题目 30 条分页；题目查询支持名称/题号字面量、平台、状态、精确主题和回收站。尝试面板最近 200 条；关联选项/竞赛当前最多 500 条，周统计不按这些显示上限截断。所有用户 SQL 参数绑定；source_url 禁止 javascript/file/data、用户名密码。笔记以纯文本展示，不执行代码。

数据库仅追加 006，不修改 001–005，升级前备份，无种子数据。私人笔记与数据库不进 Git、日志或交付包。安装包、签名、爬虫和竞赛平台 OAuth 不在本阶段。

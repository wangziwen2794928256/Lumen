# PLAN_ENGINE

Manual First preserves the existing budget-conserving planner and Accept/Modify/Reject/Undo decisions. Future external/local AI only proposes; Kernel must validate the live basis hash, task revisions, per-day capacity, conflicts and explicit user acceptance before writes.

Plan Context 1.0 allows deliberately selected task metadata, estimate/actual values, capacity and basis. It does not automatically include journals, attachments, music, bodies or plugin data. Plan Exchange 1.0 bounds UUIDs, dates, minutes, item counts and unknown fields; no apply/execute IPC exists.

Migration 008 preserves future changes in estimates/dates/states, focus actuals, decision choices and capacity. Existing decision before/after/basis and focus intervals remain authoritative. Observations contain facts, not predicted scores or training labels. Neither external nor local AI is called.

Matching Chinese document：[PLAN_ENGINE](../zh-CN/PLAN_ENGINE.md)。

-- Append-only Phase 4 schema. Migration 001 remains unchanged.
ALTER TABLE tasks ADD COLUMN lifecycle TEXT NOT NULL DEFAULT 'planned' CHECK(lifecycle IN ('planned','active','done','skipped','moved','cancelled'));
ALTER TABLE tasks ADD COLUMN estimated_minutes INTEGER NOT NULL DEFAULT 25 CHECK(estimated_minutes BETWEEN 1 AND 1440);
ALTER TABLE tasks ADD COLUMN remaining_minutes INTEGER NOT NULL DEFAULT 25 CHECK(remaining_minutes BETWEEN 0 AND 1440);
ALTER TABLE tasks ADD COLUMN priority INTEGER NOT NULL DEFAULT 1 CHECK(priority BETWEEN 0 AND 3);
ALTER TABLE tasks ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK(revision >= 0);
UPDATE tasks SET lifecycle=CASE status WHEN 'done' THEN 'done' WHEN 'doing' THEN 'active' ELSE 'planned' END;
ALTER TABLE focus_sessions ADD COLUMN state TEXT NOT NULL DEFAULT 'finished' CHECK(state IN ('running','paused','interrupted','finished','cancelled'));
ALTER TABLE focus_sessions ADD COLUMN elapsed_ms INTEGER NOT NULL DEFAULT 0 CHECK(elapsed_ms >= 0);
ALTER TABLE focus_sessions ADD COLUMN paused_ms INTEGER NOT NULL DEFAULT 0 CHECK(paused_ms >= 0);
ALTER TABLE focus_sessions ADD COLUMN interrupted_ms INTEGER NOT NULL DEFAULT 0 CHECK(interrupted_ms >= 0);
ALTER TABLE focus_sessions ADD COLUMN last_activity_at TEXT;
ALTER TABLE focus_sessions ADD COLUMN session_date TEXT;
UPDATE focus_sessions SET elapsed_ms=COALESCE(actual_seconds,0)*1000,last_activity_at=COALESCE(ended_at,started_at),session_date=substr(started_at,1,10);
CREATE UNIQUE INDEX idx_one_unfinished_focus ON focus_sessions((1)) WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL;
CREATE INDEX idx_focus_daily_actual ON focus_sessions(session_date,state);
CREATE INDEX idx_tasks_today ON tasks(lifecycle,due_date,priority) WHERE deleted_at IS NULL;
CREATE TABLE request_receipts (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  payload_sha256 TEXT NOT NULL CHECK(length(payload_sha256)=64),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z')
);

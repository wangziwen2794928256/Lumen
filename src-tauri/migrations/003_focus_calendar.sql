-- Published migrations 001/002 are immutable. Phase 5 only appends schema.
ALTER TABLE focus_sessions ADD COLUMN segment_no INTEGER NOT NULL DEFAULT 1 CHECK(segment_no>=1);
ALTER TABLE focus_sessions ADD COLUMN interruption_reason TEXT CHECK(interruption_reason IS NULL OR interruption_reason IN ('manual','restart','timestamp_gap','screen_lock','sleep'));
CREATE TABLE focus_intervals (
  session_id TEXT NOT NULL REFERENCES focus_sessions(id),
  segment_no INTEGER NOT NULL CHECK(segment_no>=0),
  local_date TEXT NOT NULL CHECK(length(local_date)=10),
  started_at TEXT NOT NULL CHECK(substr(started_at,-1)='Z'),
  ended_at TEXT NOT NULL CHECK(substr(ended_at,-1)='Z'),
  elapsed_ms INTEGER NOT NULL CHECK(elapsed_ms>=0),
  provenance TEXT NOT NULL DEFAULT 'observed' CHECK(provenance IN ('observed','legacy_total')),
  PRIMARY KEY(session_id,segment_no,local_date)
);
-- Preserve existing accumulated duration without inventing precise historical segments.
INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms,provenance)
SELECT id,0,session_date,started_at,COALESCE(last_activity_at,ended_at,started_at),elapsed_ms,'legacy_total'
FROM focus_sessions WHERE elapsed_ms>0 AND session_date IS NOT NULL;
CREATE INDEX idx_focus_intervals_date ON focus_intervals(local_date,started_at);
ALTER TABLE calendar_events ADD COLUMN event_type TEXT NOT NULL DEFAULT 'personal' CHECK(event_type IN ('study','class','competition','exam','deadline','paper','personal'));
ALTER TABLE calendar_events ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK(revision>=0);
CREATE INDEX idx_calendar_dates ON calendar_events(event_date,starts_at) WHERE deleted_at IS NULL;

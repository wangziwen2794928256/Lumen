-- 001_initial.sql: immutable after checkpoint/03-database. No personal seed records.

CREATE TABLE users (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  display_name TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_users_live ON users(deleted_at, updated_at);

CREATE TABLE settings (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  key TEXT NOT NULL UNIQUE, value_text TEXT NOT NULL,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_settings_live ON settings(deleted_at, updated_at);

CREATE TABLE journeys (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '', target_date TEXT,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_journeys_live ON journeys(deleted_at, updated_at);

CREATE TABLE journey_phases (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  journey_id TEXT NOT NULL REFERENCES journeys(id), title TEXT NOT NULL, position INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_journey_phases_live ON journey_phases(deleted_at, updated_at);

CREATE TABLE subjects (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  journey_id TEXT REFERENCES journeys(id), title TEXT NOT NULL,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_subjects_live ON subjects(deleted_at, updated_at);

CREATE TABLE tasks (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL CHECK(length(trim(title)) > 0), body TEXT NOT NULL DEFAULT '', status TEXT NOT NULL DEFAULT 'todo' CHECK(status IN ('todo','doing','done')), due_date TEXT CHECK(due_date IS NULL OR due_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'), journey_id TEXT REFERENCES journeys(id), subject_id TEXT REFERENCES subjects(id),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_tasks_live ON tasks(deleted_at, updated_at);

CREATE TABLE task_sessions (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  task_id TEXT NOT NULL REFERENCES tasks(id), started_at TEXT NOT NULL CHECK(substr(started_at,-1)='Z'), ended_at TEXT, duration_seconds INTEGER CHECK(duration_seconds >= 0),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_task_sessions_live ON task_sessions(deleted_at, updated_at);

CREATE TABLE calendar_events (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, starts_at TEXT, ends_at TEXT, event_date TEXT, task_id TEXT REFERENCES tasks(id),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_calendar_events_live ON calendar_events(deleted_at, updated_at);

CREATE TABLE focus_sessions (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  task_id TEXT REFERENCES tasks(id), started_at TEXT NOT NULL CHECK(substr(started_at,-1)='Z'), ended_at TEXT, planned_seconds INTEGER NOT NULL CHECK(planned_seconds > 0), actual_seconds INTEGER CHECK(actual_seconds >= 0),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_focus_sessions_live ON focus_sessions(deleted_at, updated_at);

CREATE TABLE quests (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '', journey_id TEXT REFERENCES journeys(id),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_quests_live ON quests(deleted_at, updated_at);

CREATE TABLE competitions (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, event_date TEXT, source_url TEXT,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_competitions_live ON competitions(deleted_at, updated_at);

CREATE TABLE coding_problems (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, source_url TEXT, difficulty TEXT,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_coding_problems_live ON coding_problems(deleted_at, updated_at);

CREATE TABLE coding_attempts (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  problem_id TEXT NOT NULL REFERENCES coding_problems(id), language TEXT, outcome TEXT, notes TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_coding_attempts_live ON coding_attempts(deleted_at, updated_at);

CREATE TABLE journal_entries (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL CHECK(length(trim(title)) > 0), body TEXT NOT NULL DEFAULT '', entry_date TEXT NOT NULL CHECK(entry_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_journal_entries_live ON journal_entries(deleted_at, updated_at);

CREATE TABLE journal_tags (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  entry_id TEXT NOT NULL REFERENCES journal_entries(id), name TEXT NOT NULL,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z'),
  UNIQUE(entry_id,name)
);

CREATE INDEX idx_journal_tags_live ON journal_tags(deleted_at, updated_at);

CREATE TABLE music_tracks (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, artist TEXT NOT NULL DEFAULT '', source_uri TEXT,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_music_tracks_live ON music_tracks(deleted_at, updated_at);

CREATE TABLE music_plays (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  track_id TEXT NOT NULL REFERENCES music_tracks(id), played_at TEXT NOT NULL CHECK(substr(played_at,-1)='Z'), duration_seconds INTEGER CHECK(duration_seconds >= 0),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_music_plays_live ON music_plays(deleted_at, updated_at);

CREATE TABLE artifacts (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '', kind TEXT NOT NULL DEFAULT 'note',
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_artifacts_live ON artifacts(deleted_at, updated_at);

CREATE TABLE artifact_files (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  artifact_id TEXT NOT NULL REFERENCES artifacts(id), attachment_id TEXT NOT NULL REFERENCES attachments(id),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_artifact_files_live ON artifact_files(deleted_at, updated_at);

CREATE TABLE weekly_reviews (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  week_start_date TEXT NOT NULL UNIQUE, body TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_weekly_reviews_live ON weekly_reviews(deleted_at, updated_at);

CREATE TABLE monthly_snapshots (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  month TEXT NOT NULL UNIQUE, summary TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_monthly_snapshots_live ON monthly_snapshots(deleted_at, updated_at);

CREATE TABLE exam_rule_snapshots (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  journey_id TEXT REFERENCES journeys(id), source_url TEXT, captured_at TEXT NOT NULL CHECK(substr(captured_at,-1)='Z'), body TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_exam_rule_snapshots_live ON exam_rule_snapshots(deleted_at, updated_at);

CREATE TABLE daily_metrics (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  metric_date TEXT NOT NULL, metric_key TEXT NOT NULL, value REAL NOT NULL,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z'),
  UNIQUE(metric_date,metric_key)
);

CREATE INDEX idx_daily_metrics_live ON daily_metrics(deleted_at, updated_at);

CREATE TABLE public_visibility (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  entity_type TEXT NOT NULL, entity_id TEXT NOT NULL, visibility TEXT NOT NULL DEFAULT 'private' CHECK(visibility IN ('private','public')),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z'),
  UNIQUE(entity_type,entity_id)
);

CREATE INDEX idx_public_visibility_live ON public_visibility(deleted_at, updated_at);

CREATE TABLE attachments (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  relative_path TEXT NOT NULL UNIQUE, mime_type TEXT NOT NULL, size_bytes INTEGER NOT NULL CHECK(size_bytes >= 0), sha256 TEXT NOT NULL CHECK(length(sha256)=64),
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z')
);

CREATE INDEX idx_attachments_live ON attachments(deleted_at, updated_at);

CREATE TABLE integration_accounts (
  id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
  provider TEXT NOT NULL, display_name TEXT NOT NULL DEFAULT '', credential_ref TEXT,
  created_at TEXT NOT NULL CHECK(substr(created_at,-1)='Z'),
  updated_at TEXT NOT NULL CHECK(substr(updated_at,-1)='Z'),
  archived_at TEXT CHECK(archived_at IS NULL OR substr(archived_at,-1)='Z'),
  deleted_at TEXT CHECK(deleted_at IS NULL OR substr(deleted_at,-1)='Z'),
  UNIQUE(provider,display_name)
);

CREATE INDEX idx_integration_accounts_live ON integration_accounts(deleted_at, updated_at);

CREATE INDEX idx_tasks_due ON tasks(due_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_tasks_journey ON tasks(journey_id);
CREATE INDEX idx_journal_date ON journal_entries(entry_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_journal_tags_entry ON journal_tags(entry_id);
CREATE INDEX idx_focus_task ON focus_sessions(task_id, started_at);
CREATE INDEX idx_attachments_sha ON attachments(sha256);
CREATE TABLE migration_history (
  version INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  checksum TEXT NOT NULL CHECK(length(checksum)=64),
  applied_at TEXT NOT NULL CHECK(substr(applied_at,-1)='Z')
);
CREATE VIRTUAL TABLE content_search USING fts5(kind UNINDEXED, entity_id UNINDEXED, title, body, tokenize='trigram');

CREATE TRIGGER tasks_fts_insert AFTER INSERT ON tasks WHEN new.deleted_at IS NULL BEGIN INSERT INTO content_search(kind,entity_id,title,body) VALUES('task',new.id,new.title,new.body); END;
CREATE TRIGGER tasks_fts_update AFTER UPDATE ON tasks BEGIN DELETE FROM content_search WHERE kind='task' AND entity_id=old.id; INSERT INTO content_search(kind,entity_id,title,body) SELECT 'task',new.id,new.title,new.body WHERE new.deleted_at IS NULL; END;
CREATE TRIGGER tasks_fts_delete AFTER DELETE ON tasks BEGIN DELETE FROM content_search WHERE kind='task' AND entity_id=old.id; END;

CREATE TRIGGER journal_entries_fts_insert AFTER INSERT ON journal_entries WHEN new.deleted_at IS NULL BEGIN INSERT INTO content_search(kind,entity_id,title,body) VALUES('journal',new.id,new.title,new.body); END;
CREATE TRIGGER journal_entries_fts_update AFTER UPDATE ON journal_entries BEGIN DELETE FROM content_search WHERE kind='journal' AND entity_id=old.id; INSERT INTO content_search(kind,entity_id,title,body) SELECT 'journal',new.id,new.title,new.body WHERE new.deleted_at IS NULL; END;
CREATE TRIGGER journal_entries_fts_delete AFTER DELETE ON journal_entries BEGIN DELETE FROM content_search WHERE kind='journal' AND entity_id=old.id; END;

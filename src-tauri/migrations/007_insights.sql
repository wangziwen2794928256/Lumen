-- Append-only Phase 9. No personal seed data or changes to planning capacity.
ALTER TABLE weekly_reviews ADD COLUMN revision INTEGER NOT NULL DEFAULT 0;
ALTER TABLE weekly_reviews ADD COLUMN next_focus TEXT NOT NULL DEFAULT '';
ALTER TABLE monthly_snapshots ADD COLUMN revision INTEGER NOT NULL DEFAULT 0;
CREATE TABLE insights_preferences (
 id INTEGER PRIMARY KEY CHECK(id=1), payload TEXT NOT NULL,
 revision INTEGER NOT NULL DEFAULT 0, updated_at TEXT NOT NULL
);
CREATE TABLE insights_days (
 date TEXT PRIMARY KEY CHECK(length(date)=10), status TEXT NOT NULL CHECK(status IN ('normal','rest','sick','exam','travel','paper_deadline','special')),
 revision INTEGER NOT NULL DEFAULT 0, updated_at TEXT NOT NULL
);
CREATE TABLE insights_receipts (
 id TEXT PRIMARY KEY CHECK(length(id)=36), payload_hash TEXT NOT NULL, created_at TEXT NOT NULL
);
CREATE TABLE insights_words (
 kind TEXT NOT NULL, id TEXT NOT NULL, date TEXT NOT NULL, fingerprint TEXT NOT NULL, tokens TEXT NOT NULL,
 PRIMARY KEY(kind,id)
);
CREATE INDEX insights_words_date ON insights_words(date,kind,id);
CREATE INDEX insights_music_date ON music_plays(played_at) WHERE deleted_at IS NULL;
CREATE INDEX insights_tasks_dates ON tasks(due_date,lifecycle) WHERE deleted_at IS NULL;
ALTER TABLE music_tracks ADD COLUMN album TEXT NOT NULL DEFAULT '';

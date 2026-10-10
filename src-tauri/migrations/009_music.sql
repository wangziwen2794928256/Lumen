ALTER TABLE music_tracks ADD COLUMN platform TEXT NOT NULL DEFAULT '';
ALTER TABLE music_tracks ADD COLUMN identity_key TEXT;
CREATE UNIQUE INDEX idx_music_identity ON music_tracks(identity_key) WHERE identity_key IS NOT NULL AND deleted_at IS NULL;
ALTER TABLE music_plays ADD COLUMN source TEXT NOT NULL DEFAULT 'manual' CHECK(source IN ('manual','windows-media'));
ALTER TABLE music_plays ADD COLUMN source_app TEXT NOT NULL DEFAULT '';
ALTER TABLE music_plays ADD COLUMN journal_id TEXT REFERENCES journal_entries(id);
ALTER TABLE music_plays ADD COLUMN event_key TEXT;
ALTER TABLE music_plays ADD COLUMN revision INTEGER NOT NULL DEFAULT 1;
CREATE UNIQUE INDEX idx_music_event ON music_plays(event_key) WHERE event_key IS NOT NULL;
CREATE INDEX idx_music_period ON music_plays(played_at,deleted_at);
CREATE TABLE music_preferences (
  singleton INTEGER PRIMARY KEY CHECK(singleton=1),
  enabled INTEGER NOT NULL DEFAULT 0 CHECK(enabled IN (0,1)),
  threshold_seconds INTEGER NOT NULL DEFAULT 30 CHECK(threshold_seconds BETWEEN 5 AND 600),
  allowed_app TEXT NOT NULL DEFAULT '',
  revision INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL
);
CREATE TABLE music_receipts (
  request_id TEXT PRIMARY KEY,
  payload_hash TEXT NOT NULL,
  created_at TEXT NOT NULL
);

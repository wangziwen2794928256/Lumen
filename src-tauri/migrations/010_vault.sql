-- Additive migration. Existing artifacts and files remain private and unchanged.
ALTER TABLE artifacts ADD COLUMN artifact_date TEXT;
ALTER TABLE artifacts ADD COLUMN role TEXT NOT NULL DEFAULT '';
ALTER TABLE artifacts ADD COLUMN organization TEXT NOT NULL DEFAULT '';
ALTER TABLE artifacts ADD COLUMN skills TEXT NOT NULL DEFAULT '[]';
ALTER TABLE artifacts ADD COLUMN technologies TEXT NOT NULL DEFAULT '[]';
ALTER TABLE artifacts ADD COLUMN repository_url TEXT NOT NULL DEFAULT '';
ALTER TABLE artifacts ADD COLUMN project_url TEXT NOT NULL DEFAULT '';
ALTER TABLE artifacts ADD COLUMN paper_url TEXT NOT NULL DEFAULT '';
ALTER TABLE artifacts ADD COLUMN journey_id TEXT REFERENCES journeys(id);
ALTER TABLE artifacts ADD COLUMN competition_id TEXT REFERENCES competitions(id);
ALTER TABLE artifacts ADD COLUMN revision INTEGER NOT NULL DEFAULT 0;
ALTER TABLE artifact_files ADD COLUMN name TEXT NOT NULL DEFAULT '附件';
CREATE TABLE vault_receipts(id TEXT PRIMARY KEY, payload_hash TEXT NOT NULL, artifact_id TEXT NOT NULL, created_at TEXT NOT NULL);
CREATE INDEX vault_artifacts_date ON artifacts(deleted_at,archived_at,artifact_date);

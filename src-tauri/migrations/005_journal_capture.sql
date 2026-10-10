ALTER TABLE journal_entries ADD COLUMN entry_type TEXT NOT NULL DEFAULT 'reflection';
ALTER TABLE journal_entries ADD COLUMN is_draft INTEGER NOT NULL DEFAULT 0 CHECK(is_draft IN (0,1));
ALTER TABLE journal_entries ADD COLUMN revision INTEGER NOT NULL DEFAULT 0;
ALTER TABLE journal_entries ADD COLUMN journey_id TEXT REFERENCES journeys(id);
ALTER TABLE journal_entries ADD COLUMN task_id TEXT REFERENCES tasks(id);
ALTER TABLE journal_entries ADD COLUMN music_id TEXT REFERENCES music_tracks(id);
CREATE TABLE journal_recovery (
 client_id TEXT PRIMARY KEY NOT NULL, entry_id TEXT NOT NULL, sequence INTEGER NOT NULL,
 payload TEXT NOT NULL, updated_at TEXT NOT NULL
);
CREATE TABLE journal_save_receipts (
 client_id TEXT NOT NULL, sequence INTEGER NOT NULL, payload_hash TEXT NOT NULL CHECK(length(payload_hash)=64),
 PRIMARY KEY(client_id,sequence)
);
CREATE TABLE journal_attachment_links (
 id TEXT PRIMARY KEY NOT NULL, entry_id TEXT NOT NULL REFERENCES journal_entries(id),
 attachment_id TEXT REFERENCES attachments(id), name TEXT NOT NULL, url TEXT,
 created_at TEXT NOT NULL, deleted_at TEXT,
 CHECK((attachment_id IS NULL) != (url IS NULL))
);
CREATE INDEX journal_attachment_entry ON journal_attachment_links(entry_id,deleted_at);
CREATE INDEX journal_page ON journal_entries(deleted_at,is_draft,entry_date,id);
CREATE VIRTUAL TABLE journal_search USING fts5(entry_id UNINDEXED,title,body,tokenize='trigram');
INSERT INTO journal_search SELECT id,title,body FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0;
CREATE TRIGGER journal_search_insert AFTER INSERT ON journal_entries WHEN new.deleted_at IS NULL AND new.is_draft=0 BEGIN
 INSERT INTO journal_search VALUES(new.id,new.title,new.body); END;
CREATE TRIGGER journal_search_update AFTER UPDATE ON journal_entries BEGIN
 DELETE FROM journal_search WHERE entry_id=old.id;
 INSERT INTO journal_search SELECT new.id,new.title,new.body WHERE new.deleted_at IS NULL AND new.is_draft=0; END;
CREATE TRIGGER journal_search_delete AFTER DELETE ON journal_entries BEGIN DELETE FROM journal_search WHERE entry_id=old.id; END;

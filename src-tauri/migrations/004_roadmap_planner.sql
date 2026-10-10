-- Append-only Phase 6; no business seed data.
ALTER TABLE journeys ADD COLUMN planning_data TEXT NOT NULL DEFAULT '{}';
ALTER TABLE journeys ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK(revision>=0);
ALTER TABLE journey_phases ADD COLUMN planning_data TEXT NOT NULL DEFAULT '{}';
ALTER TABLE journey_phases ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK(revision>=0);
ALTER TABLE subjects ADD COLUMN planning_data TEXT NOT NULL DEFAULT '{}';
ALTER TABLE subjects ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK(revision>=0);
ALTER TABLE quests ADD COLUMN planning_data TEXT NOT NULL DEFAULT '{}';
ALTER TABLE quests ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK(revision>=0);
ALTER TABLE tasks ADD COLUMN planning_data TEXT NOT NULL DEFAULT '{}';
ALTER TABLE tasks ADD COLUMN quest_id TEXT REFERENCES quests(id);
CREATE TABLE planning_preferences (
 id INTEGER PRIMARY KEY CHECK(id=1), weekday_minutes INTEGER NOT NULL CHECK(weekday_minutes BETWEEN 0 AND 1440),
 weekend_minutes INTEGER NOT NULL CHECK(weekend_minutes BETWEEN 0 AND 1440), side_quest_max INTEGER NOT NULL CHECK(side_quest_max BETWEEN 0 AND 10080),
 planning_target INTEGER NOT NULL CHECK(planning_target BETWEEN 0 AND 750), revision INTEGER NOT NULL DEFAULT 0, updated_at TEXT NOT NULL
);
CREATE TABLE capacity_days (
 date TEXT PRIMARY KEY CHECK(length(date)=10), minutes INTEGER NOT NULL CHECK(minutes BETWEEN 0 AND 1440),
 status TEXT NOT NULL CHECK(status IN ('normal','rest','exam','competition','travel','sick','paper_sprint','other')), updated_at TEXT NOT NULL
);
CREATE TABLE planning_decisions (
 id TEXT PRIMARY KEY CHECK(length(id)=36), week_start TEXT NOT NULL, status TEXT NOT NULL CHECK(status IN ('proposed','accepted','modified','rejected','undone')),
 reason TEXT NOT NULL, before_json TEXT NOT NULL, after_json TEXT NOT NULL, basis_hash TEXT NOT NULL CHECK(length(basis_hash)=64),
 created_at TEXT NOT NULL, updated_at TEXT NOT NULL
);
CREATE TABLE planning_blocks (
 id TEXT PRIMARY KEY CHECK(length(id)=36), decision_id TEXT NOT NULL REFERENCES planning_decisions(id),
 task_id TEXT NOT NULL REFERENCES tasks(id), date TEXT NOT NULL, minutes INTEGER NOT NULL CHECK(minutes>0),
 UNIQUE(decision_id,task_id,date)
);
CREATE INDEX idx_planning_blocks_date ON planning_blocks(date,task_id);
ALTER TABLE exam_rule_snapshots ADD COLUMN school TEXT NOT NULL DEFAULT '';
ALTER TABLE exam_rule_snapshots ADD COLUMN college TEXT NOT NULL DEFAULT '';
ALTER TABLE exam_rule_snapshots ADD COLUMN major TEXT NOT NULL DEFAULT '';
ALTER TABLE exam_rule_snapshots ADD COLUMN year INTEGER CHECK(year BETWEEN 2000 AND 2100);
ALTER TABLE exam_rule_snapshots ADD COLUMN written_exam TEXT NOT NULL DEFAULT '';
ALTER TABLE exam_rule_snapshots ADD COLUMN interview TEXT NOT NULL DEFAULT '';
ALTER TABLE exam_rule_snapshots ADD COLUMN source_file TEXT;
ALTER TABLE exam_rule_snapshots ADD COLUMN notes TEXT NOT NULL DEFAULT '';
ALTER TABLE exam_rule_snapshots ADD COLUMN content_hash TEXT;

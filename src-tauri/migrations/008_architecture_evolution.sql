-- Append-only Kernel migration. No core record rewrite, backfill or training.
CREATE TABLE module_preferences (
 id INTEGER PRIMARY KEY CHECK(id=1), payload TEXT NOT NULL CHECK(json_valid(payload)),
 revision INTEGER NOT NULL CHECK(revision>=0), updated_at TEXT NOT NULL
);
CREATE TABLE planner_observations (
 sequence INTEGER PRIMARY KEY AUTOINCREMENT,
 kind TEXT NOT NULL CHECK(kind IN ('task','focus','decision','capacity','preferences')),
 entity_id TEXT NOT NULL, event TEXT NOT NULL, payload TEXT NOT NULL CHECK(json_valid(payload)),
 observed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE INDEX planner_observations_entity ON planner_observations(kind,entity_id,sequence);
CREATE TRIGGER planner_observations_no_update BEFORE UPDATE ON planner_observations BEGIN SELECT RAISE(ABORT,'Kernel observations are append-only'); END;
CREATE TRIGGER planner_observations_no_delete BEFORE DELETE ON planner_observations BEGIN SELECT RAISE(ABORT,'Kernel observations are append-only'); END;
CREATE TRIGGER planner_task_create AFTER INSERT ON tasks BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('task',NEW.id,'created',json_object('estimate_minutes',NEW.estimated_minutes,'date',NEW.due_date,'state',NEW.lifecycle,'revision',NEW.revision)); END;
CREATE TRIGGER planner_task_adjust AFTER UPDATE OF estimated_minutes,due_date,lifecycle,deleted_at ON tasks
WHEN OLD.estimated_minutes IS NOT NEW.estimated_minutes OR OLD.due_date IS NOT NEW.due_date OR OLD.lifecycle IS NOT NEW.lifecycle OR OLD.deleted_at IS NOT NEW.deleted_at BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('task',NEW.id,'adjusted',json_object('estimate_before',OLD.estimated_minutes,'estimate_after',NEW.estimated_minutes,'date_before',OLD.due_date,'date_after',NEW.due_date,'state_before',OLD.lifecycle,'state_after',NEW.lifecycle,'deleted',NEW.deleted_at IS NOT NULL,'revision',NEW.revision)); END;
CREATE TRIGGER planner_focus_transition AFTER UPDATE OF state,elapsed_ms ON focus_sessions
WHEN OLD.state IS NOT NEW.state OR (NEW.state='finished' AND OLD.elapsed_ms IS NOT NEW.elapsed_ms) BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('focus',NEW.id,'transition',json_object('task_id',NEW.task_id,'state_before',OLD.state,'state_after',NEW.state,'planned_seconds',NEW.planned_seconds,'elapsed_ms',NEW.elapsed_ms,'actual_seconds',NEW.actual_seconds,'date',NEW.session_date)); END;
CREATE TRIGGER planner_decision_create AFTER INSERT ON planning_decisions BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('decision',NEW.id,'proposed',json_object('status',NEW.status,'week_start',NEW.week_start,'basis_hash',NEW.basis_hash)); END;
CREATE TRIGGER planner_decision_choice AFTER UPDATE OF status ON planning_decisions WHEN OLD.status IS NOT NEW.status BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('decision',NEW.id,'choice',json_object('before',OLD.status,'after',NEW.status,'week_start',NEW.week_start,'basis_hash',NEW.basis_hash)); END;
CREATE TRIGGER planner_capacity_create AFTER INSERT ON capacity_days BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('capacity',NEW.date,'created',json_object('minutes',NEW.minutes,'status',NEW.status)); END;
CREATE TRIGGER planner_capacity_adjust AFTER UPDATE ON capacity_days WHEN OLD.minutes IS NOT NEW.minutes OR OLD.status IS NOT NEW.status BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('capacity',NEW.date,'adjusted',json_object('minutes_before',OLD.minutes,'minutes_after',NEW.minutes,'status_before',OLD.status,'status_after',NEW.status)); END;
CREATE TRIGGER planner_capacity_remove BEFORE DELETE ON capacity_days BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('capacity',OLD.date,'override_removed',json_object('minutes',OLD.minutes,'status',OLD.status)); END;
CREATE TRIGGER planner_preferences_create AFTER INSERT ON planning_preferences BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('preferences','planning','created',json_object('weekday',NEW.weekday_minutes,'weekend',NEW.weekend_minutes,'side_quest_max',NEW.side_quest_max,'planning_target',NEW.planning_target,'revision',NEW.revision)); END;
CREATE TRIGGER planner_preferences_adjust AFTER UPDATE ON planning_preferences BEGIN
 INSERT INTO planner_observations(kind,entity_id,event,payload) VALUES('preferences','planning','adjusted',json_object('weekday_before',OLD.weekday_minutes,'weekday_after',NEW.weekday_minutes,'weekend_before',OLD.weekend_minutes,'weekend_after',NEW.weekend_minutes,'side_quest_max',NEW.side_quest_max,'planning_target',NEW.planning_target,'revision',NEW.revision)); END;

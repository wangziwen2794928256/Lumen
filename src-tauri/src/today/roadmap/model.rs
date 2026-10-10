use crate::database::{DbError, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Details {
    pub description: String,
    pub category: String,
    pub status: String,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub minimum_minutes: i64,
    pub preferred_minutes: i64,
    pub maximum_minutes: i64,
    pub priority: i64,
    pub color: String,
    pub icon: String,
    pub progress_mode: String,
    pub progress: i64,
    pub parent_id: Option<String>,
    pub preparation_phase: String,
    pub position: i64,
    pub planning_target: Option<i64>,
}
impl Default for Details {
    fn default() -> Self {
        Self {
            description: String::new(),
            category: "study".into(),
            status: "active".into(),
            start_date: None,
            target_date: None,
            minimum_minutes: 0,
            preferred_minutes: 0,
            maximum_minutes: 10080,
            priority: 1,
            color: "sage".into(),
            icon: "route".into(),
            progress_mode: "manual".into(),
            progress: 0,
            parent_id: None,
            preparation_phase: String::new(),
            position: 0,
            planning_target: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityInput {
    pub title: String,
    pub journey_id: Option<String>,
    pub data: Details,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub journey_id: Option<String>,
    pub data: Details,
    pub revision: i64,
    pub deleted_at: Option<String>,
    pub archived_at: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TaskOptions {
    pub earliest_start: Option<String>,
    pub minimum_session: i64,
    pub preferred_session: i64,
    pub splittable: bool,
    pub energy_level: String,
    pub fixed_time: bool,
}
impl Default for TaskOptions {
    fn default() -> Self {
        Self {
            earliest_start: None,
            minimum_session: 15,
            preferred_session: 25,
            splittable: true,
            energy_level: "normal".into(),
            fixed_time: false,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningTask {
    pub id: String,
    pub title: String,
    pub remaining_minutes: i64,
    pub priority: i64,
    pub deadline: Option<String>,
    pub journey_id: Option<String>,
    pub subject_id: Option<String>,
    pub quest_id: Option<String>,
    pub revision: i64,
    pub options: TaskOptions,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Allocation {
    pub task_id: String,
    pub date: String,
    pub minutes: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub weekday_minutes: i64,
    pub weekend_minutes: i64,
    pub side_quest_max: i64,
    pub planning_target: i64,
    pub revision: i64,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            weekday_minutes: 210,
            weekend_minutes: 210,
            side_quest_max: 360,
            planning_target: 400,
            revision: 0,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapacityDay {
    pub date: String,
    pub minutes: i64,
    pub status: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayBudget {
    pub date: String,
    pub capacity: i64,
    pub fixed_minutes: i64,
    pub available: i64,
    pub status: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub days: Vec<DayBudget>,
    pub allocations: Vec<Allocation>,
    pub unscheduled: Vec<(String, i64)>,
    pub warnings: Vec<String>,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub week_start: String,
    pub status: String,
    pub reason: String,
    pub before: Vec<Allocation>,
    pub after: Plan,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleInput {
    pub journey_id: Option<String>,
    pub school: String,
    pub college: String,
    pub major: String,
    pub year: i64,
    pub written_exam: String,
    pub interview: String,
    pub source_url: String,
    pub body: String,
    pub notes: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub input: RuleInput,
    pub captured_at: String,
    pub content_hash: String,
    pub deleted_at: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct Snapshot {
    pub week_start: String,
    pub entities: Vec<Entity>,
    pub tasks: Vec<PlanningTask>,
    pub preferences: Preferences,
    pub days: Vec<CapacityDay>,
    pub blocks: Vec<Allocation>,
    pub decisions: Vec<Decision>,
    pub rules: Vec<Rule>,
    pub budgets: Vec<DayBudget>,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Save {
        id: String,
        kind: String,
        revision: Option<i64>,
        input: EntityInput,
    },
    Lifecycle {
        id: String,
        kind: String,
        revision: i64,
        operation: String,
    },
    Template {
        id: String,
    },
    Preferences {
        input: Preferences,
    },
    Capacity {
        input: CapacityDay,
    },
    TaskPlan {
        id: String,
        revision: i64,
        journey_id: Option<String>,
        subject_id: Option<String>,
        quest_id: Option<String>,
        input: TaskOptions,
    },
    Propose {
        id: String,
    },
    Decide {
        id: String,
        operation: String,
        allocations: Option<Vec<Allocation>>,
    },
    Rule {
        id: String,
        input: RuleInput,
    },
    RuleTrash {
        id: String,
        deleted: bool,
    },
}
pub(super) fn date(value: &str) -> Result<NaiveDate> {
    let d = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
    if d.to_string() != value {
        return Err(DbError::invalid());
    }
    Ok(d)
}
pub(super) fn conflict() -> DbError {
    DbError::new("STATE_CONFLICT", "规划依据已变化，请刷新或重新生成建议。")
}

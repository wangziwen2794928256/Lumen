use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProblemInput {
    pub title: String,
    pub platform: String,
    pub external_id: String,
    pub source_url: String,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub notes: String,
    pub journey_id: Option<String>,
    pub task_id: Option<String>,
    pub competition_id: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptInput {
    pub problem_id: String,
    pub language: String,
    pub outcome: String,
    pub notes: String,
    pub attempted_on: String,
    pub started_at: String,
    pub manual_seconds: i64,
    pub focus_session_id: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContestInput {
    pub title: String,
    pub platform: String,
    pub event_date: String,
    pub source_url: String,
    pub participated: bool,
    pub notes: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Problem {
        request_id: String,
        id: String,
        revision: Option<i64>,
        input: ProblemInput,
    },
    Attempt {
        request_id: String,
        id: String,
        revision: Option<i64>,
        input: AttemptInput,
    },
    Contest {
        request_id: String,
        id: String,
        revision: Option<i64>,
        input: ContestInput,
    },
    Trash {
        request_id: String,
        id: String,
        kind: String,
        revision: i64,
        restore: bool,
    },
}
impl Action {
    pub fn request_id(&self) -> &str {
        match self {
            Self::Problem { request_id, .. }
            | Self::Attempt { request_id, .. }
            | Self::Contest { request_id, .. }
            | Self::Trash { request_id, .. } => request_id,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    pub week: String,
    pub query: String,
    pub platform: String,
    pub status: String,
    pub tag: String,
    pub trash: bool,
    pub page: i64,
    pub selected: Option<String>,
}
#[derive(Serialize)]
pub struct Problem {
    pub id: String,
    pub revision: i64,
    pub deleted_at: Option<String>,
    #[serde(flatten)]
    pub input: ProblemInput,
    pub attempt_count: i64,
    pub solved: bool,
    pub first_attempt: Option<String>,
    pub solved_at: Option<String>,
    pub language: Option<String>,
    pub manual_seconds: i64,
    pub focus_seconds: i64,
}
#[derive(Serialize)]
pub struct Attempt {
    pub id: String,
    pub revision: i64,
    pub deleted_at: Option<String>,
    #[serde(flatten)]
    pub input: AttemptInput,
    pub focus_seconds: i64,
}
#[derive(Serialize)]
pub struct Contest {
    pub id: String,
    pub revision: i64,
    pub deleted_at: Option<String>,
    #[serde(flatten)]
    pub input: ContestInput,
}
#[derive(Serialize)]
pub struct Choice {
    pub id: String,
    pub title: String,
    pub task_id: Option<String>,
}
#[derive(Serialize)]
pub struct Distribution {
    pub label: String,
    pub count: i64,
}
#[derive(Serialize)]
pub struct Analytics {
    pub from: String,
    pub until: String,
    pub problems: i64,
    pub solved: i64,
    pub attempts: i64,
    pub first_pass_numerator: i64,
    pub first_pass_denominator: i64,
    pub manual_seconds: i64,
    pub focus_seconds: i64,
    pub contests: i64,
    pub difficulty: Vec<Distribution>,
    pub topics: Vec<Distribution>,
}
#[derive(Serialize)]
pub struct Snapshot {
    pub problems: Vec<Problem>,
    pub total: i64,
    pub attempts: Vec<Attempt>,
    pub contests: Vec<Contest>,
    pub journeys: Vec<Choice>,
    pub tasks: Vec<Choice>,
    pub focus: Vec<Choice>,
    pub analytics: Analytics,
}

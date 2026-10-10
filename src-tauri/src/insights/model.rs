use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    pub from: String,
    pub until: String,
    pub today: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Preferences {
    pub weights: [f64; 6],
    pub thresholds: [f64; 4],
    pub stop_words: Vec<String>,
    pub memories: bool,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            weights: [1.0, 1.0, 10.0, 10.0, 10.0, 1.0],
            thresholds: [1.0, 30.0, 90.0, 180.0],
            stop_words: vec![],
            memories: true,
        }
    }
}
#[derive(Clone, Default, Serialize)]
pub struct Day {
    pub date: String,
    pub deep_minutes: f64,
    pub coding_minutes: f64,
    pub tasks_completed: i64,
    pub journal_count: i64,
    pub problem_count: i64,
    pub music_count: i64,
    pub planned_minutes: f64,
    pub capacity_minutes: i64,
    pub status: String,
    pub status_revision: i64,
    pub intensity: u8,
}
#[derive(Clone, Serialize)]
pub struct Record {
    pub kind: String,
    pub id: String,
    pub date: String,
    pub title: String,
}
#[derive(Clone, Serialize)]
pub struct Word {
    pub word: String,
    pub count: i64,
    pub records: Vec<Record>,
}
#[derive(Serialize)]
pub struct Snapshot {
    pub from: String,
    pub until: String,
    pub days: Vec<Day>,
    pub heatmap: Vec<Day>,
    pub preferences: Preferences,
    pub preferences_revision: i64,
    pub words: Vec<Word>,
    pub subjects: Vec<serde_json::Value>,
    pub journeys: Vec<serde_json::Value>,
    pub music: Vec<serde_json::Value>,
    pub topics: Vec<serde_json::Value>,
    pub records: Vec<Record>,
    pub task_summary: serde_json::Value,
    pub consistency: serde_json::Value,
    pub previous: serde_json::Value,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Preferences {
        request_id: String,
        revision: i64,
        value: Preferences,
    },
    Day {
        request_id: String,
        date: String,
        revision: i64,
        status: String,
    },
    Review {
        request_id: String,
        week: String,
        revision: i64,
        body: String,
        next_focus: String,
    },
}
impl Action {
    pub fn request_id(&self) -> &str {
        match self {
            Self::Preferences { request_id, .. }
            | Self::Day { request_id, .. }
            | Self::Review { request_id, .. } => request_id,
        }
    }
}

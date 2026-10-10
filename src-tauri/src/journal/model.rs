use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Save {
    pub id: String,
    pub client_id: String,
    pub sequence: i64,
    pub revision: i64,
    pub title: String,
    pub body: String,
    pub date: String,
    pub entry_type: String,
    pub tags: Vec<String>,
    pub journey_id: Option<String>,
    pub task_id: Option<String>,
    pub music_id: Option<String>,
    pub publish: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Filter {
    pub query: String,
    pub from: String,
    pub until: String,
    pub entry_type: String,
    pub tag: String,
    pub mode: String,
    pub page: i64,
}
#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub body: String,
    pub date: String,
    pub entry_type: String,
    pub is_draft: bool,
    pub revision: i64,
    pub journey_id: Option<String>,
    pub task_id: Option<String>,
    pub music_id: Option<String>,
    pub tags: Vec<String>,
    pub deleted_at: Option<String>,
    pub updated_at: String,
}
#[derive(Debug, Serialize)]
pub struct Choice {
    pub id: String,
    pub title: String,
}
#[derive(Debug, Serialize)]
pub struct Recovery {
    pub payload: Save,
    pub updated_at: String,
}
#[derive(Debug, Serialize)]
pub struct Page {
    pub entries: Vec<Entry>,
    pub total: i64,
    pub page: i64,
    pub recoveries: Vec<Recovery>,
    pub journeys: Vec<Choice>,
    pub tasks: Vec<Choice>,
    pub music: Vec<Choice>,
}
#[derive(Debug, Serialize)]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub size_bytes: Option<i64>,
    pub sha256: Option<String>,
}

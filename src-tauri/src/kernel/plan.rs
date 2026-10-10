use crate::database::{DbError, Result};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanItem {
    pub task_id: String,
    pub date: String,
    pub minutes: i64,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanExchange {
    pub schema_version: String,
    pub request_id: String,
    pub basis_hash: String,
    pub source: String,
    pub capacity_minutes: i64,
    pub items: Vec<PlanItem>,
}
pub fn validate_plan(p: &PlanExchange) -> Result<()> {
    crate::database::repository::validate_id(&p.request_id)?;
    if p.schema_version != "1.0"
        || !matches!(
            p.source.as_str(),
            "manual" | "external-proposal" | "local-proposal"
        )
        || p.basis_hash.len() != 64
        || !p.basis_hash.bytes().all(|b| b.is_ascii_hexdigit())
        || !(0..=10080).contains(&p.capacity_minutes)
        || p.items.len() > 500
    {
        return Err(DbError::invalid());
    }
    let mut keys = std::collections::HashSet::new();
    let mut total = 0;
    for i in &p.items {
        crate::database::repository::validate_id(&i.task_id)?;
        let d = chrono::NaiveDate::parse_from_str(&i.date, "%Y-%m-%d")
            .map_err(|_| DbError::invalid())?;
        if d.to_string() != i.date
            || !(1..=1440).contains(&i.minutes)
            || !keys.insert((&i.task_id, &i.date))
        {
            return Err(DbError::invalid());
        }
        total += i.minutes;
    }
    if total > p.capacity_minutes {
        return Err(DbError::invalid());
    }
    Ok(())
}
// No IPC apply endpoint, LLM call or model training. Future Kernel rechecks live basis,
// task revisions, per-day capacity and explicit user acceptance before any commit.

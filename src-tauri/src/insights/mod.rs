mod aggregate;
mod model;
#[cfg(test)]
mod performance;
mod review;
#[cfg(test)]
mod tests;
mod words;
use crate::database::{checksum, now, Database, DbError, Result};
use chrono::{Datelike, Duration, NaiveDate};
pub use model::*;
use serde_json::{json, Value};
use sqlx::Row;
fn date(s: &str) -> Result<NaiveDate> {
    let d = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
    if d.to_string() != s {
        return Err(DbError::invalid());
    }
    Ok(d)
}
fn range(f: &Filter) -> Result<()> {
    let a = date(&f.from)?;
    let b = date(&f.until)?;
    date(&f.today)?;
    if b < a || (b - a).num_days() > 3660 {
        return Err(DbError::invalid());
    }
    Ok(())
}
fn monday(s: &str) -> Result<String> {
    let d = date(s)?;
    Ok((d - Duration::days(d.weekday().num_days_from_monday() as i64)).to_string())
}
fn conflict() -> DbError {
    DbError::new("INSIGHTS_CONFLICT", "记录已变化，请重新载入；输入已保留。")
}
impl Database {
    pub async fn insights_close_months(&self, today: &str) -> Result<usize> {
        let current = date(today)?.format("%Y-%m").to_string();
        let months:Vec<String>=sqlx::query_scalar("WITH dates(d) AS (SELECT local_date FROM focus_intervals UNION SELECT due_date FROM tasks WHERE deleted_at IS NULL AND lifecycle='done' UNION SELECT week_start_date FROM weekly_reviews WHERE deleted_at IS NULL UNION SELECT entry_date FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0 UNION SELECT attempted_on FROM coding_attempts WHERE deleted_at IS NULL UNION SELECT date(played_at,'localtime') FROM music_plays WHERE deleted_at IS NULL UNION SELECT event_date FROM competitions WHERE deleted_at IS NULL UNION SELECT date(created_at,'localtime') FROM artifacts WHERE deleted_at IS NULL) SELECT DISTINCT substr(d,1,7) m FROM dates WHERE substr(d,1,7)<? AND NOT EXISTS (SELECT 1 FROM monthly_snapshots s WHERE s.month=substr(d,1,7) AND CASE WHEN json_valid(s.summary)=1 THEN json_extract(s.summary,'$.closed')=1 ELSE 1 END) ORDER BY m").bind(current).fetch_all(&self.pool).await?;
        let n = months.len();
        for m in months {
            self.insights_month(m, today.to_string()).await?;
        }
        Ok(n)
    }
    pub async fn insights_snapshot(&self, f: Filter) -> Result<Snapshot> {
        aggregate::snapshot(self, f).await
    }
    pub async fn insights_action(&self, a: Action) -> Result<()> {
        review::action(self, a).await
    }
    pub async fn insights_review(&self, week: String) -> Result<Value> {
        review::get(self, &week).await
    }
    pub async fn insights_month(&self, month: String, today: String) -> Result<Value> {
        review::month(self, &month, &today).await
    }
    pub async fn insights_memories(&self, today: String) -> Result<Vec<Record>> {
        aggregate::memories(self, &today).await
    }
}
pub fn spawn_monthly(db: Database) {
    tauri::async_runtime::spawn(async move {
        loop {
            let today = chrono::Local::now().date_naive().to_string();
            if db.require_module("lumen.insights").await.is_ok() {
                let _ = db.insights_close_months(&today).await;
            }
            tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        }
    });
}

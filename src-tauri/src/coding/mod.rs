mod analytics;
mod model;
mod mutations;
mod store;
#[cfg(test)]
mod tests;
use crate::database::{now, Database, DbError, Result};
pub use model::*;
use sqlx::{Row, Sqlite, Transaction};
fn id(s: &str) -> Result<()> {
    crate::database::repository::validate_id(s)
}
fn date(s: &str) -> Result<chrono::NaiveDate> {
    let d = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
    if d.format("%Y-%m-%d").to_string() != s {
        return Err(DbError::invalid());
    }
    Ok(d)
}
fn conflict() -> DbError {
    DbError::new(
        "CODING_CONFLICT",
        "记录已变化，请重新载入后再保存；输入已保留。",
    )
}
fn platform(s: &str) -> Result<()> {
    if [
        "Luogu",
        "NowCoder",
        "Codeforces",
        "LeetCode",
        "AtCoder",
        "Other",
    ]
    .contains(&s)
    {
        Ok(())
    } else {
        Err(DbError::invalid())
    }
}
fn url(s: &str) -> Result<()> {
    if s.is_empty() {
        return Ok(());
    }
    let u = tauri::Url::parse(s).map_err(|_| DbError::invalid())?;
    if s.len() > 2000
        || !["http", "https"].contains(&u.scheme())
        || u.host_str().is_none()
        || !u.username().is_empty()
        || u.password().is_some()
    {
        return Err(DbError::invalid());
    }
    Ok(())
}

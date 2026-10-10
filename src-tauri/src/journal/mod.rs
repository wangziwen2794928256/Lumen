mod attachments;
pub mod capture;
mod model;
mod mutations;
mod store;
#[cfg(test)]
mod tests;
use crate::database::{Database, DbError, Result};
pub use model::*;
use sqlx::Row;

fn id(value: &str) -> Result<()> {
    crate::database::repository::validate_id(value)
}
fn date(value: &str) -> Result<()> {
    let parsed =
        chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
    if parsed.format("%Y-%m-%d").to_string() != value {
        return Err(DbError::invalid());
    }
    Ok(())
}
fn conflict() -> DbError {
    DbError::new(
        "JOURNAL_CONFLICT",
        "记录已被另一个窗口修改，草稿已保留，请恢复为新记录或重新载入。",
    )
}

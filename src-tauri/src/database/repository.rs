use super::{now, Database, DbError, Result};
use serde::Serialize;
use sqlx::Row;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Content {
    pub id: String,
    pub title: String,
    pub body: String,
    pub date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
#[derive(Clone, Copy)]
pub enum ContentKind {
    Task,
    Journal,
}
impl ContentKind {
    fn table(self) -> &'static str {
        match self {
            Self::Task => "tasks",
            Self::Journal => "journal_entries",
        }
    }
    fn date_column(self) -> &'static str {
        match self {
            Self::Task => "due_date",
            Self::Journal => "entry_date",
        }
    }
}
pub struct ContentRepository<'a> {
    db: &'a Database,
    kind: ContentKind,
}
fn validate(title: &str, body: &str, date: Option<&str>, kind: ContentKind) -> Result<()> {
    if title.trim().is_empty() || title.len() > 1000 || body.len() > 1_000_000 {
        return Err(DbError::invalid());
    }
    if matches!(kind, ContentKind::Journal) && date.is_none() {
        return Err(DbError::invalid());
    }
    if let Some(value) = date {
        let parsed =
            chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
        if value.len() != 10 || parsed.format("%Y-%m-%d").to_string() != value {
            return Err(DbError::invalid());
        }
    }
    Ok(())
}
pub(crate) fn validate_id(id: &str) -> Result<()> {
    if uuid::Uuid::parse_str(id)
        .map_err(|_| DbError::invalid())?
        .to_string()
        != id
    {
        return Err(DbError::invalid());
    }
    Ok(())
}
fn content(row: sqlx::sqlite::SqliteRow) -> Content {
    Content {
        id: row.get("id"),
        title: row.get("title"),
        body: row.get("body"),
        date: row.get("content_date"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        deleted_at: row.get("deleted_at"),
    }
}
impl<'a> ContentRepository<'a> {
    pub fn new(db: &'a Database, kind: ContentKind) -> Self {
        Self { db, kind }
    }
    pub async fn create(&self, title: &str, body: &str, date: Option<&str>) -> Result<String> {
        validate(title, body, date, self.kind)?;
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = now();
        let sql = format!(
            "INSERT INTO {}(id,title,body,{},created_at,updated_at) VALUES(?,?,?,?,?,?)",
            self.kind.table(),
            self.kind.date_column()
        );
        sqlx::query(&sql)
            .bind(&id)
            .bind(title)
            .bind(body)
            .bind(date)
            .bind(&timestamp)
            .bind(&timestamp)
            .execute(&self.db.pool)
            .await?;
        Ok(id)
    }
    pub async fn get(&self, id: &str, include_deleted: bool) -> Result<Option<Content>> {
        validate_id(id)?;
        let sql = format!(
            "SELECT *,{} AS content_date FROM {} WHERE id=? AND (? OR deleted_at IS NULL)",
            self.kind.date_column(),
            self.kind.table()
        );
        Ok(sqlx::query(&sql)
            .bind(id)
            .bind(include_deleted)
            .fetch_optional(&self.db.pool)
            .await?
            .map(content))
    }
    pub async fn update(
        &self,
        id: &str,
        title: &str,
        body: &str,
        date: Option<&str>,
    ) -> Result<()> {
        validate_id(id)?;
        validate(title, body, date, self.kind)?;
        let sql = format!(
            "UPDATE {} SET title=?,body=?,{}=?,updated_at=? WHERE id=? AND deleted_at IS NULL",
            self.kind.table(),
            self.kind.date_column()
        );
        let updated = sqlx::query(&sql)
            .bind(title)
            .bind(body)
            .bind(date)
            .bind(now())
            .bind(id)
            .execute(&self.db.pool)
            .await?;
        if updated.rows_affected() == 0 {
            return Err(DbError::new("NOT_FOUND", "记录不存在或已移入回收站。"));
        }
        Ok(())
    }
    pub async fn set_deleted(&self, id: &str, deleted: bool) -> Result<()> {
        validate_id(id)?;
        let timestamp = now();
        let sql = format!(
            "UPDATE {} SET deleted_at=?,updated_at=? WHERE id=?",
            self.kind.table()
        );
        let mut tx = self.db.pool.begin().await?;
        let updated = sqlx::query(&sql)
            .bind(if deleted { Some(&timestamp) } else { None })
            .bind(&timestamp)
            .bind(id)
            .execute(&mut *tx)
            .await?;
        if updated.rows_affected() == 0 {
            tx.rollback().await?;
            return Err(DbError::new("NOT_FOUND", "记录不存在。"));
        }
        tx.commit().await?;
        Ok(())
    }
}
#[derive(Debug, Serialize)]
pub struct SearchHit {
    pub kind: String,
    pub id: String,
    pub title: String,
}
pub async fn search(db: &Database, text: &str) -> Result<Vec<SearchHit>> {
    if text.trim().chars().count() < 3 || text.len() > 200 {
        return Err(DbError::invalid());
    }
    // Literal FTS phrase; no query grammar or user-controlled SQL identifiers.
    let phrase = format!("\"{}\"", text.trim().replace('"', "\"\""));
    let rows = sqlx::query("SELECT kind,entity_id,title FROM content_search WHERE content_search MATCH ? ORDER BY rank LIMIT 50")
        .bind(phrase).fetch_all(&db.pool).await?;
    Ok(rows
        .into_iter()
        .map(|row| SearchHit {
            kind: row.get("kind"),
            id: row.get("entity_id"),
            title: row.get("title"),
        })
        .collect())
}

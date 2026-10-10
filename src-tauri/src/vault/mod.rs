mod files;
#[cfg(test)]
mod tests;
use crate::database::{checksum, now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use sqlx::Row;

pub const KINDS: [&str; 10] = [
    "award",
    "competition",
    "paper",
    "project",
    "certificate",
    "open_source",
    "course_project",
    "research",
    "presentation",
    "other",
];
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub id: String,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub artifact_date: Option<String>,
    pub role: String,
    pub organization: String,
    pub skills: Vec<String>,
    pub technologies: Vec<String>,
    pub repository_url: String,
    pub project_url: String,
    pub paper_url: String,
    pub journey_id: Option<String>,
    pub competition_id: Option<String>,
    pub visibility: String,
    pub revision: i64,
    pub archived_at: Option<String>,
    pub deleted_at: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Save {
        request_id: String,
        artifact: Box<Artifact>,
    },
    State {
        request_id: String,
        id: String,
        revision: i64,
        state: String,
    },
}
fn uuid(value: &str) -> Result<()> {
    uuid::Uuid::parse_str(value).map_err(|_| DbError::invalid())?;
    Ok(())
}
fn conflict() -> DbError {
    DbError::new("CONFLICT", "记录已变更，请刷新后重试。")
}
fn text(value: &str, max: usize) -> Result<()> {
    if value.chars().count() > max
        || value
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
    {
        return Err(DbError::invalid());
    }
    Ok(())
}
fn validate(a: &Artifact) -> Result<()> {
    uuid(&a.id)?;
    text(&a.title, 300)?;
    text(&a.description, 20_000)?;
    if a.title.trim().is_empty()
        || !KINDS.contains(&a.kind.as_str())
        || !["private", "public"].contains(&a.visibility.as_str())
        || a.revision < 0
        || a.archived_at.is_some()
        || a.deleted_at.is_some()
    {
        return Err(DbError::invalid());
    }
    for s in [&a.role, &a.organization] {
        text(s, 300)?;
    }
    for values in [&a.skills, &a.technologies] {
        if values.len() > 50 {
            return Err(DbError::invalid());
        }
        for v in values {
            text(v, 100)?;
            if v.trim().is_empty() {
                return Err(DbError::invalid());
            }
        }
    }
    for url in [&a.repository_url, &a.project_url, &a.paper_url] {
        if url.is_empty() {
            continue;
        }
        text(url, 2048)?;
        let u = tauri::Url::parse(url).map_err(|_| DbError::invalid())?;
        if !["https", "http"].contains(&u.scheme())
            || !u.has_host()
            || !u.username().is_empty()
            || u.password().is_some()
            || url.chars().any(char::is_control)
        {
            return Err(DbError::invalid());
        }
    }
    if let Some(d) = &a.artifact_date {
        let parsed =
            chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|_| DbError::invalid())?;
        if parsed.format("%Y-%m-%d").to_string() != *d {
            return Err(DbError::invalid());
        }
    }
    for id in [&a.journey_id, &a.competition_id].into_iter().flatten() {
        uuid(id)?;
    }
    Ok(())
}
fn row(r: sqlx::sqlite::SqliteRow) -> Result<Artifact> {
    Ok(Artifact {
        id: r.get("id"),
        title: r.get("title"),
        description: r.get("description"),
        kind: r.get("kind"),
        artifact_date: r.get("artifact_date"),
        role: r.get("role"),
        organization: r.get("organization"),
        skills: serde_json::from_str(&r.get::<String, _>("skills"))
            .map_err(|_| DbError::invalid())?,
        technologies: serde_json::from_str(&r.get::<String, _>("technologies"))
            .map_err(|_| DbError::invalid())?,
        repository_url: r.get("repository_url"),
        project_url: r.get("project_url"),
        paper_url: r.get("paper_url"),
        journey_id: r.get("journey_id"),
        competition_id: r.get("competition_id"),
        visibility: r.get("visibility"),
        revision: r.get("revision"),
        archived_at: r.get("archived_at"),
        deleted_at: r.get("deleted_at"),
    })
}
const SELECT: &str="SELECT a.*,COALESCE((SELECT v.visibility FROM public_visibility v WHERE v.entity_type='artifact' AND v.entity_id=a.id AND v.deleted_at IS NULL AND v.archived_at IS NULL),'private') AS visibility FROM artifacts a";
impl Database {
    pub async fn vault_snapshot(&self) -> Result<serde_json::Value> {
        let records = sqlx::query(&format!(
            "{SELECT} ORDER BY a.created_at DESC,a.id LIMIT 1000"
        ))
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(row)
        .collect::<Result<Vec<_>>>()?;
        let journeys=sqlx::query("SELECT id,title FROM journeys WHERE deleted_at IS NULL AND archived_at IS NULL ORDER BY title LIMIT 1000").fetch_all(&self.pool).await?.into_iter().map(|r|serde_json::json!({"id":r.get::<String,_>("id"),"title":r.get::<String,_>("title")})).collect::<Vec<_>>();
        let competitions=sqlx::query("SELECT id,title FROM competitions WHERE deleted_at IS NULL AND archived_at IS NULL ORDER BY title LIMIT 1000").fetch_all(&self.pool).await?.into_iter().map(|r|serde_json::json!({"id":r.get::<String,_>("id"),"title":r.get::<String,_>("title")})).collect::<Vec<_>>();
        Ok(
            serde_json::json!({"artifacts":records,"journeys":journeys,"competitions":competitions,"limit":1000}),
        )
    }
    pub async fn vault_action(&self, action: Action) -> Result<()> {
        let hash = checksum(&serde_json::to_vec(&action).map_err(|_| DbError::invalid())?);
        let (request, id) = match &action {
            Action::Save {
                request_id,
                artifact,
            } => (request_id, &artifact.id),
            Action::State { request_id, id, .. } => (request_id, id),
        };
        uuid(request)?;
        uuid(id)?;
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let previous: Option<String> =
            sqlx::query_scalar("SELECT payload_hash FROM vault_receipts WHERE id=?")
                .bind(request)
                .fetch_optional(&mut *tx)
                .await?;
        if let Some(previous) = previous {
            return if previous == hash {
                Ok(())
            } else {
                Err(conflict())
            };
        }
        let timestamp = now();
        match &action {
            Action::Save { artifact: a, .. } => {
                validate(a)?;
                for (table, reference) in [
                    ("journeys", &a.journey_id),
                    ("competitions", &a.competition_id),
                ] {
                    if let Some(key) = reference {
                        let count:i64=sqlx::query_scalar(&format!("SELECT count(*) FROM {table} WHERE id=? AND deleted_at IS NULL AND archived_at IS NULL")).bind(key).fetch_one(&mut *tx).await?;
                        if count != 1 {
                            return Err(conflict());
                        }
                    }
                }
                let current: Option<i64> =
                    sqlx::query_scalar("SELECT revision FROM artifacts WHERE id=?")
                        .bind(&a.id)
                        .fetch_optional(&mut *tx)
                        .await?;
                if current.is_none() {
                    if a.revision != 0 {
                        return Err(conflict());
                    }
                    sqlx::query(
                        "INSERT INTO artifacts(id,title,created_at,updated_at) VALUES(?,?,?,?)",
                    )
                    .bind(&a.id)
                    .bind(&a.title)
                    .bind(&timestamp)
                    .bind(&timestamp)
                    .execute(&mut *tx)
                    .await?;
                }
                let changed=sqlx::query("UPDATE artifacts SET title=?,description=?,kind=?,artifact_date=?,role=?,organization=?,skills=?,technologies=?,repository_url=?,project_url=?,paper_url=?,journey_id=?,competition_id=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL AND archived_at IS NULL")
                    .bind(a.title.trim()).bind(&a.description).bind(&a.kind).bind(&a.artifact_date).bind(&a.role).bind(&a.organization).bind(serde_json::to_string(&a.skills).map_err(|_|DbError::invalid())?).bind(serde_json::to_string(&a.technologies).map_err(|_|DbError::invalid())?).bind(&a.repository_url).bind(&a.project_url).bind(&a.paper_url).bind(&a.journey_id).bind(&a.competition_id).bind(&timestamp).bind(&a.id).bind(a.revision).execute(&mut *tx).await?;
                if changed.rows_affected() != 1 {
                    return Err(conflict());
                }
                sqlx::query("INSERT INTO public_visibility(id,entity_type,entity_id,visibility,created_at,updated_at) VALUES(?,'artifact',?,?,?,?) ON CONFLICT(entity_type,entity_id) DO UPDATE SET visibility=excluded.visibility,updated_at=excluded.updated_at,deleted_at=NULL,archived_at=NULL").bind(uuid::Uuid::new_v4().to_string()).bind(&a.id).bind(&a.visibility).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
            }
            Action::State {
                id,
                revision,
                state,
                ..
            } => {
                let sql=match state.as_str() {
                    "archive"=>"UPDATE artifacts SET archived_at=?,updated_at=?,revision=revision+1 WHERE id=? AND revision=? AND deleted_at IS NULL AND archived_at IS NULL",
                    "trash"=>"UPDATE artifacts SET deleted_at=?,updated_at=?,revision=revision+1 WHERE id=? AND revision=? AND deleted_at IS NULL",
                    "restore"=>"UPDATE artifacts SET deleted_at=NULL,archived_at=NULL,updated_at=?,revision=revision+1 WHERE id=? AND revision=? AND (deleted_at IS NOT NULL OR archived_at IS NOT NULL)",
                    _=>return Err(DbError::invalid()),
                };
                let q = sqlx::query(sql);
                let q = if state == "restore" {
                    q
                } else {
                    q.bind(&timestamp)
                };
                if q.bind(&timestamp)
                    .bind(id)
                    .bind(revision)
                    .execute(&mut *tx)
                    .await?
                    .rows_affected()
                    != 1
                {
                    return Err(conflict());
                }
            }
        }
        sqlx::query(
            "INSERT INTO vault_receipts(id,payload_hash,artifact_id,created_at) VALUES(?,?,?,?)",
        )
        .bind(request)
        .bind(hash)
        .bind(id)
        .bind(timestamp)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

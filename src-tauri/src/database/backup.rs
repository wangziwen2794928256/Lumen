use super::{checksum, connect, file_checksum, now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    format_version: u32,
    app_version: String,
    created_at: String,
    schema_version: i64,
    database_sha256: String,
    attachments: Vec<Attachment>,
    settings_in_database: bool,
    #[serde(default)]
    attachment_payloads: bool,
}
#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Attachment {
    id: String,
    relative_path: String,
    sha256: String,
    size_bytes: i64,
}
async fn attachment_manifest(
    pool: &sqlx::SqlitePool,
    include_deleted: bool,
) -> Result<Vec<Attachment>> {
    let rows = sqlx::query("SELECT id,relative_path,sha256,size_bytes FROM attachments WHERE ? OR deleted_at IS NULL ORDER BY id").bind(include_deleted).fetch_all(pool).await?;
    Ok(rows
        .into_iter()
        .map(|row| Attachment {
            id: row.get("id"),
            relative_path: row.get("relative_path"),
            sha256: row.get("sha256"),
            size_bytes: row.get("size_bytes"),
        })
        .collect())
}
/// Database + attachment manifest skeleton; attachment payload copying/retention is Phase 13.
pub async fn create(db: &Database) -> Result<PathBuf> {
    create_named(db, "migration").await
}
async fn create_named(db: &Database, kind: &str) -> Result<PathBuf> {
    let folder = db
        .root
        .join("backups")
        .join(format!("{kind}-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&folder)?;
    let target = folder.join("lumen.db");
    let target_text = target.to_str().ok_or_else(DbError::backup)?;
    sqlx::query("VACUUM INTO ?")
        .bind(target_text)
        .execute(&db.pool)
        .await?;
    std::fs::OpenOptions::new()
        .write(true)
        .open(&target)?
        .sync_all()?;
    let snapshot = connect(&target, false, true).await?;
    let result: Result<Manifest> = async {
        let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&snapshot)
            .await?;
        if integrity != "ok" {
            return Err(DbError::backup());
        }
        Ok(Manifest {
            format_version: 1,
            app_version: env!("CARGO_PKG_VERSION").into(),
            created_at: now(),
            schema_version: sqlx::query_scalar("SELECT MAX(version) FROM migration_history")
                .fetch_one(&snapshot)
                .await?,
            database_sha256: file_checksum(&target)?,
            attachments: attachment_manifest(&snapshot, false).await?,
            settings_in_database: true,
            attachment_payloads: false,
        })
    }
    .await;
    snapshot.close().await;
    let manifest = result?;
    let bytes = serde_json::to_vec_pretty(&manifest).map_err(|_| DbError::backup())?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(folder.join("manifest.json"))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    verify(&folder).await?;
    Ok(folder)
}

/// Read-only verification, never restores over production or runs migrations.
pub async fn verify(folder: &Path) -> Result<()> {
    let manifest: Manifest = serde_json::from_slice(&std::fs::read(folder.join("manifest.json"))?)
        .map_err(|_| DbError::backup())?;
    if manifest.format_version != 1
        || !manifest.settings_in_database
        || manifest.schema_version < 1
        || file_checksum(&folder.join("lumen.db"))? != manifest.database_sha256
    {
        return Err(DbError::backup());
    }
    let pool = connect(&folder.join("lumen.db"), false, true).await?;
    let result = async {
        let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&pool)
            .await?;
        let foreign_keys = sqlx::query("PRAGMA foreign_key_check")
            .fetch_all(&pool)
            .await?;
        if integrity != "ok" || !foreign_keys.is_empty() {
            return Err(DbError::backup());
        }
        let rows =
            sqlx::query("SELECT version,name,checksum FROM migration_history ORDER BY version")
                .fetch_all(&pool)
                .await?;
        if rows.len() != manifest.schema_version as usize {
            return Err(DbError::backup());
        }
        for (row, expected) in rows.iter().zip(super::migrations::MIGRATIONS.iter()) {
            if row.get::<i64, _>("version") != expected.version
                || row.get::<String, _>("name") != expected.name
                || row.get::<String, _>("checksum") != checksum(expected.sql.as_bytes())
            {
                return Err(DbError::backup());
            }
        }
        if rows.len() > super::migrations::MIGRATIONS.len() {
            return Err(DbError::backup());
        }
        for table in [
            "users",
            "settings",
            "journeys",
            "journey_phases",
            "subjects",
            "tasks",
            "task_sessions",
            "calendar_events",
            "focus_sessions",
            "quests",
            "competitions",
            "coding_problems",
            "coding_attempts",
            "journal_entries",
            "journal_tags",
            "music_tracks",
            "music_plays",
            "artifacts",
            "artifact_files",
            "weekly_reviews",
            "monthly_snapshots",
            "exam_rule_snapshots",
            "daily_metrics",
            "public_visibility",
            "attachments",
            "integration_accounts",
        ] {
            let sql = format!("SELECT id,created_at,updated_at,deleted_at FROM {table} LIMIT 1");
            sqlx::query(&sql).fetch_optional(&pool).await?;
        }
        // Read real rows instead of only checking existence of files or tables.
        sqlx::query("SELECT id,title,body,due_date FROM tasks LIMIT 1")
            .fetch_optional(&pool)
            .await?;
        sqlx::query("SELECT id,title,body,entry_date FROM journal_entries LIMIT 1")
            .fetch_optional(&pool)
            .await?;
        sqlx::query("SELECT key,value_text FROM settings LIMIT 1")
            .fetch_optional(&pool)
            .await?;
        sqlx::query("SELECT entity_id FROM content_search LIMIT 1")
            .fetch_optional(&pool)
            .await?;
        if attachment_manifest(&pool, manifest.attachment_payloads).await? != manifest.attachments {
            return Err(DbError::backup());
        }
        if manifest.attachment_payloads {
            for attachment in &manifest.attachments {
                verify_attachment(folder, attachment)?;
            }
        }
        Ok(())
    }
    .await;
    pool.close().await;
    result
}

// Both the source and snapshot must remain within their own attachment directory.
// Canonicalization also rejects symlink escapes; filenames never come from renderer paths.
fn attachment_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let name = relative
        .strip_prefix("attachments/")
        .ok_or_else(DbError::backup)?;
    if name.is_empty() || name.contains(['/', '\\', ':']) || name == "." || name == ".." {
        return Err(DbError::backup());
    }
    let base = root.join("attachments").canonicalize()?;
    let path = base.join(name).canonicalize()?;
    if path.parent() != Some(base.as_path()) || !path.is_file() {
        return Err(DbError::backup());
    }
    Ok(path)
}
fn verify_attachment(root: &Path, attachment: &Attachment) -> Result<PathBuf> {
    let path = attachment_path(root, &attachment.relative_path)?;
    if attachment.size_bytes < 0
        || path.metadata()?.len() != attachment.size_bytes as u64
        || file_checksum(&path)? != attachment.sha256
    {
        return Err(DbError::backup());
    }
    Ok(path)
}

/// A complete private snapshot. Existing migration backups keep their v1 behavior.
/// Incomplete folders never receive COMPLETE and are not eligible for restoration.
pub async fn create_complete(db: &Database) -> Result<PathBuf> {
    let folder = create_named(db, "manual").await?;
    let manifest_path = folder.join("manifest.json");
    let mut manifest: Manifest =
        serde_json::from_slice(&std::fs::read(&manifest_path)?).map_err(|_| DbError::backup())?;
    // Trash remains restorable user data; keep its attachment payloads too.
    let snapshot = connect(&folder.join("lumen.db"), false, true).await?;
    let attachments = attachment_manifest(&snapshot, true).await;
    snapshot.close().await;
    manifest.attachments = attachments?;
    std::fs::create_dir(folder.join("attachments"))?;
    for attachment in &manifest.attachments {
        let source = verify_attachment(&db.root, attachment)?;
        let destination = folder.join(&attachment.relative_path);
        let mut input = std::fs::File::open(source)?;
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination)?;
        std::io::copy(&mut input, &mut output)?;
        output.sync_all()?;
        verify_attachment(&folder, attachment)?;
    }
    manifest.attachment_payloads = true;
    let bytes = serde_json::to_vec_pretty(&manifest).map_err(|_| DbError::backup())?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&manifest_path)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    verify(&folder).await?;
    let mut marker = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(folder.join("COMPLETE"))?;
    marker.write_all(checksum(&bytes).as_bytes())?;
    marker.sync_all()?;
    Ok(folder)
}

/// Requires a complete, internally consistent snapshot; does not replace live data.
pub async fn verify_complete(folder: &Path) -> Result<()> {
    let bytes = std::fs::read(folder.join("manifest.json"))?;
    let manifest: Manifest = serde_json::from_slice(&bytes).map_err(|_| DbError::backup())?;
    if !manifest.attachment_payloads
        || std::fs::read_to_string(folder.join("COMPLETE"))? != checksum(&bytes)
    {
        return Err(DbError::backup());
    }
    verify(folder).await
}

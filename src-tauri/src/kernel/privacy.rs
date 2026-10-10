use crate::database::{backup, Database, DbError, Result};
use serde::Serialize;
use sqlx::Row;
use std::path::Path;
#[derive(Serialize)]
pub struct RestoreReview {
    pub candidate: String,
    pub schema: i64,
    pub integrity: String,
    pub tables: usize,
    pub sample_rows: i64,
    pub replaces_live_database: bool,
}
pub async fn rehearse(db: &Database, source: &Path) -> Result<RestoreReview> {
    backup::verify_complete(source).await?;
    let target = db
        .root
        .join("cache")
        .join(format!("restore-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&target)?;
    std::fs::create_dir(target.join("attachments"))?;
    for name in ["lumen.db", "manifest.json", "COMPLETE"] {
        copy_new(&source.join(name), &target.join(name))?;
    }
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(source.join("manifest.json"))?)
            .map_err(|_| DbError::backup())?;
    for attachment in manifest["attachments"]
        .as_array()
        .ok_or_else(DbError::backup)?
    {
        let path = attachment["relative_path"]
            .as_str()
            .ok_or_else(DbError::backup)?;
        let name = path
            .strip_prefix("attachments/")
            .ok_or_else(DbError::backup)?;
        if name.is_empty() || name.contains(['/', '\\', ':']) || name == "." || name == ".." {
            return Err(DbError::backup());
        }
        let base = source.join("attachments").canonicalize()?;
        if source.join(path).canonicalize()?.parent() != Some(base.as_path()) {
            return Err(DbError::backup());
        }
        // Source has passed strict attachment path validation. Verify copied payloads again.
        copy_new(&source.join(path), &target.join(path))?;
    }
    backup::verify_complete(&target).await?;
    // Opening SQLite may change its journal header. Re-seal the derived candidate
    // only after the actual open/read/close drill succeeds.
    std::fs::remove_file(target.join("COMPLETE"))?;
    let restored = Database::open(&target).await?;
    let result:Result<RestoreReview>=async{
        let status=restored.status().await?;
        let tables=sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '%search%' AND name!='migration_history'").fetch_all(&restored.pool).await?;
        let mut sample_rows=0;
        for row in &tables {let name:String=row.get("name");
            // SQLite-owned schema identifiers, quoted and never interpolated from file input.
            if !name.chars().all(|c|c.is_ascii_alphanumeric()||c=='_'){return Err(DbError::backup());}
            sample_rows+=sqlx::query_scalar::<_,i64>(&format!("SELECT count(*) FROM (SELECT 1 FROM \"{name}\" LIMIT 1)")).fetch_one(&restored.pool).await?;
        }
        Ok(RestoreReview{candidate:target.to_string_lossy().into_owned(),schema:status.schema_version,integrity:status.integrity,tables:tables.len(),sample_rows,replaces_live_database:false})
    }.await;
    restored.pool.close().await;
    let review = result?;
    let mut manifest = manifest;
    manifest["database_sha256"] = crate::database::file_checksum(&target.join("lumen.db"))?.into();
    manifest["schema_version"] = review.schema.into();
    let bytes = serde_json::to_vec_pretty(&manifest).map_err(|_| DbError::backup())?;
    std::fs::write(target.join("manifest.json"), &bytes)?;
    std::fs::write(target.join("COMPLETE"), crate::database::checksum(&bytes))?;
    backup::verify_complete(&target).await?;
    std::fs::write(
        target.join("RESTORE_REVIEW.json"),
        serde_json::to_vec_pretty(&review).map_err(|_| DbError::backup())?,
    )?;
    Ok(review)
}
fn copy_new(source: &Path, target: &Path) -> Result<()> {
    let mut input = std::fs::File::open(source)?;
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target)?;
    std::io::copy(&mut input, &mut output)?;
    output.sync_all()?;
    Ok(())
}
#[derive(Serialize)]
pub struct PrivacyStatus {
    pub data_directory: String,
    pub activity_tracking: bool,
    pub windows_media_enabled: bool,
    pub integrations: usize,
    pub credential_references: usize,
    pub network_upload: bool,
}
impl Database {
    pub async fn privacy_status(&self) -> Result<PrivacyStatus> {
        let media: Option<i64> =
            sqlx::query_scalar("SELECT enabled FROM music_preferences WHERE singleton=1")
                .fetch_optional(&self.pool)
                .await?;
        Ok(PrivacyStatus{data_directory:self.root.to_string_lossy().into_owned(),activity_tracking:false,windows_media_enabled:media==Some(1),integrations:sqlx::query_scalar::<_,i64>("SELECT count(*) FROM integration_accounts WHERE deleted_at IS NULL").fetch_one(&self.pool).await? as usize,credential_references:sqlx::query_scalar::<_,i64>("SELECT count(*) FROM integration_accounts WHERE deleted_at IS NULL AND credential_ref IS NOT NULL AND credential_ref!=''").fetch_one(&self.pool).await? as usize,network_upload:false})
    }
    pub async fn privacy_disable(&self, delete: bool) -> Result<()> {
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        sqlx::query("UPDATE music_preferences SET enabled=0,allowed_app='',revision=revision+1,updated_at=? WHERE singleton=1").bind(crate::database::now()).execute(&mut *tx).await?;
        sqlx::query("UPDATE integration_accounts SET credential_ref=NULL,deleted_at=?,updated_at=? WHERE deleted_at IS NULL").bind(crate::database::now()).bind(crate::database::now()).execute(&mut *tx).await?;
        if delete {
            sqlx::query("UPDATE music_plays SET deleted_at=?,updated_at=?,revision=revision+1 WHERE source='windows-media' AND deleted_at IS NULL").bind(crate::database::now()).bind(crate::database::now()).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::repository::{ContentKind, ContentRepository};
    #[tokio::test]
    async fn restore_reopens_real_rows_without_replacing_live_data() {
        let root = tempfile::tempdir().unwrap();
        let db = Database::open(root.path()).await.unwrap();
        let id = ContentRepository::new(&db, ContentKind::Task)
            .create("RESTORE_PRIVATE_FIXTURE", "body", None)
            .await
            .unwrap();
        let snapshot = backup::create_complete(&db).await.unwrap();
        ContentRepository::new(&db, ContentKind::Task)
            .update(&id, "new local text", "new body", None)
            .await
            .unwrap();
        let review = rehearse(&db, &snapshot).await.unwrap();
        backup::verify_complete(Path::new(&review.candidate))
            .await
            .unwrap();
        assert_eq!(review.integrity, "ok");
        assert_eq!(review.schema, 10);
        assert!(review.sample_rows > 0);
        assert!(!review.replaces_live_database);
        let candidate = Database::open(Path::new(&review.candidate)).await.unwrap();
        assert_eq!(
            ContentRepository::new(&candidate, ContentKind::Task)
                .get(&id, true)
                .await
                .unwrap()
                .unwrap()
                .title,
            "RESTORE_PRIVATE_FIXTURE"
        );
        assert_eq!(
            ContentRepository::new(&db, ContentKind::Task)
                .get(&id, true)
                .await
                .unwrap()
                .unwrap()
                .title,
            "new local text"
        );
        candidate.pool.close().await;
        db.pool.close().await;
    }
    #[tokio::test]
    async fn disconnect_redacts_references_and_preserves_manual_history() {
        let root = tempfile::tempdir().unwrap();
        let db = Database::open(root.path()).await.unwrap();
        assert!(!db.privacy_status().await.unwrap().activity_tracking);
        let track = uuid::Uuid::new_v4().to_string();
        let time = crate::database::now();
        sqlx::query("INSERT INTO music_tracks(id,title,created_at,updated_at) VALUES(?,'private fixture',?,?)").bind(&track).bind(&time).bind(&time).execute(&db.pool).await.unwrap();
        for source in ["manual", "windows-media"] {
            sqlx::query("INSERT INTO music_plays(id,track_id,played_at,created_at,updated_at,source) VALUES(?,?,?,?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(&track).bind(&time).bind(&time).bind(&time).bind(source).execute(&db.pool).await.unwrap();
        }
        db.privacy_disable(false).await.unwrap();
        assert_eq!(db.privacy_status().await.unwrap().credential_references, 0);
        db.privacy_disable(true).await.unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM music_plays WHERE source='manual' AND deleted_at IS NULL"
            )
            .fetch_one(&db.pool)
            .await
            .unwrap(),
            1
        );
        assert_eq!(sqlx::query_scalar::<_,i64>("SELECT count(*) FROM music_plays WHERE source='windows-media' AND deleted_at IS NULL").fetch_one(&db.pool).await.unwrap(),0);
        db.pool.close().await;
    }
}

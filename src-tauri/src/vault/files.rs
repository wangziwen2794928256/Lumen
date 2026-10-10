use super::*;
use crate::database::{checksum, file_checksum};
use std::{io::Write, path::Path};
impl Database {
    pub async fn vault_files(&self, artifact_id: &str) -> Result<serde_json::Value> {
        uuid(artifact_id)?;
        let rows=sqlx::query("SELECT f.id,f.name,a.size_bytes,a.sha256,a.mime_type FROM artifact_files f JOIN attachments a ON a.id=f.attachment_id WHERE f.artifact_id=? AND f.deleted_at IS NULL AND a.deleted_at IS NULL ORDER BY f.created_at,f.id").bind(artifact_id).fetch_all(&self.pool).await?;
        Ok(serde_json::json!(rows.into_iter().map(|r|serde_json::json!({"id":r.get::<String,_>("id"),"name":r.get::<String,_>("name"),"size_bytes":r.get::<i64,_>("size_bytes"),"sha256":r.get::<String,_>("sha256"),"mime_type":r.get::<String,_>("mime_type")})).collect::<Vec<_>>()))
    }
    pub async fn vault_import(&self, id: &str, revision: i64, source: &Path) -> Result<()> {
        uuid(id)?;
        if !source.is_file() || std::fs::metadata(source)?.len() > 20 * 1024 * 1024 {
            return Err(DbError::invalid());
        }
        let bytes = std::fs::read(source)?;
        if bytes.len() > 20 * 1024 * 1024 {
            return Err(DbError::invalid());
        }
        let (ext, mime) = if bytes.starts_with(b"%PDF-") {
            ("pdf", "application/pdf")
        } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            ("png", "image/png")
        } else if bytes.starts_with(&[255, 216, 255]) {
            ("jpg", "image/jpeg")
        } else if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
            ("webp", "image/webp")
        } else {
            return Err(DbError::new(
                "ATTACHMENT_INVALID",
                "仅支持20 MB以内的PDF、PNG、JPEG、WebP。",
            ));
        };
        let name = source
            .file_name()
            .and_then(|x| x.to_str())
            .ok_or_else(DbError::invalid)?;
        text(name, 255)?;
        let folder = self.root.join("attachments").canonicalize()?;
        if !folder.starts_with(self.root.canonicalize()?) {
            return Err(DbError::invalid());
        }
        let key = uuid::Uuid::new_v4().to_string();
        let relative = format!("attachments/{key}.{ext}");
        let path = self.root.join(&relative);
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let timestamp = now();
        if sqlx::query("UPDATE artifacts SET revision=revision+1,updated_at=? WHERE id=? AND revision=? AND deleted_at IS NULL AND archived_at IS NULL").bind(&timestamp).bind(id).bind(revision).execute(&mut *tx).await?.rows_affected()!=1 {return Err(conflict());}
        // Exclusive random destination; failed imports never replace an existing attachment.
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES(?,?,?,?,?,?,?)").bind(&key).bind(relative).bind(mime).bind(bytes.len() as i64).bind(checksum(&bytes)).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO artifact_files(id,artifact_id,attachment_id,name,created_at,updated_at) VALUES(?,?,?,?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(id).bind(key).bind(name).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn vault_verify(&self, id: &str) -> Result<(String, Option<std::path::PathBuf>)> {
        uuid(id)?;
        let r=sqlx::query("SELECT a.relative_path,a.size_bytes,a.sha256 FROM artifact_files f JOIN attachments a ON a.id=f.attachment_id JOIN artifacts v ON v.id=f.artifact_id WHERE f.id=? AND f.deleted_at IS NULL AND a.deleted_at IS NULL AND v.deleted_at IS NULL").bind(id).fetch_one(&self.pool).await?;
        let relative: String = r.get("relative_path");
        let p = Path::new(&relative);
        if !relative.starts_with("attachments/")
            || p.components().count() != 2
            || p.components()
                .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            return Err(DbError::invalid());
        }
        let p = self.root.join(p);
        if !p.exists() {
            return Ok(("missing".into(), None));
        }
        let p = p.canonicalize()?;
        if !p.starts_with(self.root.join("attachments").canonicalize()?)
            || !p.starts_with(self.root.canonicalize()?)
        {
            return Err(DbError::invalid());
        }
        if !p.is_file()
            || std::fs::metadata(&p)?.len() != r.get::<i64, _>("size_bytes") as u64
            || file_checksum(&p)? != r.get::<String, _>("sha256")
        {
            return Ok(("changed".into(), None));
        }
        Ok(("verified".into(), Some(p)))
    }
}

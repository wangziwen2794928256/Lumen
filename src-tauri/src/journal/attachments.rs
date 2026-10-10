use super::*;
use crate::database::{checksum, file_checksum, now};
use std::path::Path;
impl Database {
    pub async fn journal_attachments(&self, entry_id: &str) -> Result<Vec<Attachment>> {
        id(entry_id)?;
        Ok(sqlx::query("SELECT l.id,l.name,l.url,a.size_bytes,a.sha256 FROM journal_attachment_links l LEFT JOIN attachments a ON a.id=l.attachment_id WHERE l.entry_id=? AND l.deleted_at IS NULL ORDER BY l.created_at").bind(entry_id).fetch_all(&self.pool).await?.into_iter().map(|r|Attachment{id:r.get("id"),name:r.get("name"),url:r.get("url"),size_bytes:r.get("size_bytes"),sha256:r.get("sha256")}).collect())
    }
    pub async fn journal_import(&self, entry_id: &str, source: &Path) -> Result<()> {
        let entry = self.journal_get(entry_id).await?;
        if entry.deleted_at.is_some() {
            return Err(conflict());
        }
        let metadata = std::fs::metadata(source)?;
        if !metadata.is_file() || metadata.len() > 20 * 1024 * 1024 {
            return Err(DbError::new(
                "ATTACHMENT_INVALID",
                "仅支持 20 MB 以内的图片或 PDF。",
            ));
        }
        let bytes = std::fs::read(source)?;
        if bytes.len() > 20 * 1024 * 1024 {
            return Err(DbError::invalid());
        }
        let (ext, mime) = if bytes.starts_with(b"%PDF-") {
            ("pdf", "application/pdf")
        } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            ("png", "image/png")
        } else if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
            ("jpg", "image/jpeg")
        } else if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
            ("webp", "image/webp")
        } else {
            return Err(DbError::new(
                "ATTACHMENT_INVALID",
                "文件内容不是支持的图片或 PDF。",
            ));
        };
        let key = uuid::Uuid::new_v4().to_string();
        let relative = format!("attachments/{key}.{ext}");
        let directory = self.root.join("attachments");
        let canonical = directory.canonicalize()?;
        if !canonical.starts_with(self.root.canonicalize()?) {
            return Err(DbError::invalid());
        }
        let path = self.root.join(&relative);
        // Exclusive destination avoids replacing an existing file. fsync before publishing metadata.
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        let timestamp = now();
        let name = source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("附件");
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let live: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM journal_entries WHERE id=? AND deleted_at IS NULL",
        )
        .bind(entry_id)
        .fetch_one(&mut *tx)
        .await?;
        if live != 1 {
            return Err(conflict());
        }
        sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES(?,?,?,?,?,?,?)").bind(&key).bind(relative).bind(mime).bind(bytes.len() as i64).bind(checksum(&bytes)).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO journal_attachment_links(id,entry_id,attachment_id,name,created_at) VALUES(?,?,?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(entry_id).bind(key).bind(name).bind(timestamp).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn journal_link(&self, entry_id: &str, url: &str) -> Result<()> {
        if url.len() > 2048
            || url.chars().any(char::is_control)
            || !(url.starts_with("https://") || url.starts_with("http://"))
        {
            return Err(DbError::invalid());
        }
        let parsed = tauri::Url::parse(url).map_err(|_| DbError::invalid())?;
        if !parsed.has_host() {
            return Err(DbError::invalid());
        }
        if self.journal_get(entry_id).await?.deleted_at.is_some() {
            return Err(conflict());
        }
        sqlx::query("INSERT INTO journal_attachment_links(id,entry_id,name,url,created_at) VALUES(?,?,?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(entry_id).bind(url).bind(url).bind(now()).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn journal_verify(&self, link_id: &str) -> Result<String> {
        id(link_id)?;
        let row=sqlx::query("SELECT a.relative_path,a.sha256,a.size_bytes FROM journal_attachment_links l JOIN attachments a ON a.id=l.attachment_id WHERE l.id=? AND l.deleted_at IS NULL").bind(link_id).fetch_one(&self.pool).await?;
        let relative: String = row.get("relative_path");
        let parts: Vec<_> = Path::new(&relative).components().collect();
        if parts.len() != 2
            || !relative.starts_with("attachments/")
            || parts
                .iter()
                .any(|c| !matches!(c, std::path::Component::Normal(_)))
        {
            return Err(DbError::new("ATTACHMENT_INVALID", "附件路径不在受控目录。"));
        }
        let path = self.root.join(relative);
        if !path.exists() {
            return Ok("missing".into());
        }
        let canonical = path.canonicalize()?;
        if !canonical.starts_with(self.root.join("attachments").canonicalize()?)
            || !canonical.starts_with(self.root.canonicalize()?)
        {
            return Err(DbError::invalid());
        }
        let correct = std::fs::metadata(&canonical)?.len()
            == row.get::<i64, _>("size_bytes") as u64
            && file_checksum(&canonical)? == row.get::<String, _>("sha256");
        Ok(if correct { "verified" } else { "changed" }.into())
    }
    pub async fn journal_open_attachment(&self, link_id: &str) -> Result<()> {
        if self.journal_verify(link_id).await? != "verified" {
            return Err(DbError::new(
                "ATTACHMENT_INVALID",
                "附件缺失或哈希不匹配，未打开。",
            ));
        }
        let relative:String=sqlx::query_scalar("SELECT a.relative_path FROM journal_attachment_links l JOIN attachments a ON a.id=l.attachment_id WHERE l.id=?").bind(link_id).fetch_one(&self.pool).await?;
        let path = self.root.join(relative).canonicalize()?;
        crate::kernel::platform::open_verified_attachment(&path)
    }
}

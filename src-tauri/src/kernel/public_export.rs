//! Kernel-owned whitelist: this API never serializes database rows or private associations.
use crate::database::{now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Profile {
    pub display_name: String,
    pub headline: String,
    pub about: String,
    pub links: Vec<Link>,
    pub public: bool,
    pub revision: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub label: String,
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicArtifact {
    pub id: String,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub date: Option<String>,
    pub role: String,
    pub organization: String,
    pub skills: Vec<String>,
    pub technologies: Vec<String>,
    pub links: Vec<Link>,
    pub public: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicProfile {
    pub schema_version: String,
    pub exported_at: String,
    pub profile: Option<Profile>,
    pub artifacts: Vec<PublicArtifact>,
}
impl PublicProfile {
    pub fn review_hash(&self) -> Result<String> {
        Ok(crate::database::checksum(
            &serde_json::to_vec(&(&self.profile, &self.artifacts))
                .map_err(|_| DbError::invalid())?,
        ))
    }
    pub fn require_review(&self, expected: &str) -> Result<()> {
        if self.review_hash()? != expected {
            return Err(DbError::new(
                "PUBLIC_REVIEW_STALE",
                "公开资料已变更，请重新检查后再导出。",
            ));
        }
        Ok(())
    }
}
pub fn validate_url(url: &str) -> Result<()> {
    let parsed = tauri::Url::parse(url).map_err(|_| DbError::invalid())?;
    if url.len() > 2048
        || url.chars().any(|c| c.is_control() || c.is_whitespace())
        || !["https", "http"].contains(&parsed.scheme())
        || !parsed.has_host()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        return Err(DbError::invalid());
    }
    Ok(())
}
fn validate(p: &Profile) -> Result<()> {
    if p.display_name.chars().count() > 100
        || p.headline.chars().count() > 300
        || p.about.chars().count() > 10_000
        || p.links.len() > 20
        || p.revision < 0
        || [&p.display_name, &p.headline, &p.about]
            .iter()
            .any(|s| s.chars().any(|c| c.is_control() && c != '\n' && c != '\t'))
    {
        return Err(DbError::invalid());
    }
    for link in &p.links {
        if link.label.trim().is_empty()
            || link.label.chars().count() > 100
            || link.label.chars().any(char::is_control)
        {
            return Err(DbError::invalid());
        }
        validate_url(&link.url)?;
    }
    Ok(())
}
async fn read_profile(connection: &mut sqlx::SqliteConnection) -> Result<Profile> {
    let text: Option<String> = sqlx::query_scalar("SELECT value_text FROM settings WHERE key='profile.v1' AND deleted_at IS NULL AND archived_at IS NULL")
        .fetch_optional(connection).await?;
    let profile = match text {
        Some(s) => serde_json::from_str(&s).map_err(|_| DbError::invalid())?,
        None => Profile::default(),
    };
    validate(&profile)?;
    Ok(profile)
}
impl Database {
    pub async fn profile_get(&self) -> Result<Profile> {
        read_profile(&mut *self.pool.acquire().await?).await
    }
    pub async fn profile_save(&self, mut profile: Profile) -> Result<()> {
        validate(&profile)?;
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let old = read_profile(&mut tx).await?;
        if old.revision != profile.revision {
            return Err(DbError::new("CONFLICT", "个人档案已变更，请刷新后重试。"));
        }
        profile.revision = profile
            .revision
            .checked_add(1)
            .ok_or_else(DbError::invalid)?;
        let json = serde_json::to_string(&profile).map_err(|_| DbError::invalid())?;
        sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES (?,'profile.v1',?,?,?) ON CONFLICT(key) DO UPDATE SET value_text=excluded.value_text,updated_at=excluded.updated_at,archived_at=NULL,deleted_at=NULL")
            .bind(uuid::Uuid::new_v4().to_string()).bind(json).bind(now()).bind(now()).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn public_projection(&self) -> Result<PublicProfile> {
        let mut tx = self.pool.begin().await?;
        let mut profile = read_profile(&mut tx).await?;
        // Local revision is never a public field with private editing history.
        profile.revision = 0;
        let rows = sqlx::query("SELECT a.id,a.title,a.description,a.kind,a.artifact_date,a.role,a.organization,a.skills,a.technologies,a.repository_url,a.project_url,a.paper_url FROM artifacts a JOIN public_visibility v ON v.entity_id=a.id AND v.entity_type='artifact' AND v.visibility='public' AND v.archived_at IS NULL AND v.deleted_at IS NULL WHERE a.archived_at IS NULL AND a.deleted_at IS NULL ORDER BY a.id")
            .fetch_all(&mut *tx).await?;
        let mut artifacts = Vec::new();
        for row in rows {
            let kind: String = row.get("kind");
            if !crate::vault::KINDS.contains(&kind.as_str()) {
                continue;
            }
            let id: String = row.get("id");
            uuid::Uuid::parse_str(&id).map_err(|_| DbError::invalid())?;
            let mut links = Vec::new();
            for (column, label) in [
                ("repository_url", "Repository"),
                ("project_url", "Project"),
                ("paper_url", "Paper"),
            ] {
                let url: String = row.get(column);
                if !url.is_empty() {
                    validate_url(&url)?;
                    links.push(Link {
                        label: label.into(),
                        url,
                    });
                }
            }
            artifacts.push(PublicArtifact {
                id,
                title: row.get("title"),
                description: row.get("description"),
                kind,
                date: row.get("artifact_date"),
                role: row.get("role"),
                organization: row.get("organization"),
                skills: serde_json::from_str(row.get("skills")).map_err(|_| DbError::invalid())?,
                technologies: serde_json::from_str(row.get("technologies"))
                    .map_err(|_| DbError::invalid())?,
                links,
                public: true,
            });
        }
        tx.commit().await?;
        Ok(PublicProfile {
            schema_version: "1.0".into(),
            exported_at: now(),
            profile: profile.public.then_some(profile),
            artifacts,
        })
    }
    pub async fn public_export(&self, expected_hash: &str) -> Result<ExportReceipt> {
        let projection = self.public_projection().await?;
        projection.require_review(expected_hash)?;
        let json = serde_json::to_vec_pretty(&projection).map_err(|_| DbError::invalid())?;
        // A new directory is never reused or overwritten; renderer cannot supply a path.
        let id = uuid::Uuid::new_v4().to_string();
        let directory = self.root.join("exports").join(format!("website-{id}"));
        std::fs::create_dir(&directory)?;
        for (name, bytes) in assets(&json) {
            std::fs::write(directory.join(name), bytes)?;
        }
        let receipt = ExportReceipt {
            id,
            directory: directory.to_string_lossy().into_owned(),
            sha256: crate::database::checksum(&json),
            artifacts: projection.artifacts.len(),
        };
        // Written last: no successful receipt is returned for a partial export.
        std::fs::write(directory.join("COMPLETE"), receipt.sha256.as_bytes())?;
        Ok(receipt)
    }
}
#[derive(Serialize)]
pub struct ExportReceipt {
    pub id: String,
    pub directory: String,
    pub sha256: String,
    pub artifacts: usize,
}
pub fn assets(json: &[u8]) -> [(&'static str, &[u8]); 4] {
    [
        ("index.html", include_bytes!("../../../website/index.html")),
        ("site.js", include_bytes!("../../../website/site.js")),
        ("site.css", include_bytes!("../../../website/site.css")),
        ("public-profile.json", json),
    ]
}
#[cfg(test)]
mod tests;

use super::{backup, now, Database, DbError, Result};
use chrono::{Datelike, NaiveDate};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Policy {
    pub enabled: bool,
    pub daily: usize,
    pub weekly: usize,
    pub monthly: usize,
}
impl Default for Policy {
    fn default() -> Self {
        Self {
            enabled: true,
            daily: 7,
            weekly: 4,
            monthly: 6,
        }
    }
}
impl Policy {
    fn validate(&self) -> Result<()> {
        if !(1..=90).contains(&self.daily)
            || !(1..=52).contains(&self.weekly)
            || !(1..=60).contains(&self.monthly)
        {
            return Err(DbError::invalid());
        }
        Ok(())
    }
}
pub async fn policy(db: &Database) -> Result<Policy> {
    let text: Option<String> = sqlx::query_scalar(
        "SELECT value_text FROM settings WHERE key='backup.policy.v1' AND deleted_at IS NULL",
    )
    .fetch_optional(&db.pool)
    .await?;
    let p: Policy = match text {
        Some(s) => serde_json::from_str(&s).map_err(|_| DbError::invalid())?,
        None => Policy::default(),
    };
    p.validate()?;
    Ok(p)
}
pub async fn save_policy(db: &Database, p: Policy) -> Result<()> {
    p.validate()?;
    let text = serde_json::to_string(&p).map_err(|_| DbError::invalid())?;
    let time = now();
    sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'backup.policy.v1',?,?,?) ON CONFLICT(key) DO UPDATE SET value_text=excluded.value_text,updated_at=excluded.updated_at,deleted_at=NULL,archived_at=NULL")
        .bind(uuid::Uuid::new_v4().to_string()).bind(text).bind(&time).bind(&time).execute(&db.pool).await?;
    Ok(())
}
fn date_of(name: &str) -> Option<NaiveDate> {
    let rest = name.strip_prefix("daily-")?;
    let date = rest.get(..10)?;
    if rest.get(10..11)? != "-" {
        return None;
    }
    let id = rest.get(11..)?;
    if uuid::Uuid::parse_str(id).ok()?.to_string() != id {
        return None;
    }
    let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    (parsed.format("%Y-%m-%d").to_string() == date).then_some(parsed)
}
// Keep the newest snapshot in each of the newest N distinct calendar buckets.
// Tiers overlap: union avoids making three copies of the same snapshot.
fn keep_indices(dates: &[NaiveDate], p: &Policy) -> BTreeSet<usize> {
    let mut keep = BTreeSet::new();
    let mut days = BTreeSet::new();
    let mut weeks = BTreeSet::new();
    let mut months = BTreeSet::new();
    for (i, d) in dates.iter().enumerate() {
        if days.len() < p.daily && days.insert(*d) {
            keep.insert(i);
        }
        let week = d.iso_week();
        if weeks.len() < p.weekly && weeks.insert((week.year(), week.week())) {
            keep.insert(i);
        }
        if months.len() < p.monthly && months.insert((d.year(), d.month())) {
            keep.insert(i);
        }
    }
    keep
}
fn backup_roots(db: &Database) -> Result<BTreeSet<PathBuf>> {
    let mut roots = BTreeSet::from([db.root.join("backups").canonicalize()?]);
    // Packaged Windows hosts may expose a merged AppData view: the directory
    // resolves logically while its files resolve into the package's LocalCache.
    // Only the actual core database's sibling backup directory is another root.
    let core = crate::kernel::storage::core_database(&db.root).canonicalize()?;
    if let Some(parent) = core.parent() {
        let physical = parent.join("backups");
        if physical.is_dir() {
            roots.insert(physical.canonicalize()?);
        }
    }
    Ok(roots)
}
async fn candidates(db: &Database, day: NaiveDate) -> Result<Vec<(NaiveDate, PathBuf)>> {
    let roots = backup_roots(db)?;
    let base = db.root.join("backups").canonicalize()?;
    let mut result = Vec::new();
    for entry in std::fs::read_dir(&base)? {
        let entry = entry?;
        // Reparse points/symlinks and unknown folders never become cleanup candidates.
        let kind = entry.file_type()?;
        if !kind.is_dir() || kind.is_symlink() {
            continue;
        }
        let Some(date) = date_of(&entry.file_name().to_string_lossy()) else {
            continue;
        };
        if date > day {
            continue;
        }
        let absolute = entry.path().canonicalize()?;
        if !absolute.parent().is_some_and(|p| roots.contains(p))
            || absolute.file_name() != Some(entry.file_name().as_os_str())
        {
            continue;
        }
        if backup::verify_complete(&absolute).await.is_ok() {
            result.push((date, absolute));
        }
    }
    result.sort_by(|a, b| b.cmp(a));
    Ok(result)
}
/// Runs while the app is open. A missed day is not backfilled with invented data.
/// The cross-process lock covers creation, validation and retention.
pub async fn run_day(db: &Database, day: NaiveDate) -> Result<Option<PathBuf>> {
    let p = policy(db).await?;
    if !p.enabled {
        return Ok(None);
    }
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(db.root.join("backup.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| DbError::new("BACKUP_BUSY", "另一个窗口正在备份。"))?;
    let existing = candidates(db, day).await?;
    if let Some((_, path)) = existing.iter().find(|(date, _)| *date == day) {
        return Ok(Some(path.clone()));
    }
    // A failed new snapshot never triggers retention of old snapshots.
    let source = backup::create_complete(db).await?;
    let target = db
        .root
        .join("backups")
        .join(format!("daily-{day}-{}", uuid::Uuid::new_v4()));
    std::fs::rename(source, &target)?;
    let target = target.canonicalize()?;
    backup::verify_complete(&target).await?;
    let list = candidates(db, day).await?;
    let keep = keep_indices(&list.iter().map(|(d, _)| *d).collect::<Vec<_>>(), &p);
    let roots = backup_roots(db)?;
    for (i, (_, path)) in list.iter().enumerate() {
        if keep.contains(&i) {
            continue;
        }
        let absolute = path.canonicalize()?;
        if !absolute.parent().is_some_and(|p| roots.contains(p))
            || absolute != *path
            || date_of(&path.file_name().unwrap_or_default().to_string_lossy()).is_none()
        {
            return Err(DbError::backup());
        }
        backup::verify_complete(&absolute).await?;
        // Exact verified absolute child of the application's backup directory only.
        std::fs::remove_dir_all(absolute)?;
    }
    Ok(Some(target))
}
#[derive(Clone, Serialize, Default)]
pub struct SchedulerStatus {
    pub last_success: Option<String>,
    pub error_code: Option<String>,
}
static STATUS: std::sync::OnceLock<std::sync::Mutex<SchedulerStatus>> = std::sync::OnceLock::new();
pub fn status() -> SchedulerStatus {
    STATUS
        .get_or_init(Default::default)
        .lock()
        .map(|s| s.clone())
        .unwrap_or_default()
}
pub fn initialize(db: Database) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(300));
        loop {
            interval.tick().await;
            let day = chrono::Local::now().date_naive();
            let result = run_day(&db, day).await;
            if let Ok(mut status) = STATUS.get_or_init(Default::default).lock() {
                match result {
                    Ok(Some(_)) => {
                        status.last_success = Some(day.to_string());
                        status.error_code = None;
                    }
                    Ok(None) => {
                        status.error_code = None;
                    }
                    Err(e) => status.error_code = Some(e.code.into()),
                }
            }
        }
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tiers_are_a_union_of_calendar_buckets() {
        let day = NaiveDate::from_ymd_opt(2026, 10, 10).unwrap();
        let dates: Vec<_> = (0..400).map(|n| day - chrono::Duration::days(n)).collect();
        let p = Policy::default();
        let keep = keep_indices(&dates, &p);
        assert!(keep.len() <= 17);
        assert!((0..7).all(|n| keep.contains(&n)));
        assert_eq!(
            keep,
            BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 10, 13, 20, 40, 71, 102, 132])
        );
        let selected_months: BTreeSet<_> = keep
            .iter()
            .map(|i| (dates[*i].year(), dates[*i].month()))
            .collect();
        assert_eq!(selected_months.len(), 6);
        assert!(date_of("daily-2026-10-10-00000000-0000-4000-8000-000000000001").is_some());
        assert!(date_of("migration-2026-10-10-00000000-0000-4000-8000-000000000001").is_none());
        assert!(date_of("daily-2026-02-30-00000000-0000-4000-8000-000000000001").is_none());
    }
    #[tokio::test]
    async fn daily_snapshot_is_reused_after_reopen() {
        let root = tempfile::tempdir().unwrap();
        let day = NaiveDate::from_ymd_opt(2026, 10, 11).unwrap();
        let db = Database::open(root.path()).await.unwrap();
        let first = run_day(&db, day).await.unwrap().unwrap();
        let hash = super::super::file_checksum(&first.join("lumen.db")).unwrap();
        db.pool.close().await;
        let reopened = Database::open(root.path()).await.unwrap();
        let second = run_day(&reopened, day).await.unwrap().unwrap();
        assert_eq!(first, second);
        assert_eq!(
            hash,
            super::super::file_checksum(&second.join("lumen.db")).unwrap()
        );
        assert_eq!(candidates(&reopened, day).await.unwrap().len(), 1);
        reopened.pool.close().await;
    }
    #[tokio::test]
    async fn daily_idempotence_retention_and_failure_preserve_old_backups() {
        let root = tempfile::tempdir().unwrap();
        let db = Database::open(root.path()).await.unwrap();
        let day = NaiveDate::from_ymd_opt(2026, 10, 10).unwrap();
        let p = Policy {
            enabled: true,
            daily: 1,
            weekly: 1,
            monthly: 1,
        };
        save_policy(&db, p).await.unwrap();
        let old = run_day(&db, day).await.unwrap().unwrap();
        assert_eq!(run_day(&db, day).await.unwrap().unwrap(), old);
        let protected = backup::create_complete(&db).await.unwrap();
        let migration = backup::create(&db).await.unwrap();
        let next = run_day(&db, day + chrono::Duration::days(1))
            .await
            .unwrap()
            .unwrap();
        assert!(!old.exists());
        assert!(next.exists() && protected.exists() && migration.exists());
        sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES('00000000-0000-4000-8000-000000000001','attachments/missing.png','image/png',0,?,?,?)").bind(super::super::checksum(b"")).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
        assert!(run_day(&db, day + chrono::Duration::days(2)).await.is_err());
        assert!(next.exists());
        backup::verify_complete(&next).await.unwrap();
        save_policy(
            &db,
            Policy {
                enabled: false,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert!(run_day(&db, day + chrono::Duration::days(2))
            .await
            .unwrap()
            .is_none());
        db.pool.close().await;
    }
}

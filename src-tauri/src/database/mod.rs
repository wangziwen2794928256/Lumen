pub mod backup;
pub mod backup_schedule;
mod error;
mod migrations;
pub mod repository;
#[cfg(test)]
mod tests;

pub use error::{DbError, Result};
use fs2::FileExt;
use serde::Serialize;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::{ConnectOptions, SqlitePool};
use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: SqlitePool,
    pub(crate) root: PathBuf,
}
#[derive(Serialize)]
pub struct DatabaseStatus {
    pub schema_version: i64,
    pub integrity: String,
    pub storage: &'static str,
    pub task_count: i64,
    pub journal_count: i64,
}
pub(crate) fn now() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
pub(crate) fn checksum(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
pub(crate) fn file_checksum(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut reader = std::io::BufReader::new(std::fs::File::open(path)?);
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 65536];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hash.finalize()))
}
pub(crate) async fn connect(path: &Path, create: bool, read_only: bool) -> Result<SqlitePool> {
    let mut options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(create)
        .read_only(read_only)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5))
        .disable_statement_logging();
    if !read_only {
        options = options
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Full);
    }
    Ok(SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?)
}
impl Database {
    /// Production callers supply ONLY app.path().app_data_dir(); fixtures supply TempDir.
    pub async fn open(root: &Path) -> Result<Self> {
        std::fs::create_dir_all(root)?;
        crate::kernel::storage::initialize(root)?;
        std::fs::create_dir_all(root.join("attachments"))?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(root.join("migration.lock"))?;
        lock.try_lock_exclusive()
            .map_err(|_| DbError::new("DATABASE_BUSY", "另一个窗口正在初始化数据库。"))?;
        let pool = connect(&crate::kernel::storage::core_database(root), true, false).await?;
        let database = Self {
            pool,
            root: root.to_path_buf(),
        };
        let outcome = migrations::apply(&database, &migrations::MIGRATIONS).await;
        if let Err(error) = outcome {
            database.pool.close().await;
            return Err(error);
        }
        database.status().await?;
        Ok(database)
    }
    pub async fn status(&self) -> Result<DatabaseStatus> {
        let integrity: String = sqlx::query_scalar("PRAGMA quick_check")
            .fetch_one(&self.pool)
            .await?;
        if integrity != "ok" {
            return Err(DbError::new("DATABASE_CORRUPT", "数据库完整性检查失败。"));
        }
        Ok(DatabaseStatus {
            schema_version: sqlx::query_scalar(
                "SELECT COALESCE(MAX(version),0) FROM migration_history",
            )
            .fetch_one(&self.pool)
            .await?,
            integrity,
            storage: "AppData/lumen.db",
            task_count: sqlx::query_scalar("SELECT count(*) FROM tasks WHERE deleted_at IS NULL")
                .fetch_one(&self.pool)
                .await?,
            journal_count: sqlx::query_scalar(
                "SELECT count(*) FROM journal_entries WHERE deleted_at IS NULL",
            )
            .fetch_one(&self.pool)
            .await?,
        })
    }
}

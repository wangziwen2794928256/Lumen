use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DbError {
    pub code: &'static str,
    pub message: &'static str,
}
impl DbError {
    pub fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
    pub fn invalid() -> Self {
        Self::new("INVALID_INPUT", "输入不符合数据格式要求。")
    }
    pub fn migration() -> Self {
        Self::new(
            "MIGRATION_MISMATCH",
            "数据库版本或迁移校验不匹配；未继续迁移。",
        )
    }
    pub fn backup() -> Self {
        Self::new("BACKUP_INVALID", "备份校验失败；未修改原数据库。")
    }
}
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for DbError {}
impl From<std::io::Error> for DbError {
    fn from(_: std::io::Error) -> Self {
        Self::new(
            "STORAGE_UNAVAILABLE",
            "数据目录不可访问；请检查磁盘空间与权限。",
        )
    }
}
impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref db) = error {
            match db.code().as_deref() {
                Some("5" | "6") => return Self::new("DATABASE_BUSY", "数据库正忙，请稍后重试。"),
                Some("11" | "26") => {
                    return Self::new("DATABASE_CORRUPT", "数据库不可读；请使用已验证备份恢复。")
                }
                _ => {}
            }
            if db.is_foreign_key_violation() || db.is_unique_violation() || db.is_check_violation()
            {
                return Self::new("CONSTRAINT_VIOLATION", "记录违反数据约束。");
            }
        }
        Self::new("DATABASE_ERROR", "数据库操作失败；未披露记录内容。")
    }
}
pub type Result<T> = std::result::Result<T, DbError>;

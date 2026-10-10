use crate::database::{DbError, Result};
use std::path::{Path, PathBuf};
pub const DIRECTORIES: [&str; 9] = [
    "plugins",
    "attachments",
    "backups",
    "exports",
    "extensions",
    "themes",
    "logs",
    "cache",
    "core",
];
pub fn initialize(root: &Path) -> Result<()> {
    for d in DIRECTORIES {
        std::fs::create_dir_all(root.join(d))?;
    }
    Ok(())
}
// Only resolves existing isolated namespaces; never follows a path supplied by a guest.
pub fn plugin_namespace(root: &Path, id: &str) -> Result<PathBuf> {
    if !super::permissions::valid_id(id) || id.starts_with("lumen.") {
        return Err(DbError::invalid());
    }
    let base = root.join("plugins").canonicalize()?;
    let path = base.join(id);
    let real = path.canonicalize()?;
    if !real.starts_with(&base) || real == base {
        return Err(DbError::invalid());
    }
    Ok(real)
}
pub fn core_database(root: &Path) -> PathBuf {
    root.join("lumen.db")
}

use crate::database::{DbError, Result};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommunityManifest {
    pub id: String,
    pub version: String,
    pub api_version: String,
    pub runtime: String,
    pub entry: String,
    pub permissions: Vec<String>,
}
pub fn valid_id(id: &str) -> bool {
    id.len() <= 80
        && id.split('.').count() >= 2
        && id.split('.').all(|s| {
            !s.is_empty()
                && s.as_bytes()[0].is_ascii_lowercase()
                && s.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
}
pub fn validate_manifest(m: &CommunityManifest) -> Result<()> {
    if !valid_id(&m.id)
        || m.id.starts_with("lumen.")
        || m.api_version != "1.0"
        || m.runtime != "wasm-component"
        || m.entry != "plugin.wasm"
        || m.version.split('.').count() != 3
        || m.version.split('.').any(|s| s.parse::<u32>().is_err())
        || m.permissions.len() > 3
    {
        return Err(DbError::invalid());
    }
    let allowed = ["storage.own", "widget.contribute", "plan.propose"];
    let mut found = std::collections::HashSet::new();
    if m.permissions
        .iter()
        .any(|p| !allowed.contains(&p.as_str()) || !found.insert(p))
    {
        return Err(DbError::new(
            "PERMISSION_DENIED",
            "插件权限不被 Kernel 允许。",
        ));
    }
    Ok(())
}
// Runtime is deliberately closed. Passing validation is NOT execution authorization.
pub fn authorize_community_execution(_m: &CommunityManifest) -> Result<()> {
    Err(DbError::new(
        "PLUGIN_RUNTIME_DEFERRED",
        "本阶段不运行社区插件。",
    ))
}
#[derive(Debug)]
pub struct Grant {
    pub plugin_id: String,
    pub version: String,
    pub content_hash: String,
    pub capabilities: Vec<String>,
    pub revoked: bool,
}
pub fn check_capability(
    m: &CommunityManifest,
    g: &Grant,
    loaded_hash: &str,
    capability: &str,
) -> Result<()> {
    validate_manifest(m)?;
    if g.revoked
        || g.plugin_id != m.id
        || g.version != m.version
        || loaded_hash.len() != 64
        || !loaded_hash.bytes().all(|b| b.is_ascii_hexdigit())
        || g.content_hash != loaded_hash
        || !m.permissions.iter().any(|p| p == capability)
        || !g.capabilities.iter().any(|p| p == capability)
    {
        return Err(DbError::new(
            "PERMISSION_DENIED",
            "插件能力未被授予或已撤销。",
        ));
    }
    Ok(())
}
#[derive(Debug, PartialEq)]
pub enum Lifecycle {
    Staged,
    Disabled,
    Enabled,
    UpdateStaged,
    RollbackStaged,
    Uninstalled,
}
pub fn transition(current: Lifecycle, action: &str) -> Result<Lifecycle> {
    use Lifecycle::*;
    match (current, action) {
        (Staged, "install") => Ok(Disabled),
        (Disabled, "enable") => Err(DbError::new(
            "PLUGIN_RUNTIME_DEFERRED",
            "本阶段不运行社区插件。",
        )),
        (Enabled, "disable") => Ok(Disabled),
        (Disabled, "update") => Ok(UpdateStaged),
        (UpdateStaged, "commit") => Ok(Disabled),
        (UpdateStaged, "rollback") => Ok(Disabled),
        (Disabled, "rollback") => Ok(RollbackStaged),
        (RollbackStaged, "commit") => Ok(Disabled),
        (Disabled, "uninstall") => Ok(Uninstalled),
        _ => Err(DbError::invalid()),
    }
}

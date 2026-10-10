use crate::database::{now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use sqlx::Row;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Name {
    #[serde(rename = "zh-CN")]
    pub zh: String,
    pub en: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub id: String,
    pub name: Name,
    pub version: String,
    pub api_version: String,
    pub layer: String,
    pub route: String,
    pub icon: String,
    pub required: bool,
    pub status: String,
    pub permissions: Vec<String>,
}
pub fn catalogue() -> Vec<Manifest> {
    serde_json::from_str(include_str!("../../../src/kernel/modules/catalogue.json"))
        .expect("bundled manifest is tested")
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preference {
    pub id: String,
    pub enabled: bool,
    pub hidden: bool,
    pub pinned: bool,
    pub order: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub revision: i64,
    pub items: Vec<Preference>,
}
impl Default for Registry {
    fn default() -> Self {
        Self {
            revision: 0,
            items: catalogue()
                .into_iter()
                .enumerate()
                .map(|(order, m)| Preference {
                    id: m.id,
                    enabled: true,
                    hidden: false,
                    pinned: false,
                    order: order as i64,
                })
                .collect(),
        }
    }
}
pub fn validate(value: &Registry) -> Result<()> {
    let known = catalogue();
    if value.revision < 0 || value.items.len() != known.len() {
        return Err(DbError::invalid());
    }
    let mut ids = std::collections::HashSet::new();
    for i in &value.items {
        let m = known
            .iter()
            .find(|m| m.id == i.id)
            .ok_or_else(DbError::invalid)?;
        if !ids.insert(&i.id) || !(0..=1000).contains(&i.order) || (m.required && !i.enabled) {
            return Err(DbError::invalid());
        }
    }
    Ok(())
}
impl Database {
    pub async fn module_registry(&self) -> Result<Registry> {
        let row = sqlx::query("SELECT payload,revision FROM module_preferences WHERE id=1")
            .fetch_optional(&self.pool)
            .await?;
        match row {
            None => Ok(Registry::default()),
            Some(r) => {
                let mut v: Registry = serde_json::from_str(r.get::<&str, _>("payload"))
                    .map_err(|_| DbError::invalid())?;
                v.revision = r.get("revision");
                // Catalogue growth must preserve older installed preferences without rewriting them.
                let known = catalogue();
                let mut seen = std::collections::HashSet::new();
                if v.items
                    .iter()
                    .any(|p| !seen.insert(p.id.clone()) || !known.iter().any(|m| m.id == p.id))
                {
                    return Err(DbError::invalid());
                }
                let mut order = v.items.iter().map(|p| p.order).max().unwrap_or(-1) + 1;
                for m in known {
                    if !seen.contains(&m.id) {
                        v.items.push(Preference {
                            id: m.id,
                            enabled: true,
                            hidden: false,
                            pinned: false,
                            order,
                        });
                        order += 1;
                    }
                }
                validate(&v)?;
                Ok(v)
            }
        }
    }
    pub async fn module_save(&self, value: Registry) -> Result<Registry> {
        validate(&value)?;
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let actual: Option<i64> =
            sqlx::query_scalar("SELECT revision FROM module_preferences WHERE id=1")
                .fetch_optional(&mut *tx)
                .await?;
        if actual.unwrap_or(0) != value.revision {
            return Err(DbError::new(
                "MODULE_CONFLICT",
                "模块配置已变化，请重新载入。",
            ));
        }
        let next = Registry {
            revision: value.revision + 1,
            ..value
        };
        let json = serde_json::to_string(&next).map_err(|_| DbError::invalid())?;
        sqlx::query("INSERT INTO module_preferences(id,payload,revision,updated_at) VALUES(1,?,?,?) ON CONFLICT(id) DO UPDATE SET payload=excluded.payload,revision=excluded.revision,updated_at=excluded.updated_at").bind(json).bind(next.revision).bind(now()).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(next)
    }
    pub async fn require_module(&self, id: &str) -> Result<()> {
        if !self
            .module_registry()
            .await?
            .items
            .iter()
            .any(|p| p.id == id && p.enabled)
        {
            return Err(DbError::new("MODULE_DISABLED", "此模块已停用。"));
        }
        Ok(())
    }
}

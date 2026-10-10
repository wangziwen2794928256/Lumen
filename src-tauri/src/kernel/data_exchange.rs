use crate::database::{checksum, now, Database, DbError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Column, Row, SqliteConnection, TypeInfo, ValueRef};
use std::collections::{BTreeMap, BTreeSet};
// Portable data backup; excludes secrets, integration grants and executable preferences.
const TABLES: &[&str] = &[
    "users",
    "journeys",
    "journey_phases",
    "subjects",
    "tasks",
    "task_sessions",
    "calendar_events",
    "focus_sessions",
    "focus_intervals",
    "quests",
    "competitions",
    "coding_problems",
    "coding_attempts",
    "journal_entries",
    "journal_tags",
    "music_tracks",
    "music_plays",
    "artifacts",
    "weekly_reviews",
    "monthly_snapshots",
    "exam_rule_snapshots",
    "public_visibility",
    "planning_decisions",
    "planning_blocks",
];
type Record = BTreeMap<String, Value>;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exchange {
    pub format: String,
    pub schema: i64,
    pub tables: BTreeMap<String, Vec<Record>>,
}
#[derive(Debug, Serialize, Clone, Default)]
pub struct Review {
    pub add: usize,
    pub update: usize,
    pub conflicts: usize,
    pub invalid: usize,
    pub unchanged: usize,
    pub fingerprint: String,
}
fn record(row: &sqlx::sqlite::SqliteRow) -> Result<Record> {
    row.columns()
        .iter()
        .map(|c| {
            let raw = row.try_get_raw(c.ordinal())?;
            let value = if raw.is_null() {
                Value::Null
            } else {
                match raw.type_info().name() {
                    "INTEGER" => Value::from(row.try_get::<i64, _>(c.ordinal())?),
                    "REAL" => Value::from(row.try_get::<f64, _>(c.ordinal())?),
                    "TEXT" => Value::from(row.try_get::<String, _>(c.ordinal())?),
                    _ => return Err(DbError::invalid()),
                }
            };
            Ok((c.name().to_string(), value))
        })
        .collect()
}
async fn snapshot(conn: &mut SqliteConnection) -> Result<Exchange> {
    let mut tables = BTreeMap::new();
    for table in TABLES {
        let order = if *table == "focus_intervals" {
            "session_id,segment_no,local_date"
        } else {
            "id"
        };
        let sql = format!("SELECT * FROM {table} ORDER BY {order}");
        let rows = sqlx::query(&sql).fetch_all(&mut *conn).await?;
        tables.insert(
            (*table).into(),
            rows.iter().map(record).collect::<Result<_>>()?,
        );
    }
    Ok(Exchange {
        format: "lumen-data-backup/1".into(),
        schema: 10,
        tables,
    })
}
impl Database {
    pub async fn private_exchange(&self) -> Result<Exchange> {
        let mut tx = self.pool.begin().await?;
        let data = snapshot(&mut tx).await?;
        tx.commit().await?;
        Ok(data)
    }
    pub async fn exchange_import(&self, text: &str, expected: Option<&str>) -> Result<Review> {
        if text.len() > 64 * 1024 * 1024 {
            return Err(DbError::invalid());
        }
        let incoming: Exchange = serde_json::from_str(text).map_err(|_| DbError::invalid())?;
        if incoming.format != "lumen-data-backup/1"
            || incoming.schema != 10
            || incoming
                .tables
                .keys()
                .any(|k| !TABLES.contains(&k.as_str()))
            || incoming.tables.values().map(Vec::len).sum::<usize>() > 100000
        {
            return Err(DbError::invalid());
        }
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        sqlx::query("PRAGMA defer_foreign_keys=ON")
            .execute(&mut *tx)
            .await?;
        let current = snapshot(&mut tx).await?;
        let active:i64=sqlx::query_scalar("SELECT count(*) FROM focus_sessions WHERE state IN ('running','paused','interrupted') AND deleted_at IS NULL").fetch_one(&mut *tx).await?;
        if active > 0 {
            return Err(DbError::new(
                "IMPORT_FOCUS_ACTIVE",
                "请先结束当前专注，再检查导入。",
            ));
        }
        let bytes = serde_json::to_vec(&(&incoming, &current)).map_err(|_| DbError::invalid())?;
        let mut review = Review {
            fingerprint: checksum(&bytes),
            ..Default::default()
        };
        if expected.is_some_and(|e| e != review.fingerprint) {
            return Err(DbError::new(
                "IMPORT_STALE",
                "导入资料或本地数据已变化，请重新检查。",
            ));
        }
        for table in TABLES {
            let Some(rows) = incoming.tables.get(*table) else {
                continue;
            };
            let columns = sqlx::query(&format!("PRAGMA table_info({table})"))
                .fetch_all(&mut *tx)
                .await?;
            let names: BTreeSet<String> = columns.iter().map(|r| r.get("name")).collect();
            let pk: Vec<&str> = if *table == "focus_intervals" {
                vec!["session_id", "segment_no", "local_date"]
            } else {
                vec!["id"]
            };
            let mut seen = BTreeSet::new();
            for row in rows {
                let id = row.get(pk[0]).and_then(Value::as_str);
                let identity =
                    serde_json::to_string(&pk.iter().map(|k| row.get(*k)).collect::<Vec<_>>())
                        .map_err(|_| DbError::invalid())?;
                if row.keys().cloned().collect::<BTreeSet<_>>() != names
                    || id.is_none()
                    || !seen.insert(identity)
                    || crate::database::repository::validate_id(id.unwrap_or_default()).is_err()
                    || columns.iter().any(|c| {
                        let name: String = c.get("name");
                        let kind: String = c.get("type");
                        match row.get(&name) {
                            Some(Value::Null) => false,
                            Some(v) => match kind.as_str() {
                                "INTEGER" => v.as_i64().is_none(),
                                "REAL" => !v.is_number(),
                                "TEXT" => !v.is_string(),
                                _ => true,
                            },
                            None => true,
                        }
                    })
                    || (*table == "focus_sessions"
                        && !matches!(
                            row.get("state").and_then(Value::as_str),
                            Some("finished" | "cancelled")
                        ))
                    || row.iter().any(|(k, v)| {
                        if let Some(s) = v.as_str() {
                            if s.len() > 1_000_000 {
                                return true;
                            }
                            if k.ends_with("_url") && !s.is_empty() {
                                return tauri::Url::parse(s)
                                    .map(|u| {
                                        !matches!(u.scheme(), "http" | "https")
                                            || !u.has_host()
                                            || !u.username().is_empty()
                                            || u.password().is_some()
                                    })
                                    .unwrap_or(true);
                            }
                        }
                        !matches!(
                            v,
                            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_)
                        )
                    })
                {
                    review.invalid += 1;
                    continue;
                }
                let old = current.tables[*table]
                    .iter()
                    .find(|r| pk.iter().all(|k| r.get(*k) == row.get(*k)));
                if old == Some(row) {
                    review.unchanged += 1;
                    continue;
                }
                if let Some(old) = old {
                    let newer = match (
                        row.get("updated_at").and_then(Value::as_str),
                        old.get("updated_at").and_then(Value::as_str),
                    ) {
                        (Some(a), Some(b)) => match (
                            chrono::DateTime::parse_from_rfc3339(a),
                            chrono::DateTime::parse_from_rfc3339(b),
                        ) {
                            (Ok(a), Ok(b)) => a > b,
                            _ => false,
                        },
                        _ => false,
                    };
                    if !newer {
                        review.conflicts += 1;
                        continue;
                    }
                }
                let keys: Vec<_> = row.keys().collect();
                let sql = format!(
                    "INSERT INTO {table} ({}) VALUES ({}) ON CONFLICT({}) DO UPDATE SET {}",
                    keys.iter()
                        .map(|s| format!("\"{s}\""))
                        .collect::<Vec<_>>()
                        .join(","),
                    vec!["?"; keys.len()].join(","),
                    pk.join(","),
                    keys.iter()
                        .filter(|s| !pk.contains(&s.as_str()))
                        .map(|s| format!("\"{s}\"=excluded.\"{s}\""))
                        .collect::<Vec<_>>()
                        .join(",")
                );
                sqlx::query("SAVEPOINT import_row")
                    .execute(&mut *tx)
                    .await?;
                let mut query = sqlx::query(&sql);
                for key in &keys {
                    if key.as_str() == "revision" {
                        if let Some(old_revision) =
                            old.and_then(|r| r.get("revision")).and_then(Value::as_i64)
                        {
                            query = query.bind(
                                row[*key]
                                    .as_i64()
                                    .unwrap_or_default()
                                    .max(old_revision.saturating_add(1)),
                            );
                            continue;
                        }
                    }
                    query = match &row[*key] {
                        Value::Null => query.bind(Option::<String>::None),
                        Value::String(s) => query.bind(s),
                        Value::Bool(b) => query.bind(*b),
                        Value::Number(n) if n.is_i64() => query.bind(n.as_i64()),
                        Value::Number(n) => query.bind(n.as_f64()),
                        _ => return Err(DbError::invalid()),
                    };
                }
                if query.execute(&mut *tx).await.is_err() {
                    sqlx::query("ROLLBACK TO import_row")
                        .execute(&mut *tx)
                        .await?;
                    review.invalid += 1;
                } else if old.is_some() {
                    review.update += 1;
                } else {
                    review.add += 1;
                }
                sqlx::query("RELEASE import_row").execute(&mut *tx).await?;
            }
        }
        review.invalid += sqlx::query("PRAGMA foreign_key_check")
            .fetch_all(&mut *tx)
            .await?
            .len();
        if review.invalid == 0 && review.conflicts == 0 && review.add + review.update > 0 {
            let sessions: Vec<String> = sqlx::query_scalar("SELECT id FROM focus_sessions")
                .fetch_all(&mut *tx)
                .await?;
            for id in sessions {
                crate::today::intervals::rebuild_metrics(&mut tx, &id, &now()).await?;
            }
        }
        if expected.is_some() {
            if review.invalid > 0 || review.conflicts > 0 {
                return Err(DbError::new(
                    "IMPORT_BLOCKED",
                    "存在冲突或无效记录，未导入。",
                ));
            }
            tx.commit().await?;
        } else {
            tx.rollback().await?;
        }
        Ok(review)
    }
    pub async fn private_export(&self, format: &str) -> Result<String> {
        if !["json", "csv", "markdown"].contains(&format) {
            return Err(DbError::invalid());
        }
        let data = self.private_exchange().await?;
        let folder = self
            .root
            .join("exports")
            .join(format!("private-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&folder)?;
        let write = |name: &str, bytes: &[u8]| -> Result<()> {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(folder.join(name))?;
            f.write_all(bytes)?;
            f.sync_all()?;
            Ok(())
        };
        if format == "json" {
            write(
                "lumen-data-backup.json",
                &serde_json::to_vec_pretty(&data).map_err(|_| DbError::invalid())?,
            )?;
        } else {
            for (group, tables) in [
                ("Journal", vec!["journal_entries", "journal_tags"]),
                ("Focus", vec!["focus_sessions", "focus_intervals"]),
                ("Tasks", vec!["tasks"]),
                ("Music", vec!["music_tracks", "music_plays"]),
                ("Coding", vec!["coding_problems", "coding_attempts"]),
                ("Artifacts", vec!["artifacts"]),
            ] {
                let mut out = if format == "csv" {
                    "table,id,record_json\r\n".to_string()
                } else {
                    String::new()
                };
                for table in tables {
                    let rows = &data.tables[table];
                    if format == "markdown" {
                        out += &format!("# {table}\n\n");
                        for row in rows {
                            out += "```json\n";
                            out += &serde_json::to_string_pretty(row)
                                .map_err(|_| DbError::invalid())?;
                            out += "\n```\n\n";
                        }
                    } else {
                        for row in rows {
                            let json =
                                serde_json::to_string(row).map_err(|_| DbError::invalid())?;
                            out += &format!(
                                "\"{table}\",\"{}\",\"{}\"\r\n",
                                row.get("id")
                                    .or_else(|| row.get("session_id"))
                                    .and_then(Value::as_str)
                                    .unwrap_or_default(),
                                json.replace('"', "\"\"")
                            );
                        }
                    }
                }
                write(
                    &format!("{group}.{}", if format == "csv" { "csv" } else { "md" }),
                    out.as_bytes(),
                )?;
            }
        }
        write(
            "COMPLETE",
            format!("private export {}\n{}", now(), format).as_bytes(),
        )?;
        Ok(folder.to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::repository::{ContentKind, ContentRepository};
    #[tokio::test]
    async fn private_roundtrip_dry_run_staleness_invalid_and_exports() {
        let source = tempfile::tempdir().unwrap();
        let source_db = Database::open(source.path()).await.unwrap();
        let id = ContentRepository::new(&source_db, ContentKind::Task)
            .create("PRIVATE_IMPORT_FIXTURE", "private body", None)
            .await
            .unwrap();
        let focus = uuid::Uuid::new_v4().to_string();
        let time = now();
        sqlx::query("INSERT INTO focus_sessions(id,task_id,started_at,planned_seconds,created_at,updated_at,session_date,state) VALUES(?,?,?,60,?,?,'2026-10-10','finished')").bind(&focus).bind(&id).bind(&time).bind(&time).bind(&time).execute(&source_db.pool).await.unwrap();
        sqlx::query("INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms) VALUES(?,1,'2026-10-10',?,?,1000)").bind(&focus).bind(&time).bind(&time).execute(&source_db.pool).await.unwrap();
        let original = source_db.private_exchange().await.unwrap();
        let text = serde_json::to_string(&original).unwrap();
        let target = tempfile::tempdir().unwrap();
        let db = Database::open(target.path()).await.unwrap();
        let review = db.exchange_import(&text, None).await.unwrap();
        assert_eq!(review.add, 3);
        assert_eq!(review.invalid, 0);
        assert!(ContentRepository::new(&db, ContentKind::Task)
            .get(&id, true)
            .await
            .unwrap()
            .is_none());
        db.exchange_import(&text, Some(&review.fingerprint))
            .await
            .unwrap();
        let again = db.exchange_import(&text, None).await.unwrap();
        assert_eq!(again.add, 0);
        assert_eq!(again.unchanged, 3);
        assert_eq!(
            db.exchange_import(&text, Some(&review.fingerprint))
                .await
                .unwrap_err()
                .code,
            "IMPORT_STALE"
        );
        let mut bad = serde_json::to_value(&original).unwrap();
        bad["tables"]["tasks"][0]["id"] = "invalid".into();
        let bad_text = bad.to_string();
        let rejected = db.exchange_import(&bad_text, None).await.unwrap();
        assert!(rejected.invalid > 0);
        assert!(db
            .exchange_import(&bad_text, Some(&rejected.fingerprint))
            .await
            .is_err());
        for format in ["json", "csv", "markdown"] {
            let dir = db.private_export(format).await.unwrap();
            assert!(std::path::Path::new(&dir).join("COMPLETE").exists());
            if format != "json" {
                assert_eq!(std::fs::read_dir(dir).unwrap().count(), 7);
            }
        }
        assert!(db.private_export("../../db").await.is_err());
        source_db.pool.close().await;
        db.pool.close().await;
    }
}

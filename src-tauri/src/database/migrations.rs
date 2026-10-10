use super::{backup, checksum, now, Database, DbError, Result};
use sqlx::Row;

#[derive(Clone, Copy)]
pub(crate) struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}
pub(crate) const MIGRATIONS: [Migration; 10] = [
    Migration {
        version: 1,
        name: "001_initial.sql",
        sql: include_str!("../../migrations/001_initial.sql"),
    },
    Migration {
        version: 2,
        name: "002_today.sql",
        sql: include_str!("../../migrations/002_today.sql"),
    },
    Migration {
        version: 3,
        name: "003_focus_calendar.sql",
        sql: include_str!("../../migrations/003_focus_calendar.sql"),
    },
    Migration {
        version: 4,
        name: "004_roadmap_planner.sql",
        sql: include_str!("../../migrations/004_roadmap_planner.sql"),
    },
    Migration {
        version: 5,
        name: "005_journal_capture.sql",
        sql: include_str!("../../migrations/005_journal_capture.sql"),
    },
    Migration {
        version: 6,
        name: "006_coding.sql",
        sql: include_str!("../../migrations/006_coding.sql"),
    },
    Migration {
        version: 7,
        name: "007_insights.sql",
        sql: include_str!("../../migrations/007_insights.sql"),
    },
    Migration {
        version: 8,
        name: "008_architecture_evolution.sql",
        sql: include_str!("../../migrations/008_architecture_evolution.sql"),
    },
    Migration {
        version: 9,
        name: "009_music.sql",
        sql: include_str!("../../migrations/009_music.sql"),
    },
    Migration {
        version: 10,
        name: "010_vault.sql",
        sql: include_str!("../../migrations/010_vault.sql"),
    },
];

pub(crate) async fn apply(db: &Database, migrations: &[Migration]) -> Result<()> {
    let history_exists: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='migration_history'",
    )
    .fetch_one(&db.pool)
    .await?;
    let mut applied = 0;
    if history_exists == 1 {
        let rows =
            sqlx::query("SELECT version,name,checksum FROM migration_history ORDER BY version")
                .fetch_all(&db.pool)
                .await?;
        for row in rows {
            let version: i64 = row.get("version");
            let migration = migrations
                .iter()
                .find(|m| m.version == version)
                .ok_or_else(DbError::migration)?;
            if version != applied + 1
                || row.get::<String, _>("name") != migration.name
                || row.get::<String, _>("checksum") != checksum(migration.sql.as_bytes())
            {
                return Err(DbError::migration());
            }
            applied = version;
        }
    } else {
        let tables: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )
        .fetch_one(&db.pool)
        .await?;
        if tables != 0 {
            return Err(DbError::migration());
        }
    }
    for migration in migrations {
        if migration.version <= applied {
            continue;
        }
        if migration.version != applied + 1 {
            return Err(DbError::migration());
        }
        if history_exists == 1 || applied > 0 {
            backup::create(db).await?;
        }
        let mut transaction = db.pool.begin().await?;
        let outcome: std::result::Result<(), sqlx::Error> = async {
            sqlx::raw_sql(migration.sql)
                .execute(&mut *transaction)
                .await?;
            sqlx::query(
                "INSERT INTO migration_history(version,name,checksum,applied_at) VALUES(?,?,?,?)",
            )
            .bind(migration.version)
            .bind(migration.name)
            .bind(checksum(migration.sql.as_bytes()))
            .bind(now())
            .execute(&mut *transaction)
            .await?;
            Ok(())
        }
        .await;
        if outcome.is_err() {
            transaction.rollback().await?;
            return Err(DbError::new(
                "MIGRATION_FAILED",
                "迁移失败并已回滚；升级前备份已保留。",
            ));
        }
        transaction.commit().await?;
        applied = migration.version;
    }
    Ok(())
}

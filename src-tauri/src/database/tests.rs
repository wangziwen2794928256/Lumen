use super::{
    backup, checksum, connect,
    migrations::{self, Migration},
    repository::{search, ContentKind, ContentRepository},
    Database,
};
use sqlx::Row;

async fn fixture() -> (tempfile::TempDir, Database) {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    (folder, db)
}

#[tokio::test]
async fn vault_upgrade_preserves_legacy_artifact_and_private_default() {
    let folder = tempfile::tempdir().unwrap();
    crate::kernel::storage::initialize(folder.path()).unwrap();
    let db = Database {
        pool: connect(&folder.path().join("lumen.db"), true, false)
            .await
            .unwrap(),
        root: folder.path().to_path_buf(),
    };
    migrations::apply(&db, &migrations::MIGRATIONS[..9])
        .await
        .unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO artifacts(id,title,description,kind,created_at,updated_at) VALUES(?,'legacy','original','note',?,?)").bind(&id).bind(super::now()).bind(super::now()).execute(&db.pool).await.unwrap();
    migrations::apply(&db, &migrations::MIGRATIONS)
        .await
        .unwrap();
    migrations::apply(&db, &migrations::MIGRATIONS)
        .await
        .unwrap();
    let snapshot = db.vault_snapshot().await.unwrap();
    assert_eq!(snapshot["artifacts"][0]["title"], "legacy");
    assert_eq!(snapshot["artifacts"][0]["description"], "original");
    assert_eq!(snapshot["artifacts"][0]["kind"], "note");
    assert_eq!(snapshot["artifacts"][0]["visibility"], "private");
    assert_eq!(db.status().await.unwrap().schema_version, 10);
    assert_eq!(
        std::fs::read_dir(folder.path().join("backups"))
            .unwrap()
            .count(),
        9
    );
    db.pool.close().await;
}

#[tokio::test]
async fn phase5_upgrade_preserves_phase4_totals_and_marks_legacy_precision() {
    let folder = tempfile::tempdir().unwrap();
    std::fs::create_dir(folder.path().join("backups")).unwrap();
    let db = Database {
        pool: connect(&folder.path().join("lumen.db"), true, false)
            .await
            .unwrap(),
        root: folder.path().to_path_buf(),
    };
    migrations::apply(&db, &migrations::MIGRATIONS[..2])
        .await
        .unwrap();
    let task = uuid::Uuid::new_v4().to_string();
    let session = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO tasks(id,title,created_at,updated_at) VALUES(?,'legacy fixture',?,?)")
        .bind(&task)
        .bind(super::now())
        .bind(super::now())
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO focus_sessions(id,task_id,started_at,ended_at,planned_seconds,actual_seconds,elapsed_ms,state,last_activity_at,session_date,created_at,updated_at) VALUES(?,?,'2026-10-03T01:00:00.000Z','2026-10-03T02:00:00.000Z',1500,9,9000,'finished','2026-10-03T02:00:00.000Z','2026-10-03',?,?)").bind(&session).bind(&task).bind(super::now()).bind(super::now()).execute(&db.pool).await.unwrap();
    migrations::apply(&db, &migrations::MIGRATIONS)
        .await
        .unwrap();
    migrations::apply(&db, &migrations::MIGRATIONS)
        .await
        .unwrap();
    let saved: (i64, String, i64) = sqlx::query_as(
        "SELECT elapsed_ms,provenance,segment_no FROM focus_intervals WHERE session_id=?",
    )
    .bind(&session)
    .fetch_one(&db.pool)
    .await
    .unwrap();
    assert_eq!(saved, (9000, "legacy_total".into(), 0));
    let total: i64 = sqlx::query_scalar("SELECT actual_seconds FROM focus_sessions WHERE id=?")
        .bind(&session)
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(total, 9);
    assert_eq!(db.status().await.unwrap().schema_version, 10);
    assert_eq!(
        std::fs::read_dir(folder.path().join("backups"))
            .unwrap()
            .count(),
        9
    );
    db.pool.close().await;
}

#[tokio::test]
async fn initial_schema_reopens_without_seeds_and_checks_foreign_keys() {
    let (folder, db) = fixture().await;
    assert_eq!(db.status().await.unwrap().schema_version, 10);
    assert_eq!(db.status().await.unwrap().task_count, 0);
    let names: Vec<String> =
        sqlx::query_scalar("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(&db.pool)
            .await
            .unwrap();
    for name in [
        "users",
        "settings",
        "journeys",
        "journey_phases",
        "subjects",
        "tasks",
        "task_sessions",
        "calendar_events",
        "focus_sessions",
        "quests",
        "competitions",
        "coding_problems",
        "coding_attempts",
        "journal_entries",
        "journal_tags",
        "music_tracks",
        "music_plays",
        "artifacts",
        "artifact_files",
        "weekly_reviews",
        "monthly_snapshots",
        "exam_rule_snapshots",
        "daily_metrics",
        "public_visibility",
        "attachments",
        "integration_accounts",
        "migration_history",
        "content_search",
    ] {
        assert!(names.contains(&name.to_string()), "missing {name}");
    }
    let fk: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(fk, 1);
    assert!(sqlx::query(
        "INSERT INTO journal_tags(id,entry_id,name,created_at,updated_at) VALUES(?,?,?,?,?)"
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(uuid::Uuid::new_v4().to_string())
    .bind("fixture")
    .bind(super::now())
    .bind(super::now())
    .execute(&db.pool)
    .await
    .is_err());
    db.pool.close().await;
    let reopened = Database::open(folder.path()).await.unwrap();
    assert_eq!(reopened.status().await.unwrap().schema_version, 10);
    assert_eq!(
        std::fs::read_dir(folder.path().join("backups"))
            .unwrap()
            .count(),
        9
    );
    reopened.pool.close().await;
}

#[tokio::test]
async fn content_crud_parameterization_soft_delete_restore_and_fts() {
    let (_folder, db) = fixture().await;
    for kind in [ContentKind::Task, ContentKind::Journal] {
        let repo = ContentRepository::new(&db, kind);
        let title = "数学学习笔记 '); DROP TABLE tasks; --";
        let id = repo
            .create(title, "parameterized fixture", Some("2026-10-03"))
            .await
            .unwrap();
        let row = repo.get(&id, false).await.unwrap().unwrap();
        assert_eq!(row.title, title);
        assert_eq!(row.date.as_deref(), Some("2026-10-03"));
        assert!(row.created_at.ends_with('Z'));
        uuid::Uuid::parse_str(&row.id).unwrap();
        assert!(search(&db, "数学学习")
            .await
            .unwrap()
            .iter()
            .any(|hit| hit.id == id));
        repo.update(&id, "更新后的笔记", "updated fixture", Some("2026-10-04"))
            .await
            .unwrap();
        assert!(!search(&db, "数学学习")
            .await
            .unwrap()
            .iter()
            .any(|hit| hit.id == id));
        repo.set_deleted(&id, true).await.unwrap();
        assert!(repo.get(&id, false).await.unwrap().is_none());
        assert!(repo
            .get(&id, true)
            .await
            .unwrap()
            .unwrap()
            .deleted_at
            .is_some());
        assert!(!search(&db, "更新后的")
            .await
            .unwrap()
            .iter()
            .any(|hit| hit.id == id));
        assert_eq!(
            repo.update(&id, "refuse update trash", "", Some("2026-10-04"))
                .await
                .unwrap_err()
                .code,
            "NOT_FOUND"
        );
        repo.set_deleted(&id, false).await.unwrap();
        assert!(search(&db, "更新后的")
            .await
            .unwrap()
            .iter()
            .any(|hit| hit.id == id));
        assert_eq!(
            repo.get(&id, false).await.unwrap().unwrap().body,
            "updated fixture"
        );
    }
    // Treat FTS operators and quotes as literal input, never raw SQL/FTS grammar.
    assert!(search(&db, "\" OR * --").await.unwrap().is_empty());
    db.pool.close().await;
}

#[tokio::test]
async fn invalid_dates_uuid_and_empty_title_do_not_write() {
    let (_folder, db) = fixture().await;
    let repo = ContentRepository::new(&db, ContentKind::Journal);
    for date in [
        None,
        Some("2026-02-30"),
        Some("2026-1-1"),
        Some("2026-10-03T00:00:00Z"),
    ] {
        assert_eq!(
            repo.create("fixture", "", date).await.unwrap_err().code,
            "INVALID_INPUT"
        );
    }
    assert_eq!(
        repo.create("  ", "", Some("2026-10-03"))
            .await
            .unwrap_err()
            .code,
        "INVALID_INPUT"
    );
    assert_eq!(
        repo.get("bad-id", false).await.unwrap_err().code,
        "INVALID_INPUT"
    );
    assert_eq!(db.status().await.unwrap().journal_count, 0);
    db.pool.close().await;
}

#[tokio::test]
async fn failed_migration_rolls_back_and_pre_upgrade_backup_restores_rows() {
    let (folder, db) = fixture().await;
    let repo = ContentRepository::new(&db, ContentKind::Task);
    let id = repo
        .create("recovery fixture", "retained body", None)
        .await
        .unwrap();
    let previous_backups: Vec<_> = std::fs::read_dir(folder.path().join("backups"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    let broken = [
        Migration {
            version: 1,
            name: migrations::MIGRATIONS[0].name,
            sql: migrations::MIGRATIONS[0].sql,
        },
        Migration {
            version: 2,
            name: migrations::MIGRATIONS[1].name,
            sql: migrations::MIGRATIONS[1].sql,
        },
        migrations::MIGRATIONS[2],
        migrations::MIGRATIONS[3],
        migrations::MIGRATIONS[4],
        migrations::MIGRATIONS[5],
        migrations::MIGRATIONS[6],
        migrations::MIGRATIONS[7],
        migrations::MIGRATIONS[8],
        migrations::MIGRATIONS[9],
        Migration {
            version: 11,
            name: "011_broken.sql",
            sql: "CREATE TABLE partial_upgrade(id TEXT); INSERT INTO missing_table VALUES(1);",
        },
    ];
    assert_eq!(
        migrations::apply(&db, &broken).await.unwrap_err().code,
        "MIGRATION_FAILED"
    );
    let tables: i64 =
        sqlx::query_scalar("SELECT count(*) FROM sqlite_master WHERE name='partial_upgrade'")
            .fetch_one(&db.pool)
            .await
            .unwrap();
    assert_eq!(tables, 0);
    assert_eq!(db.status().await.unwrap().schema_version, 10);
    assert_eq!(
        repo.get(&id, false).await.unwrap().unwrap().body,
        "retained body"
    );
    let path = std::fs::read_dir(folder.path().join("backups"))
        .unwrap()
        .find(|entry| !previous_backups.contains(&entry.as_ref().unwrap().path()))
        .unwrap()
        .unwrap()
        .path();
    backup::verify(&path).await.unwrap();
    let restore = tempfile::tempdir().unwrap();
    std::fs::copy(path.join("lumen.db"), restore.path().join("lumen.db")).unwrap();
    let restored = Database::open(restore.path()).await.unwrap();
    assert_eq!(
        ContentRepository::new(&restored, ContentKind::Task)
            .get(&id, false)
            .await
            .unwrap()
            .unwrap()
            .title,
        "recovery fixture"
    );
    restored.pool.close().await;
    db.pool.close().await;
}

#[tokio::test]
async fn migration_checksum_drift_unknown_version_and_backup_failure_stop_upgrade() {
    let (folder, db) = fixture().await;
    let altered = [Migration {
        version: 1,
        name: "001_initial.sql",
        sql: "SELECT 1;",
    }];
    assert_eq!(
        migrations::apply(&db, &altered).await.unwrap_err().code,
        "MIGRATION_MISMATCH"
    );
    sqlx::query(
        "INSERT INTO migration_history(version,name,checksum,applied_at) VALUES(99,'unknown',?,?)",
    )
    .bind(checksum(b"unknown"))
    .bind(super::now())
    .execute(&db.pool)
    .await
    .unwrap();
    assert_eq!(
        migrations::apply(&db, &migrations::MIGRATIONS)
            .await
            .unwrap_err()
            .code,
        "MIGRATION_MISMATCH"
    );
    // Destructive fixture-only manipulation; never exposed through IPC.
    sqlx::query("DELETE FROM migration_history WHERE version=99")
        .execute(&db.pool)
        .await
        .unwrap();
    std::fs::rename(
        folder.path().join("backups"),
        folder.path().join("saved-fixture-backups"),
    )
    .unwrap();
    std::fs::write(
        folder.path().join("backups"),
        b"fixture blocks backup directory",
    )
    .unwrap();
    let upgrade = [
        Migration {
            version: 1,
            name: migrations::MIGRATIONS[0].name,
            sql: migrations::MIGRATIONS[0].sql,
        },
        Migration {
            version: 2,
            name: migrations::MIGRATIONS[1].name,
            sql: migrations::MIGRATIONS[1].sql,
        },
        migrations::MIGRATIONS[2],
        migrations::MIGRATIONS[3],
        migrations::MIGRATIONS[4],
        migrations::MIGRATIONS[5],
        migrations::MIGRATIONS[6],
        migrations::MIGRATIONS[7],
        migrations::MIGRATIONS[8],
        migrations::MIGRATIONS[9],
        Migration {
            version: 11,
            name: "011_new.sql",
            sql: "CREATE TABLE should_not_exist(id TEXT);",
        },
    ];
    assert_eq!(
        migrations::apply(&db, &upgrade).await.unwrap_err().code,
        "STORAGE_UNAVAILABLE"
    );
    assert_eq!(db.status().await.unwrap().schema_version, 10);
    db.pool.close().await;
}

#[tokio::test]
async fn wal_backup_is_consistent_and_tampering_is_rejected() {
    let (_folder, db) = fixture().await;
    sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,?,?,?,?)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind("fixture_setting")
        .bind("retained setting")
        .bind(super::now())
        .bind(super::now())
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES(?,?,?,?,?,?,?)")
        .bind(uuid::Uuid::new_v4().to_string()).bind("fixture.txt").bind("text/plain").bind(7)
        .bind(checksum(b"fixture")).bind(super::now()).bind(super::now()).execute(&db.pool).await.unwrap();
    let id = ContentRepository::new(&db, ContentKind::Journal)
        .create("backup fixture", "WAL row", Some("2026-10-03"))
        .await
        .unwrap();
    let backup_folder = backup::create(&db).await.unwrap();
    backup::verify(&backup_folder).await.unwrap();
    let snapshot = connect(&backup_folder.join("lumen.db"), false, true)
        .await
        .unwrap();
    let row = sqlx::query("SELECT body FROM journal_entries WHERE id=?")
        .bind(id)
        .fetch_one(&snapshot)
        .await
        .unwrap();
    assert_eq!(row.get::<String, _>("body"), "WAL row");
    let setting: String =
        sqlx::query_scalar("SELECT value_text FROM settings WHERE key='fixture_setting'")
            .fetch_one(&snapshot)
            .await
            .unwrap();
    assert_eq!(setting, "retained setting");
    snapshot.close().await;
    let target = backup_folder.join("lumen.db");
    let mut bytes = std::fs::read(&target).unwrap();
    bytes[100] ^= 0xff;
    std::fs::write(&target, bytes).unwrap();
    assert_eq!(
        backup::verify(&backup_folder).await.unwrap_err().code,
        "BACKUP_INVALID"
    );
    assert_eq!(db.status().await.unwrap().integrity, "ok");
    db.pool.close().await;
}

#[tokio::test]
async fn failed_initial_migration_leaves_empty_schema_and_successful_upgrade_is_idempotent() {
    let folder = tempfile::tempdir().unwrap();
    std::fs::create_dir(folder.path().join("backups")).unwrap();
    let db = Database {
        pool: connect(&folder.path().join("lumen.db"), true, false)
            .await
            .unwrap(),
        root: folder.path().to_path_buf(),
    };
    let broken = [Migration {
        version: 1,
        name: "001_invalid.sql",
        sql: "CREATE TABLE partial_initial(id TEXT); SELECT * FROM missing_initial;",
    }];
    assert_eq!(
        migrations::apply(&db, &broken).await.unwrap_err().code,
        "MIGRATION_FAILED"
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM sqlite_master WHERE type='table'")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    migrations::apply(&db, &migrations::MIGRATIONS)
        .await
        .unwrap();
    let upgraded = [
        Migration {
            version: 1,
            name: migrations::MIGRATIONS[0].name,
            sql: migrations::MIGRATIONS[0].sql,
        },
        Migration {
            version: 2,
            name: migrations::MIGRATIONS[1].name,
            sql: migrations::MIGRATIONS[1].sql,
        },
        migrations::MIGRATIONS[2],
        migrations::MIGRATIONS[3],
        migrations::MIGRATIONS[4],
        migrations::MIGRATIONS[5],
        migrations::MIGRATIONS[6],
        migrations::MIGRATIONS[7],
        migrations::MIGRATIONS[8],
        migrations::MIGRATIONS[9],
        Migration {
            version: 11,
            name: "011_fixture.sql",
            sql: "CREATE TABLE fixture_upgrade(id TEXT);",
        },
    ];
    migrations::apply(&db, &upgraded).await.unwrap();
    migrations::apply(&db, &upgraded).await.unwrap();
    assert_eq!(db.status().await.unwrap().schema_version, 11);
    assert_eq!(
        std::fs::read_dir(folder.path().join("backups"))
            .unwrap()
            .count(),
        10
    );
    db.pool.close().await;
}

#[tokio::test]
async fn corrupt_database_is_not_recreated_and_migration_lock_is_respected() {
    let folder = tempfile::tempdir().unwrap();
    let bytes = b"not a database - isolated fixture";
    std::fs::write(folder.path().join("lumen.db"), bytes).unwrap();
    assert!(Database::open(folder.path()).await.is_err());
    assert_eq!(
        std::fs::read(folder.path().join("lumen.db")).unwrap(),
        bytes
    );
    let locked = tempfile::tempdir().unwrap();
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(locked.path().join("migration.lock"))
        .unwrap();
    fs2::FileExt::lock_exclusive(&file).unwrap();
    let result = Database::open(locked.path()).await;
    assert!(matches!(result, Err(ref error) if error.code == "DATABASE_BUSY"));
}

#[tokio::test]
async fn complete_backup_copies_payloads_and_detects_tampering_without_live_writes() {
    let (root, db) = fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    let relative = format!("attachments/{id}.png");
    let bytes = b"isolated backup payload";
    std::fs::write(root.path().join(&relative), bytes).unwrap();
    sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES(?,?, 'image/png',?,?,?,?)")
        .bind(&id).bind(&relative).bind(bytes.len() as i64).bind(checksum(bytes)).bind(super::now()).bind(super::now()).execute(&db.pool).await.unwrap();
    sqlx::query("UPDATE attachments SET deleted_at=? WHERE id=?")
        .bind(super::now())
        .bind(&id)
        .execute(&db.pool)
        .await
        .unwrap();
    let folder = backup::create_complete(&db).await.unwrap();
    backup::verify_complete(&folder).await.unwrap();
    assert_eq!(std::fs::read(folder.join(&relative)).unwrap(), bytes);
    std::fs::write(folder.join(&relative), b"tampered").unwrap();
    assert!(backup::verify_complete(&folder).await.is_err());
    assert_eq!(std::fs::read(root.path().join(&relative)).unwrap(), bytes);
    let legacy = backup::create(&db).await.unwrap();
    backup::verify(&legacy).await.unwrap();
    assert!(backup::verify_complete(&legacy).await.is_err());
    db.pool.close().await;
}

#[tokio::test]
async fn complete_backup_fails_closed_on_missing_payload_or_path_escape() {
    let (root, db) = fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES(?, 'attachments/missing.png', 'image/png',0,?,?,?)")
        .bind(&id).bind(checksum(b"")).bind(super::now()).bind(super::now()).execute(&db.pool).await.unwrap();
    assert!(backup::create_complete(&db).await.is_err());
    sqlx::query("UPDATE attachments SET relative_path='attachments/../outside.png'")
        .execute(&db.pool)
        .await
        .unwrap();
    assert!(backup::create_complete(&db).await.is_err());
    assert!(std::fs::read_dir(root.path().join("backups"))
        .unwrap()
        .all(|entry| !entry.unwrap().path().join("COMPLETE").exists()));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM attachments")
            .fetch_one(&db.pool)
            .await
            .unwrap(),
        1
    );
    db.pool.close().await;
}

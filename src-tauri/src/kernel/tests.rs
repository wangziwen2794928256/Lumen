use super::*;
use crate::database::{
    repository::{ContentKind, ContentRepository},
    Database,
};
#[tokio::test]
async fn module_registry_validation_revision_disable_and_reopen() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let mut value = db.module_registry().await.unwrap();
    assert_eq!(value.items.len(), modules::catalogue().len());
    let initial = value.clone();
    value
        .items
        .iter_mut()
        .find(|i| i.id == "lumen.insights")
        .unwrap()
        .enabled = false;
    let saved = db.module_save(value).await.unwrap();
    assert_eq!(saved.revision, 1);
    assert_eq!(
        db.require_module("lumen.insights").await.unwrap_err().code,
        "MODULE_DISABLED"
    );
    assert_eq!(
        db.module_save(initial).await.unwrap_err().code,
        "MODULE_CONFLICT"
    );
    let mut invalid = saved.clone();
    invalid.items[0].enabled = false;
    assert!(db.module_save(invalid).await.is_err());
    let mut invalid = saved.clone();
    invalid.items[0].id = "community.escape".into();
    assert!(db.module_save(invalid).await.is_err());
    db.pool.close().await;
    let reopened = Database::open(root.path()).await.unwrap();
    assert!(
        !reopened
            .module_registry()
            .await
            .unwrap()
            .items
            .iter()
            .find(|i| i.id == "lumen.insights")
            .unwrap()
            .enabled
    );
    reopened.pool.close().await;
}
#[test]
fn plugin_permissions_reject_ambient_authority_and_runtime_remains_closed() {
    let mut m = permissions::CommunityManifest {
        id: "org.example".into(),
        version: "1.0.0".into(),
        api_version: "1.0".into(),
        runtime: "wasm-component".into(),
        entry: "plugin.wasm".into(),
        permissions: vec!["storage.own".into()],
    };
    permissions::validate_manifest(&m).unwrap();
    assert_eq!(
        permissions::authorize_community_execution(&m)
            .unwrap_err()
            .code,
        "PLUGIN_RUNTIME_DEFERRED"
    );
    for denied in [
        "core.sqlite",
        "filesystem.any",
        "network.any",
        "native.execute",
        "kernel.uninstall",
    ] {
        m.permissions = vec![denied.into()];
        assert!(permissions::validate_manifest(&m).is_err());
    }
    m.permissions = vec![];
    for id in [
        "lumen.security",
        "../escape",
        "x/../../a",
        "COM1",
        "org..empty",
    ] {
        m.id = id.into();
        assert!(permissions::validate_manifest(&m).is_err());
    }
    m.id = "org.example".into();
    m.api_version = "2.0".into();
    assert!(permissions::validate_manifest(&m).is_err());
    assert_eq!(
        permissions::transition(permissions::Lifecycle::Staged, "install").unwrap(),
        permissions::Lifecycle::Disabled
    );
    assert!(permissions::transition(permissions::Lifecycle::Disabled, "enable").is_err());
    assert!(permissions::transition(permissions::Lifecycle::Enabled, "uninstall").is_err());
    assert_eq!(
        permissions::transition(permissions::Lifecycle::UpdateStaged, "rollback").unwrap(),
        permissions::Lifecycle::Disabled
    );
}
#[test]
fn plugin_namespace_and_platform_boundaries() {
    let root = tempfile::tempdir().unwrap();
    storage::initialize(root.path()).unwrap();
    let ns = root.path().join("plugins/org.example");
    std::fs::create_dir(&ns).unwrap();
    assert_eq!(
        storage::plugin_namespace(root.path(), "org.example").unwrap(),
        ns.canonicalize().unwrap()
    );
    assert!(storage::plugin_namespace(root.path(), "../lumen.db").is_err());
    assert!(storage::plugin_namespace(root.path(), "lumen.security").is_err());
    assert_eq!(
        storage::core_database(root.path()),
        root.path().join("lumen.db")
    );
    assert_eq!(platform::macos::ADAPTER.name, "macos");
    const { assert!(!platform::macos::ADAPTER.session_events) };
    const { assert!(!platform::linux::ADAPTER.attachment_viewer) };
}
#[test]
fn plan_exchange_is_bounded_and_data_only() {
    let mut p = plan::PlanExchange {
        schema_version: "1.0".into(),
        request_id: uuid::Uuid::new_v4().to_string(),
        basis_hash: "a".repeat(64),
        source: "external-proposal".into(),
        capacity_minutes: 30,
        items: vec![plan::PlanItem {
            task_id: uuid::Uuid::new_v4().to_string(),
            date: "2026-10-04".into(),
            minutes: 25,
        }],
    };
    plan::validate_plan(&p).unwrap();
    p.capacity_minutes = 20;
    assert!(plan::validate_plan(&p).is_err());
    p.capacity_minutes = 30;
    p.items[0].date = "2026-02-29".into();
    assert!(plan::validate_plan(&p).is_err());
}
#[tokio::test]
async fn planner_observations_preserve_estimates_choices_and_reject_mutation() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let id = ContentRepository::new(&db, ContentKind::Task)
        .create("fixture", "", Some("2026-10-04"))
        .await
        .unwrap();
    sqlx::query("UPDATE tasks SET estimated_minutes=30 WHERE id=?")
        .bind(&id)
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query("UPDATE tasks SET estimated_minutes=45,due_date='2026-10-05' WHERE id=?")
        .bind(&id)
        .execute(&db.pool)
        .await
        .unwrap();
    let values:Vec<String>=sqlx::query_scalar("SELECT payload FROM planner_observations WHERE kind='task' AND entity_id=? ORDER BY sequence").bind(&id).fetch_all(&db.pool).await.unwrap();
    assert!(values
        .iter()
        .any(|v| v.contains("\"estimate_before\":30") && v.contains("\"estimate_after\":45")));
    let decision = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO planning_decisions(id,week_start,status,reason,before_json,after_json,basis_hash,created_at,updated_at) VALUES(?,'2026-09-28','proposed','','{}','{}',?,datetime('now'),datetime('now'))").bind(&decision).bind("a".repeat(64)).execute(&db.pool).await.unwrap();
    sqlx::query("UPDATE planning_decisions SET status='rejected' WHERE id=?")
        .bind(decision)
        .execute(&db.pool)
        .await
        .unwrap();
    let last:String=sqlx::query_scalar("SELECT payload FROM planner_observations WHERE kind='decision' ORDER BY sequence DESC LIMIT 1").fetch_one(&db.pool).await.unwrap();
    assert!(last.contains("rejected"));
    let focus = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO focus_sessions(id,task_id,started_at,planned_seconds,actual_seconds,elapsed_ms,state,last_activity_at,session_date,created_at,updated_at) VALUES(?,?,'2026-10-04T01:00:00Z',1500,0,0,'running','2026-10-04T01:00:00Z','2026-10-04','2026-10-04T01:00:00Z','2026-10-04T01:00:00Z')").bind(&focus).bind(&id).execute(&db.pool).await.unwrap();
    sqlx::query(
        "UPDATE focus_sessions SET state='finished',actual_seconds=26,elapsed_ms=26914 WHERE id=?",
    )
    .bind(focus)
    .execute(&db.pool)
    .await
    .unwrap();
    let observed:String=sqlx::query_scalar("SELECT payload FROM planner_observations WHERE kind='focus' ORDER BY sequence DESC LIMIT 1").fetch_one(&db.pool).await.unwrap();
    assert!(
        observed.contains("\"elapsed_ms\":26914") && observed.contains("\"planned_seconds\":1500")
    );
    sqlx::query("INSERT INTO capacity_days(date,minutes,status,updated_at) VALUES('2026-10-04',210,'normal','2026-10-04T01:00:00Z')").execute(&db.pool).await.unwrap();
    sqlx::query("UPDATE capacity_days SET minutes=120,status='rest' WHERE date='2026-10-04'")
        .execute(&db.pool)
        .await
        .unwrap();
    let capacity:String=sqlx::query_scalar("SELECT payload FROM planner_observations WHERE kind='capacity' ORDER BY sequence DESC LIMIT 1").fetch_one(&db.pool).await.unwrap();
    assert!(
        capacity.contains("\"minutes_before\":210") && capacity.contains("\"minutes_after\":120")
    );
    assert!(sqlx::query("DELETE FROM planner_observations")
        .execute(&db.pool)
        .await
        .is_err());
    assert!(
        sqlx::query("UPDATE planner_observations SET event='tampered'")
            .execute(&db.pool)
            .await
            .is_err()
    );
    db.pool.close().await;
}
#[test]
fn capability_grants_are_bound_to_identity_version_hash_and_revocation() {
    let m = permissions::CommunityManifest {
        id: "org.example".into(),
        version: "1.0.0".into(),
        api_version: "1.0".into(),
        runtime: "wasm-component".into(),
        entry: "plugin.wasm".into(),
        permissions: vec!["storage.own".into()],
    };
    let mut g = permissions::Grant {
        plugin_id: m.id.clone(),
        version: m.version.clone(),
        content_hash: "a".repeat(64),
        capabilities: vec!["storage.own".into()],
        revoked: false,
    };
    permissions::check_capability(&m, &g, &"a".repeat(64), "storage.own").unwrap();
    assert!(permissions::check_capability(&m, &g, &"b".repeat(64), "storage.own").is_err());
    assert!(permissions::check_capability(&m, &g, &"a".repeat(64), "core.sqlite").is_err());
    g.revoked = true;
    assert!(permissions::check_capability(&m, &g, &"a".repeat(64), "storage.own").is_err());
}

use super::*;
use crate::kernel::platform::media::Sample;
fn save(request: String, id: Option<String>, revision: i64, journal: Option<String>) -> Action {
    Action::Save {
        request_id: request,
        id,
        revision,
        title: "Music fixture".into(),
        artist: "Artist fixture".into(),
        album: "Album fixture".into(),
        platform: "Manual".into(),
        played_at: "2026-10-04T04:00:00Z".into(),
        duration_seconds: 180,
        journal_id: journal,
    }
}
#[tokio::test]
async fn manual_receipts_revision_trash_restore_and_private_totals() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    assert!(!db.music_preferences().await.unwrap().enabled);
    let request = uuid::Uuid::new_v4().to_string();
    db.music_action(save(request.clone(), None, 0, None))
        .await
        .unwrap();
    db.music_action(save(request, None, 0, None)).await.unwrap();
    let s = db
        .music_snapshot("2026-10-04".into(), "2026-10-04".into(), false)
        .await
        .unwrap();
    assert_eq!(s["totals"]["count"], 1);
    assert_eq!(s["totals"]["seconds"], 180);
    assert_eq!(s["visibility"], "private");
    let id = s["items"][0]["id"].as_str().unwrap().to_owned();
    db.music_action(save(
        uuid::Uuid::new_v4().to_string(),
        Some(id.clone()),
        1,
        None,
    ))
    .await
    .unwrap();
    assert!(db
        .music_action(save(
            uuid::Uuid::new_v4().to_string(),
            Some(id.clone()),
            1,
            None
        ))
        .await
        .is_err());
    db.music_action(Action::Trash {
        request_id: uuid::Uuid::new_v4().to_string(),
        id: id.clone(),
        revision: 2,
        restore: false,
    })
    .await
    .unwrap();
    assert_eq!(
        db.music_snapshot("2026-10-04".into(), "2026-10-04".into(), false)
            .await
            .unwrap()["totals"]["count"],
        0
    );
    db.music_action(Action::Trash {
        request_id: uuid::Uuid::new_v4().to_string(),
        id,
        revision: 3,
        restore: true,
    })
    .await
    .unwrap();
    db.pool.close().await;
    let db = Database::open(folder.path()).await.unwrap();
    assert_eq!(
        db.music_snapshot("2026-10-04".into(), "2026-10-04".into(), false)
            .await
            .unwrap()["totals"]["count"],
        1
    );
    db.pool.close().await;
}
#[tokio::test]
async fn media_event_updates_once_and_never_resurrects_trash() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let mut r = tracker::Recording {
        sample: Sample {
            title: "Fixture".into(),
            artist: "A".into(),
            album: "B".into(),
            app: "fixture.player".into(),
            position_ms: 30_000,
            playing: true,
        },
        event: uuid::Uuid::new_v4().to_string(),
        seconds: 30,
        started_at: "2026-10-04T04:00:00Z".into(),
    };
    db.record_media(&r).await.unwrap();
    r.seconds = 40;
    db.record_media(&r).await.unwrap();
    db.record_media(&r).await.unwrap();
    let s = db
        .music_snapshot("2026-10-04".into(), "2026-10-04".into(), false)
        .await
        .unwrap();
    assert_eq!(s["totals"]["count"], 1);
    assert_eq!(s["totals"]["seconds"], 40);
    assert_eq!(s["items"][0]["source"], "windows-media");
    db.music_action(Action::Trash {
        request_id: uuid::Uuid::new_v4().to_string(),
        id: s["items"][0]["id"].as_str().unwrap().into(),
        revision: 3,
        restore: false,
    })
    .await
    .unwrap();
    r.seconds = 60;
    db.record_media(&r).await.unwrap();
    assert_eq!(
        db.music_snapshot("2026-10-04".into(), "2026-10-04".into(), false)
            .await
            .unwrap()["totals"]["count"],
        0
    );
    db.pool.close().await;
}
#[tokio::test]
async fn preferences_and_journal_links_fail_closed() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    assert!(db
        .music_action(Action::Preferences {
            request_id: uuid::Uuid::new_v4().to_string(),
            preferences: Preferences {
                enabled: true,
                threshold_seconds: 30,
                allowed_app: String::new(),
                revision: 0
            }
        })
        .await
        .is_err());
    assert!(db
        .music_action(save(
            uuid::Uuid::new_v4().to_string(),
            None,
            0,
            Some(uuid::Uuid::new_v4().to_string())
        ))
        .await
        .is_err());
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO journal_entries(id,title,body,entry_date,created_at,updated_at) VALUES(?,'Music note fixture','','2026-10-04',?,?)").bind(&id).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    db.music_action(save(uuid::Uuid::new_v4().to_string(), None, 0, Some(id)))
        .await
        .unwrap();
    let s = db
        .music_snapshot("2026-10-04".into(), "2026-10-04".into(), false)
        .await
        .unwrap();
    assert_eq!(s["items"][0]["journal_title"], "Music note fixture");
    let p = Preferences {
        enabled: true,
        threshold_seconds: 5,
        allowed_app: "fixture.player".into(),
        revision: 0,
    };
    db.music_action(Action::Preferences {
        request_id: uuid::Uuid::new_v4().to_string(),
        preferences: p.clone(),
    })
    .await
    .unwrap();
    assert!(db
        .music_action(Action::Preferences {
            request_id: uuid::Uuid::new_v4().to_string(),
            preferences: p
        })
        .await
        .is_err());
    assert!(db
        .music_snapshot("2025-01-01".into(), "2026-10-04".into(), false)
        .await
        .is_err());
    db.pool.close().await;
}
#[tokio::test]
async fn old_registry_grows_without_changing_saved_preferences() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let mut old = crate::kernel::modules::Registry::default();
    old.items.retain(|m| m.id != "lumen.music");
    old.items[1].enabled = false;
    old.items[2].pinned = true;
    old.revision = 7;
    let payload = serde_json::to_string(&old).unwrap();
    sqlx::query("INSERT INTO module_preferences VALUES(1,?,7,?)")
        .bind(&payload)
        .bind(now())
        .execute(&db.pool)
        .await
        .unwrap();
    let current = db.module_registry().await.unwrap();
    assert_eq!(current.items.len(), 10);
    assert_eq!(current.revision, 7);
    assert!(!current.items[1].enabled);
    assert!(current.items[2].pinned);
    assert!(
        current
            .items
            .iter()
            .find(|m| m.id == "lumen.music")
            .unwrap()
            .enabled
    );
    let stored: String = sqlx::query_scalar("SELECT payload FROM module_preferences")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    assert_eq!(stored, payload);
    db.pool.close().await;
}

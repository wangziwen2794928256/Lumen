use super::*;
use crate::vault::{Action, Artifact};
fn artifact(visibility: &str, title: &str) -> Artifact {
    Artifact {
        id: uuid::Uuid::new_v4().to_string(),
        title: title.into(),
        description: "Explicit public body <script>alert(1)</script>".into(),
        kind: "project".into(),
        artifact_date: None,
        role: String::new(),
        organization: String::new(),
        skills: vec!["Rust".into()],
        technologies: vec![],
        repository_url: "https://example.org/repo".into(),
        project_url: String::new(),
        paper_url: String::new(),
        journey_id: None,
        competition_id: None,
        visibility: visibility.into(),
        revision: 0,
        archived_at: None,
        deleted_at: None,
    }
}
async fn save(db: &Database, a: Artifact) {
    db.vault_action(Action::Save {
        request_id: uuid::Uuid::new_v4().to_string(),
        artifact: Box::new(a),
    })
    .await
    .unwrap();
}
#[tokio::test]
async fn whitelist_private_sentinels_and_standalone_export() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    assert!(!db.profile_get().await.unwrap().public);
    let profile = Profile {
        display_name: "PRIVATE_NAME".into(),
        about: "PRIVATE_BIO".into(),
        links: vec![Link {
            label: "PRIVATE_CONTACT".into(),
            url: "https://example.org/private".into(),
        }],
        ..Default::default()
    };
    db.profile_save(profile).await.unwrap();
    save(&db, artifact("private", "PRIVATE_ARTIFACT")).await;
    let live = artifact("public", "Public work");
    let live_id = live.id.clone();
    save(&db, live).await;
    let journey = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO journeys(id,title,description,created_at,updated_at) VALUES(?,'PRIVATE_JOURNEY','PRIVATE_ASSOCIATION',?,?)").bind(&journey).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    sqlx::query("UPDATE artifacts SET journey_id=? WHERE id=?")
        .bind(&journey)
        .bind(&live_id)
        .execute(&db.pool)
        .await
        .unwrap();
    let attachment = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO attachments(id,relative_path,mime_type,size_bytes,sha256,created_at,updated_at) VALUES(?,'attachments/PRIVATE_FILE.pdf','application/pdf',1,?,?,?)").bind(&attachment).bind("a".repeat(64)).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    sqlx::query("INSERT INTO artifact_files(id,artifact_id,attachment_id,created_at,updated_at,name) VALUES(?,?,?,?,?,'PRIVATE_FILENAME')").bind(uuid::Uuid::new_v4().to_string()).bind(&live_id).bind(attachment).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    for state in ["archive", "trash"] {
        let a = artifact("public", &format!("EXCLUDED_{state}"));
        let id = a.id.clone();
        save(&db, a).await;
        db.vault_action(Action::State {
            request_id: uuid::Uuid::new_v4().to_string(),
            id,
            revision: 1,
            state: state.into(),
        })
        .await
        .unwrap();
    }
    sqlx::query("INSERT INTO journal_entries(id,title,body,entry_date,is_draft,created_at,updated_at) VALUES(?,'PRIVATE_JOURNAL','PRIVATE_MOOD_CAPTURE','2026-10-08',0,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    db.music_action(crate::music::Action::Save {
        request_id: uuid::Uuid::new_v4().to_string(),
        id: None,
        revision: 0,
        title: "PRIVATE_SONG".into(),
        artist: "PRIVATE_ARTIST".into(),
        album: String::new(),
        platform: "Manual".into(),
        played_at: "2026-10-08T00:00:00Z".into(),
        duration_seconds: 180,
        journal_id: None,
    })
    .await
    .unwrap();
    sqlx::query("INSERT INTO focus_sessions(id,started_at,planned_seconds,created_at,updated_at) VALUES(?,'2026-10-08T00:00:00Z',60,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    sqlx::query("INSERT INTO settings(id,key,value_text,created_at,updated_at) VALUES(?,'secret','PRIVATE_SETTINGS',?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    let result = db.public_projection().await.unwrap();
    assert_eq!(
        result.require_review("stale").unwrap_err().code,
        "PUBLIC_REVIEW_STALE"
    );
    assert!(db.public_export("stale").await.is_err());
    assert!(result.profile.is_none());
    assert_eq!(result.artifacts.len(), 1);
    assert_eq!(result.artifacts[0].id, live_id);
    let json = serde_json::to_string(&result).unwrap();
    for marker in [
        "PRIVATE_",
        "EXCLUDED_",
        "journey_id",
        "competition_id",
        "attachments",
        "focus_sessions",
        "lumen.db",
        "revision",
    ] {
        assert!(!json.contains(marker), "{marker}");
    }
    let receipt = db
        .public_export(&result.review_hash().unwrap())
        .await
        .unwrap();
    let again = db
        .public_export(&result.review_hash().unwrap())
        .await
        .unwrap();
    assert_ne!(receipt.id, again.id);
    let folder = std::path::Path::new(&receipt.directory);
    let bytes = std::fs::read(folder.join("public-profile.json")).unwrap();
    assert_eq!(crate::database::checksum(&bytes), receipt.sha256);
    db.pool.close().await; // Fixture only: detach website and remove its test database.
    let detached = tempfile::tempdir().unwrap();
    for (name, _) in assets(&bytes) {
        std::fs::copy(folder.join(name), detached.path().join(name)).unwrap();
    }
    std::fs::remove_file(root.path().join("lumen.db")).unwrap();
    assert!(!detached.path().join("lumen.db").exists());
    assert_eq!(
        std::fs::read(detached.path().join("public-profile.json")).unwrap(),
        bytes
    );
    for file in ["index.html", "site.js", "site.css"] {
        assert!(detached.path().join(file).is_file());
    }
}
#[tokio::test]
async fn explicit_public_identity_conflict_and_revocation() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let p = Profile {
        display_name: "Public identity".into(),
        public: true,
        ..Default::default()
    };
    db.profile_save(p.clone()).await.unwrap();
    let review = db.public_projection().await.unwrap().review_hash().unwrap();
    assert!(db.public_projection().await.unwrap().profile.is_some());
    assert_eq!(db.profile_save(p).await.unwrap_err().code, "CONFLICT");
    let mut p = db.profile_get().await.unwrap();
    p.public = false;
    db.profile_save(p).await.unwrap();
    assert!(db.public_export(&review).await.is_err());
    assert!(db.public_projection().await.unwrap().profile.is_none());
    db.pool.close().await;
    let db = Database::open(folder.path()).await.unwrap();
    assert!(!db.profile_get().await.unwrap().public);
    assert_eq!(db.status().await.unwrap().schema_version, 10);
}
#[test]
fn unsafe_contact_and_unknown_fields_rejected() {
    for value in [
        "javascript:alert(1)",
        "file:///secret",
        "https://user:pass@example.org",
        "https://example.org/\n",
    ] {
        assert!(validate_url(value).is_err());
    }
    assert!(serde_json::from_str::<Profile>(r#"{"private_sql":"SELECT *"}"#).is_err());
}

use super::*;
fn artifact() -> Artifact {
    Artifact {
        id: uuid::Uuid::new_v4().to_string(),
        title: "真实测试作品".into(),
        description: "用户原文".into(),
        kind: "project".into(),
        artifact_date: Some("2026-10-04".into()),
        role: "作者".into(),
        organization: String::new(),
        skills: vec!["Rust".into()],
        technologies: vec![],
        repository_url: String::new(),
        project_url: String::new(),
        paper_url: String::new(),
        journey_id: None,
        competition_id: None,
        visibility: "private".into(),
        revision: 0,
        archived_at: None,
        deleted_at: None,
    }
}
#[tokio::test]
async fn crud_receipts_conflicts_visibility_and_restore() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let mut a = artifact();
    let request = uuid::Uuid::new_v4().to_string();
    for _ in 0..2 {
        db.vault_action(Action::Save {
            request_id: request.clone(),
            artifact: Box::new(a.clone()),
        })
        .await
        .unwrap();
    }
    let data = db.vault_snapshot().await.unwrap();
    assert_eq!(data["artifacts"].as_array().unwrap().len(), 1);
    assert_eq!(data["artifacts"][0]["visibility"], "private");
    assert_eq!(data["artifacts"][0]["revision"], 1);
    assert_eq!(
        db.vault_action(Action::Save {
            request_id: uuid::Uuid::new_v4().to_string(),
            artifact: Box::new(a.clone())
        })
        .await
        .unwrap_err()
        .code,
        "CONFLICT"
    );
    a.revision = 1;
    a.visibility = "public".into();
    db.vault_action(Action::Save {
        request_id: uuid::Uuid::new_v4().to_string(),
        artifact: Box::new(a.clone()),
    })
    .await
    .unwrap();
    for (revision, state) in [(2, "archive"), (3, "restore"), (4, "trash"), (5, "restore")] {
        db.vault_action(Action::State {
            request_id: uuid::Uuid::new_v4().to_string(),
            id: a.id.clone(),
            revision,
            state: state.into(),
        })
        .await
        .unwrap();
    }
    let data = db.vault_snapshot().await.unwrap();
    assert_eq!(data["artifacts"][0]["revision"], 6);
    assert_eq!(data["artifacts"][0]["description"], "用户原文");
    assert_eq!(data["artifacts"][0]["visibility"], "public");
    db.pool.close().await;
}
#[tokio::test]
async fn validation_links_and_file_manifest_integrity() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let mut a = artifact();
    a.repository_url = "javascript:alert(1)".into();
    assert!(validate(&a).is_err());
    a.repository_url = "https://user:password@example.com".into();
    assert!(validate(&a).is_err());
    a.repository_url = String::new();
    a.artifact_date = Some("2026-02-30".into());
    assert!(validate(&a).is_err());
    a.artifact_date = None;
    a.journey_id = Some(uuid::Uuid::new_v4().to_string());
    assert!(db
        .vault_action(Action::Save {
            request_id: uuid::Uuid::new_v4().to_string(),
            artifact: Box::new(a.clone())
        })
        .await
        .is_err());
    a.journey_id = None;
    db.vault_action(Action::Save {
        request_id: uuid::Uuid::new_v4().to_string(),
        artifact: Box::new(a.clone()),
    })
    .await
    .unwrap();
    let source = folder.path().join("fixture.pdf");
    std::fs::write(&source, b"%PDF-1.7 isolated fixture").unwrap();
    db.vault_import(&a.id, 1, &source).await.unwrap();
    assert!(db.vault_import(&a.id, 1, &source).await.is_err());
    let files = db.vault_files(&a.id).await.unwrap();
    assert_eq!(files.as_array().unwrap().len(), 1);
    let id = files[0]["id"].as_str().unwrap();
    let (status, path) = db.vault_verify(id).await.unwrap();
    assert_eq!(status, "verified");
    assert_eq!(files[0]["sha256"].as_str().unwrap().len(), 64);
    let path = path.unwrap();
    std::fs::write(&path, b"changed").unwrap();
    assert_eq!(db.vault_verify(id).await.unwrap().0, "changed");
    std::fs::remove_file(path).unwrap();
    assert_eq!(db.vault_verify(id).await.unwrap().0, "missing");
    sqlx::query("UPDATE attachments SET relative_path='../fixture.pdf'")
        .execute(&db.pool)
        .await
        .unwrap();
    assert!(db.vault_verify(id).await.is_err());
    db.pool.close().await;
}

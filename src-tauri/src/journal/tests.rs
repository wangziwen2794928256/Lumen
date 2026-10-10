use super::*;
fn draft() -> Save {
    Save {
        id: uuid::Uuid::new_v4().to_string(),
        client_id: uuid::Uuid::new_v4().to_string(),
        sequence: 1,
        revision: 0,
        title: "测试草稿".into(),
        body: "# 标题\n崩溃安全自动保存".into(),
        date: "2026-10-03".into(),
        entry_type: "study_note".into(),
        tags: vec!["学习".into()],
        journey_id: None,
        task_id: None,
        music_id: None,
        publish: false,
    }
}
#[tokio::test]
async fn autosave_restart_idempotence_conflict_and_recovery() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let first = draft();
    let entry = db.journal_save(first.clone()).await.unwrap();
    assert!(entry.is_draft);
    assert_eq!(entry.revision, 1);
    assert_eq!(db.journal_save(first.clone()).await.unwrap().revision, 1);
    let mut wrong = first.clone();
    wrong.sequence = 2;
    wrong.body = "冲突窗口的完整草稿".into();
    assert_eq!(
        db.journal_save(wrong.clone()).await.unwrap_err().code,
        "JOURNAL_CONFLICT"
    );
    assert_eq!(db.journal_get(&first.id).await.unwrap().body, first.body);
    db.pool.close().await;
    let db = Database::open(root.path()).await.unwrap();
    let recovered = db.journal_page(Filter::default()).await.unwrap();
    assert_eq!(recovered.recoveries[0].payload.body, wrong.body);
    let mut fixed = wrong;
    fixed.sequence = 3;
    fixed.revision = 1;
    fixed.publish = true;
    let entry = db.journal_save(fixed).await.unwrap();
    assert!(!entry.is_draft);
    assert_eq!(entry.revision, 2);
    assert!(db
        .journal_page(Filter::default())
        .await
        .unwrap()
        .recoveries
        .is_empty());
    db.journal_trash(&entry.id, entry.revision, false)
        .await
        .unwrap();
    assert_eq!(db.journal_page(Filter::default()).await.unwrap().total, 0);
    assert_eq!(
        db.journal_trash(&entry.id, entry.revision, false)
            .await
            .unwrap_err()
            .code,
        "JOURNAL_CONFLICT"
    );
    db.journal_trash(&entry.id, entry.revision + 1, true)
        .await
        .unwrap();
    assert_eq!(db.journal_page(Filter::default()).await.unwrap().total, 1);
}
#[tokio::test]
async fn chinese_fts_filters_pagination_and_invalid_input() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    for i in 0..35 {
        let mut input = draft();
        input.title = format!("记录 {i}");
        input.publish = true;
        db.journal_save(input).await.unwrap();
    }
    let filter = Filter {
        query: "安全自动".into(),
        tag: "学习".into(),
        from: "2026-10-03".into(),
        until: "2026-10-03".into(),
        entry_type: "study_note".into(),
        ..Default::default()
    };
    let page = db.journal_page(filter.clone()).await.unwrap();
    assert_eq!(page.total, 35);
    assert_eq!(page.entries.len(), 30);
    let next = db.journal_page(Filter { page: 1, ..filter }).await.unwrap();
    assert_eq!(next.entries.len(), 5);
    assert!(!page
        .entries
        .iter()
        .any(|e| next.entries.iter().any(|n| n.id == e.id)));
    assert_eq!(
        db.journal_page(Filter {
            query: "\" OR *".into(),
            ..Default::default()
        })
        .await
        .unwrap()
        .total,
        0
    );
    let mut bad = draft();
    bad.date = "2026-02-30".into();
    assert!(db.journal_save(bad).await.is_err());
    let mut bad = draft();
    bad.task_id = Some(uuid::Uuid::new_v4().to_string());
    assert!(db.journal_save(bad).await.is_err());
}
#[tokio::test]
async fn controlled_attachments_hash_missing_changed_and_links() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let entry = db.journal_save(draft()).await.unwrap();
    let source = root.path().join("fixture.pdf");
    std::fs::write(&source, b"%PDF-1.7\nfixture").unwrap();
    db.journal_import(&entry.id, &source).await.unwrap();
    let a = db.journal_attachments(&entry.id).await.unwrap();
    assert_eq!(a.len(), 1);
    assert_eq!(db.journal_verify(&a[0].id).await.unwrap(), "verified");
    let path: String = sqlx::query_scalar("SELECT relative_path FROM attachments LIMIT 1")
        .fetch_one(&db.pool)
        .await
        .unwrap();
    std::fs::write(root.path().join(&path), b"changed").unwrap();
    assert_eq!(db.journal_verify(&a[0].id).await.unwrap(), "changed");
    std::fs::remove_file(root.path().join(&path)).unwrap();
    assert_eq!(db.journal_verify(&a[0].id).await.unwrap(), "missing");
    sqlx::query("UPDATE attachments SET relative_path='../fixture.pdf'")
        .execute(&db.pool)
        .await
        .unwrap();
    assert!(db.journal_verify(&a[0].id).await.is_err());
    assert!(db
        .journal_link(&entry.id, "javascript:alert(1)")
        .await
        .is_err());
    db.journal_link(&entry.id, "https://example.com/paper")
        .await
        .unwrap();
}

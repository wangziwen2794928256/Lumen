use super::*;
async fn fixture() -> (tempfile::TempDir, Database) {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).await.unwrap();
    (dir, db)
}
fn filter(from: &str, until: &str) -> Filter {
    Filter {
        from: from.into(),
        until: until.into(),
        today: until.into(),
    }
}
fn request() -> String {
    uuid::Uuid::new_v4().to_string()
}
async fn journal(db: &Database, day: &str, body: &str, draft: i64) -> String {
    let id = request();
    sqlx::query("INSERT INTO journal_entries(id,title,body,entry_date,is_draft,created_at,updated_at) VALUES(?,'用户记录',?,?,?, ?,?)").bind(&id).bind(body).bind(day).bind(draft).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    id
}
#[tokio::test]
async fn insights_zero_dates_settings_special_days_and_receipt_conflicts() {
    let (_dir, db) = fixture().await;
    let s = db
        .insights_snapshot(filter("2026-10-01", "2026-10-04"))
        .await
        .unwrap();
    assert_eq!(s.days.len(), 4);
    assert!(s.words.is_empty());
    assert_eq!(s.task_summary["total"], 0);
    assert_eq!(s.days[0].intensity, 0);
    assert!(db
        .insights_snapshot(filter("2026-02-30", "2026-10-04"))
        .await
        .is_err());
    assert!(db
        .insights_snapshot(filter("2026-10-04", "2026-10-01"))
        .await
        .is_err());
    let prefs = Preferences {
        thresholds: [5., 4., 3., 2.],
        ..Preferences::default()
    };
    assert!(db
        .insights_action(Action::Preferences {
            request_id: request(),
            revision: 0,
            value: prefs
        })
        .await
        .is_err());
    let action = Action::Day {
        request_id: request(),
        date: "2026-10-04".into(),
        revision: 0,
        status: "rest".into(),
    };
    db.insights_action(action.clone()).await.unwrap();
    db.insights_action(action.clone()).await.unwrap();
    let changed = match action {
        Action::Day {
            request_id,
            date,
            revision,
            ..
        } => Action::Day {
            request_id,
            date,
            revision,
            status: "sick".into(),
        },
        _ => unreachable!(),
    };
    assert_eq!(
        db.insights_action(changed).await.unwrap_err().code,
        "INSIGHTS_CONFLICT"
    );
    let s = db
        .insights_snapshot(filter("2026-10-01", "2026-10-04"))
        .await
        .unwrap();
    assert_eq!(s.days[3].status, "rest");
    assert_eq!(s.days[3].capacity_minutes, 210);
    assert_eq!(s.consistency["days7"]["eligible"], 6);
    assert_eq!(s.consistency["days7"]["protected"], 1);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM capacity_days")
            .fetch_one(&db.pool)
            .await
            .unwrap(),
        0
    );
}
#[tokio::test]
async fn insights_words_sources_revision_deletion_and_music_separation() {
    let (_dir, db) = fixture().await;
    let id = journal(&db, "2026-10-04", "我们研究数学，数学学习 Rust！", 0).await;
    journal(&db, "2026-10-04", "草稿隐藏词语", 1).await;
    let s = db
        .insights_snapshot(filter("2026-10-01", "2026-10-04"))
        .await
        .unwrap();
    let word = s.words.iter().find(|w| w.word == "数学").unwrap();
    assert_eq!(word.count, 2);
    assert_eq!(word.records[0].id, id);
    assert!(!s.words.iter().any(|w| w.word == "我们" || w.word == "隐藏"));
    assert!(s.music.is_empty());
    sqlx::query("UPDATE journal_entries SET body='物理物理',revision=revision+1 WHERE id=?")
        .bind(&id)
        .execute(&db.pool)
        .await
        .unwrap();
    let s = db
        .insights_snapshot(filter("2026-10-01", "2026-10-04"))
        .await
        .unwrap();
    assert!(!s.words.iter().any(|w| w.word == "数学"));
    assert!(s.words.iter().any(|w| w.word == "物理"));
    sqlx::query("UPDATE journal_entries SET deleted_at=? WHERE id=?")
        .bind(now())
        .bind(id)
        .execute(&db.pool)
        .await
        .unwrap();
    assert!(db
        .insights_snapshot(filter("2026-10-01", "2026-10-04"))
        .await
        .unwrap()
        .words
        .is_empty());
}
#[tokio::test]
async fn insights_actual_cross_day_coding_manual_and_plan_never_double_count() {
    let (_dir, db) = fixture().await;
    let task = request();
    let focus = request();
    let problem = request();
    sqlx::query("INSERT INTO tasks(id,title,due_date,lifecycle,estimated_minutes,created_at,updated_at) VALUES(?,'fixture','2026-10-04','done',100,?,?)").bind(&task).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    sqlx::query("INSERT INTO focus_sessions(id,task_id,started_at,ended_at,planned_seconds,actual_seconds,elapsed_ms,state,created_at,updated_at) VALUES(?,?,'2026-10-03T15:59:00Z','2026-10-03T16:01:00Z',900,120,120000,'finished',?,?)").bind(&focus).bind(&task).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    for (segment, day) in [(1, "2026-10-03"), (2, "2026-10-04")] {
        sqlx::query("INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms) VALUES(?,?,?,'2026-10-03T15:59:00Z','2026-10-03T16:01:00Z',60000)").bind(&focus).bind(segment).bind(day).execute(&db.pool).await.unwrap();
    }
    sqlx::query(
        "INSERT INTO coding_problems(id,title,created_at,updated_at) VALUES(?,'fixture',?,?)",
    )
    .bind(&problem)
    .bind(now())
    .bind(now())
    .execute(&db.pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO coding_attempts(id,problem_id,outcome,attempted_on,manual_seconds,focus_session_id,created_at,updated_at) VALUES(?,?,'accepted','2026-10-04',180,?,?,?)").bind(request()).bind(problem).bind(focus).bind(now()).bind(now()).execute(&db.pool).await.unwrap();
    let s = db
        .insights_snapshot(filter("2026-10-03", "2026-10-04"))
        .await
        .unwrap();
    assert_eq!(s.days[0].deep_minutes, 1.);
    assert_eq!(s.days[1].deep_minutes, 1.);
    assert_eq!(s.days[0].coding_minutes, 1.);
    assert_eq!(s.days[1].coding_minutes, 4.);
    assert_eq!(s.days[1].planned_minutes, 100.);
    assert_eq!(s.days[1].tasks_completed, 1);
    assert_eq!(s.days[1].problem_count, 1);
}
#[tokio::test]
async fn insights_review_persistence_no_generated_words_and_month_frozen() {
    let (dir, db) = fixture().await;
    let r = db.insights_review("2026-10-04".into()).await.unwrap();
    assert_eq!(r["body"], "");
    assert_eq!(r["week"], "2026-09-28");
    let action = Action::Review {
        request_id: request(),
        week: "2026-10-04".into(),
        revision: 0,
        body: "研究数学".into(),
        next_focus: "专注物理".into(),
    };
    db.insights_action(action.clone()).await.unwrap();
    db.insights_action(action).await.unwrap();
    assert_eq!(
        db.insights_action(Action::Review {
            request_id: request(),
            week: "2026-10-04".into(),
            revision: 0,
            body: "过期".into(),
            next_focus: "".into()
        })
        .await
        .unwrap_err()
        .code,
        "INSIGHTS_CONFLICT"
    );
    sqlx::raw_sql("WITH RECURSIVE n(i) AS(SELECT 1 UNION ALL SELECT i+1 FROM n WHERE i<601) INSERT INTO artifacts(id,title,kind,created_at,updated_at) SELECT printf('60000000-0000-4000-8000-%012d',i),'隔离项目','project','2026-09-29T12:00:00Z','2026-09-29T12:00:00Z' FROM n").execute(&db.pool).await.unwrap();
    assert_eq!(db.insights_close_months("2026-10-04").await.unwrap(), 1);
    assert_eq!(db.insights_close_months("2026-10-04").await.unwrap(), 0);
    let month = db
        .insights_month("2026-09".into(), "2026-10-04".into())
        .await
        .unwrap();
    assert_eq!(month["closed"], true);
    assert_eq!(month["counts"]["projects"], 601);
    assert!(month.to_string().contains("数学"));
    assert!(!month.to_string().contains("My Reflection"));
    journal(&db, "2026-09-29", "后来添加的内容", 0).await;
    assert_eq!(
        db.insights_month("2026-09".into(), "2026-10-04".into())
            .await
            .unwrap(),
        month
    );
    db.pool.close().await;
    let reopened = Database::open(dir.path()).await.unwrap();
    assert_eq!(
        reopened.insights_review("2026-10-04".into()).await.unwrap()["body"],
        "研究数学"
    );
}
#[tokio::test]
async fn insights_anniversary_leap_day_and_disable() {
    let (_dir, db) = fixture().await;
    let id = journal(&db, "2024-02-29", "往年记录", 0).await;
    assert_eq!(
        db.insights_memories("2028-02-29".into()).await.unwrap()[0].id,
        id
    );
    assert!(db
        .insights_memories("2027-02-28".into())
        .await
        .unwrap()
        .is_empty());
    let p = Preferences {
        memories: false,
        ..Preferences::default()
    };
    db.insights_action(Action::Preferences {
        request_id: request(),
        revision: 0,
        value: p,
    })
    .await
    .unwrap();
    assert!(db
        .insights_memories("2028-02-29".into())
        .await
        .unwrap()
        .is_empty());
}

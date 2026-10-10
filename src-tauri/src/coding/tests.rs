use super::*;
fn key() -> String {
    uuid::Uuid::new_v4().to_string()
}
fn filter() -> Filter {
    Filter {
        week: "2026-10-03".into(),
        query: String::new(),
        platform: String::new(),
        tag: String::new(),
        status: String::new(),
        trash: false,
        page: 0,
        selected: None,
    }
}
fn problem(title: &str, external: &str) -> ProblemInput {
    ProblemInput {
        title: title.into(),
        platform: "LeetCode".into(),
        external_id: external.into(),
        difficulty: "medium".into(),
        source_url: "https://leetcode.com/problems/two-sum/".into(),
        tags: vec!["Graph".into(), "DP".into(), "DP".into()],
        notes: "fixture".into(),
        journey_id: None,
        task_id: None,
        competition_id: None,
    }
}
fn attempt(problem: &str, date: &str, outcome: &str) -> AttemptInput {
    AttemptInput {
        problem_id: problem.into(),
        language: "Rust".into(),
        outcome: outcome.into(),
        notes: String::new(),
        attempted_on: date.into(),
        started_at: format!("{date}T10:00:00Z"),
        manual_seconds: 60,
        focus_session_id: None,
    }
}
async fn save(db: &Database, id: &str, p: ProblemInput) -> Action {
    let action = Action::Problem {
        request_id: key(),
        id: id.into(),
        revision: None,
        input: p,
    };
    db.coding_action(action.clone()).await.unwrap();
    action
}
async fn add(db: &Database, p: &str, date: &str, outcome: &str) -> String {
    let id = key();
    db.coding_action(Action::Attempt {
        request_id: key(),
        id: id.clone(),
        revision: None,
        input: attempt(p, date, outcome),
    })
    .await
    .unwrap();
    id
}
#[tokio::test]
async fn coding_identity_revision_retry_soft_delete_restore_and_restart() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let p = key();
    let action = save(&db, &p, problem("题目", "1")).await;
    db.coding_action(action.clone()).await.unwrap();
    let s = db.coding_snapshot(filter()).await.unwrap();
    assert_eq!(s.total, 1);
    assert_eq!(s.problems[0].revision, 1);
    assert_eq!(s.problems[0].input.tags.len(), 2);
    let mut changed = action.clone();
    if let Action::Problem { input, .. } = &mut changed {
        input.title = "changed".into();
    }
    assert_eq!(
        db.coding_action(changed).await.unwrap_err().code,
        "CODING_CONFLICT"
    );
    let dup = Action::Problem {
        request_id: key(),
        id: key(),
        revision: None,
        input: problem("重复", "1"),
    };
    assert_eq!(
        db.coding_action(dup).await.unwrap_err().code,
        "CODING_DUPLICATE"
    );
    let bad = Action::Problem {
        request_id: key(),
        id: p.clone(),
        revision: Some(0),
        input: problem("陈旧", "1"),
    };
    assert_eq!(
        db.coding_action(bad).await.unwrap_err().code,
        "CODING_CONFLICT"
    );
    let mut invalid = problem("非法链接", "2");
    invalid.source_url = "javascript:alert(1)".into();
    assert_eq!(
        db.coding_action(Action::Problem {
            request_id: key(),
            id: key(),
            revision: None,
            input: invalid
        })
        .await
        .unwrap_err()
        .code,
        "INVALID_INPUT"
    );
    add(&db, &p, "2026-10-03", "accepted").await;
    db.coding_action(Action::Trash {
        request_id: key(),
        id: p.clone(),
        kind: "problem".into(),
        revision: 1,
        restore: false,
    })
    .await
    .unwrap();
    assert_eq!(
        db.coding_snapshot(filter()).await.unwrap().analytics.solved,
        0
    );
    let replacement = key();
    save(&db, &replacement, problem("替代", "1")).await;
    assert_eq!(
        db.coding_action(Action::Trash {
            request_id: key(),
            id: p.clone(),
            kind: "problem".into(),
            revision: 2,
            restore: true
        })
        .await
        .unwrap_err()
        .code,
        "CODING_DUPLICATE"
    );
    db.coding_action(Action::Trash {
        request_id: key(),
        id: replacement,
        kind: "problem".into(),
        revision: 1,
        restore: false,
    })
    .await
    .unwrap();
    db.coding_action(Action::Trash {
        request_id: key(),
        id: p.clone(),
        kind: "problem".into(),
        revision: 2,
        restore: true,
    })
    .await
    .unwrap();
    db.pool.close().await;
    let reopened = Database::open(root.path()).await.unwrap();
    let s = reopened.coding_snapshot(filter()).await.unwrap();
    assert_eq!(s.problems[0].attempt_count, 1);
    assert!(s.problems[0].solved);
    assert_eq!(s.problems[0].revision, 3);
}
#[tokio::test]
async fn coding_analytics_cohorts_multiple_attempts_unsolved_and_zero() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let zero = db.coding_snapshot(filter()).await.unwrap().analytics;
    assert_eq!(
        (
            zero.problems,
            zero.first_pass_denominator,
            zero.manual_seconds
        ),
        (0, 0, 0)
    );
    let p1 = key();
    let p2 = key();
    let p3 = key();
    let p4 = key();
    save(&db, &p1, problem("首过", "a")).await;
    save(&db, &p2, problem("重试", "b")).await;
    save(&db, &p3, problem("未解", "c")).await;
    save(&db, &p4, problem("以前开始", "d")).await;
    add(&db, &p1, "2026-09-28", "accepted").await;
    add(&db, &p2, "2026-09-29", "wrong_answer").await;
    add(&db, &p2, "2026-09-30", "accepted").await;
    add(&db, &p3, "2026-10-03", "unfinished").await;
    add(&db, &p4, "2026-09-27", "wrong_answer").await;
    add(&db, &p4, "2026-10-01", "accepted").await;
    let s = db.coding_snapshot(filter()).await.unwrap();
    let a = s.analytics;
    assert_eq!((a.problems, a.solved, a.attempts), (4, 3, 5));
    assert_eq!((a.first_pass_numerator, a.first_pass_denominator), (1, 3));
    assert_eq!(a.manual_seconds, 300);
    assert_eq!(a.topics[0].count, 4);
    assert_eq!(a.difficulty[0].count, 4);
    let mut f = filter();
    f.status = "unsolved".into();
    assert_eq!(db.coding_snapshot(f).await.unwrap().total, 1);
    let c = key();
    db.coding_action(Action::Contest {
        request_id: key(),
        id: c.clone(),
        revision: None,
        input: ContestInput {
            title: "fixture contest".into(),
            platform: "AtCoder".into(),
            event_date: "2026-10-03".into(),
            source_url: String::new(),
            participated: true,
            notes: String::new(),
        },
    })
    .await
    .unwrap();
    assert_eq!(
        db.coding_snapshot(filter())
            .await
            .unwrap()
            .analytics
            .contests,
        1
    );
    db.coding_action(Action::Trash {
        request_id: key(),
        id: c,
        kind: "contest".into(),
        revision: 1,
        restore: false,
    })
    .await
    .unwrap();
    assert_eq!(
        db.coding_snapshot(filter())
            .await
            .unwrap()
            .analytics
            .contests,
        0
    );
}
#[tokio::test]
async fn coding_focus_uniqueness_restore_task_guard_pagination_and_filters() {
    let root = tempfile::tempdir().unwrap();
    let db = Database::open(root.path()).await.unwrap();
    let p = key();
    save(&db, &p, problem("关联题", "focus")).await;
    let task = key();
    let focus = key();
    let stamp = now();
    sqlx::query("INSERT INTO tasks(id,title,created_at,updated_at) VALUES(?,'fixture',?,?)")
        .bind(&task)
        .bind(&stamp)
        .bind(&stamp)
        .execute(&db.pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO focus_sessions(id,task_id,started_at,ended_at,planned_seconds,actual_seconds,elapsed_ms,state,last_activity_at,session_date,created_at,updated_at) VALUES(?,?,'2026-10-03T01:00:00Z','2026-10-03T01:01:00Z',60,35,35000,'finished','2026-10-03T01:01:00Z','2026-10-03',?,?)").bind(&focus).bind(&task).bind(&stamp).bind(&stamp).execute(&db.pool).await.unwrap();
    let mut a = attempt(&p, "2026-10-03", "accepted");
    a.focus_session_id = Some(focus.clone());
    let first = key();
    db.coding_action(Action::Attempt {
        request_id: key(),
        id: first.clone(),
        revision: None,
        input: a.clone(),
    })
    .await
    .unwrap();
    assert_eq!(
        db.coding_snapshot(filter())
            .await
            .unwrap()
            .analytics
            .focus_seconds,
        35
    );
    assert_eq!(
        db.coding_action(Action::Attempt {
            request_id: key(),
            id: key(),
            revision: None,
            input: a.clone()
        })
        .await
        .unwrap_err()
        .code,
        "CODING_FOCUS_USED"
    );
    db.coding_action(Action::Trash {
        request_id: key(),
        id: first.clone(),
        kind: "attempt".into(),
        revision: 1,
        restore: false,
    })
    .await
    .unwrap();
    let second = key();
    db.coding_action(Action::Attempt {
        request_id: key(),
        id: second.clone(),
        revision: None,
        input: a.clone(),
    })
    .await
    .unwrap();
    assert_eq!(
        db.coding_action(Action::Trash {
            request_id: key(),
            id: first.clone(),
            kind: "attempt".into(),
            revision: 2,
            restore: true
        })
        .await
        .unwrap_err()
        .code,
        "CODING_FOCUS_USED"
    );
    db.coding_action(Action::Trash {
        request_id: key(),
        id: second,
        kind: "attempt".into(),
        revision: 1,
        restore: false,
    })
    .await
    .unwrap();
    db.coding_action(Action::Trash {
        request_id: key(),
        id: first,
        kind: "attempt".into(),
        revision: 2,
        restore: true,
    })
    .await
    .unwrap();
    for n in 0..34 {
        save(&db, &key(), problem("分页", &format!("page{n}"))).await;
    }
    let mut f = filter();
    assert_eq!(
        db.coding_snapshot(f.clone()).await.unwrap().problems.len(),
        30
    );
    f.page = 1;
    assert_eq!(
        db.coding_snapshot(f.clone()).await.unwrap().problems.len(),
        5
    );
    f.page = 0;
    f.query = "%' OR --".into();
    assert_eq!(db.coding_snapshot(f).await.unwrap().total, 0);
    let mut bad = attempt(&p, "2026-02-30", "accepted");
    assert_eq!(
        db.coding_action(Action::Attempt {
            request_id: key(),
            id: key(),
            revision: None,
            input: bad.clone()
        })
        .await
        .unwrap_err()
        .code,
        "INVALID_INPUT"
    );
    bad.attempted_on = "2026-10-03".into();
    bad.started_at = "invalid".into();
    assert!(db
        .coding_action(Action::Attempt {
            request_id: key(),
            id: key(),
            revision: None,
            input: bad
        })
        .await
        .is_err());
}

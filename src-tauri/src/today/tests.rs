use super::*;
pub(super) async fn fixture() -> (tempfile::TempDir, TodayService) {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let service = TodayService::open(db).await.unwrap();
    (folder, service)
}
pub(super) fn input() -> TaskInput {
    TaskInput {
        title: "Phase 4 isolated fixture".into(),
        body: "'); DROP TABLE tasks; --".into(),
        due_date: Some(Local::now().format("%Y-%m-%d").to_string()),
        estimated_minutes: 25,
        priority: 2,
    }
}
#[tokio::test]
async fn create_edit_concurrency_soft_delete_restore_and_capture_retries() {
    let (_folder, service) = fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    for _ in 0..2 {
        service
            .act(Action::Create {
                id: id.clone(),
                input: input(),
            })
            .await
            .unwrap();
    }
    assert_eq!(service.snapshot().await.unwrap().tasks.len(), 1);
    let mut changed = input();
    changed.title = "edited fixture".into();
    service
        .act(Action::Edit {
            id: id.clone(),
            revision: 0,
            input: changed.clone(),
        })
        .await
        .unwrap();
    assert_eq!(
        service
            .act(Action::Edit {
                id: id.clone(),
                revision: 0,
                input: changed
            })
            .await
            .err()
            .unwrap()
            .code,
        "STATE_CONFLICT"
    );
    service
        .act(Action::Trash {
            id: id.clone(),
            revision: 1,
            deleted: true,
        })
        .await
        .unwrap();
    assert!(service.snapshot().await.unwrap().tasks.is_empty());
    service
        .act(Action::Trash {
            id: id.clone(),
            revision: 2,
            deleted: false,
        })
        .await
        .unwrap();
    assert_eq!(
        service.snapshot().await.unwrap().tasks[0].title,
        "edited fixture"
    );
    let note = uuid::Uuid::new_v4().to_string();
    for _ in 0..2 {
        service
            .act(Action::Capture {
                id: note.clone(),
                body: "真实 fixture 随手记".into(),
            })
            .await
            .unwrap();
    }
    assert_eq!(service.snapshot().await.unwrap().notes.len(), 1);
    service.db.pool.close().await;
}
#[tokio::test]
async fn focus_state_machine_metrics_are_atomic_and_finish_is_idempotent() {
    let (_folder, service) = fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    let focus = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: id.clone(),
            input: input(),
        })
        .await
        .unwrap();
    for _ in 0..2 {
        service
            .act(Action::Start {
                id: focus.clone(),
                task_id: id.clone(),
            })
            .await
            .unwrap();
    }
    assert_eq!(
        service
            .act(Action::Trash {
                id: id.clone(),
                revision: 1,
                deleted: true
            })
            .await
            .err()
            .unwrap()
            .code,
        "STATE_CONFLICT"
    );
    // Simulate monotonic elapsed in the private fixture clock instead of waiting real minutes.
    {
        let mut clock = service.clock.lock().await;
        clock.instant = Instant::now() - std::time::Duration::from_secs(7);
        clock.wall = Utc::now() - chrono::Duration::seconds(7);
    }
    service
        .act(Action::Session {
            id: focus.clone(),
            operation: "pause".into(),
        })
        .await
        .unwrap();
    let paused = service
        .snapshot()
        .await
        .unwrap()
        .session
        .unwrap()
        .elapsed_ms;
    {
        let mut clock = service.clock.lock().await;
        clock.instant = Instant::now() - std::time::Duration::from_secs(3);
        clock.wall = Utc::now() - chrono::Duration::seconds(3);
    }
    service
        .act(Action::Session {
            id: focus.clone(),
            operation: "resume".into(),
        })
        .await
        .unwrap();
    let resumed = service.snapshot().await.unwrap().session.unwrap();
    assert!(resumed.elapsed_ms - paused < 1000);
    assert!(resumed.paused_ms >= 3000);
    for _ in 0..2 {
        service
            .act(Action::Session {
                id: focus.clone(),
                operation: "finish".into(),
            })
            .await
            .unwrap();
    }
    let snapshot = service.snapshot().await.unwrap();
    assert!(snapshot.session.is_none());
    assert_eq!(snapshot.tasks[0].lifecycle, "done");
    assert_eq!(snapshot.timeline.len(), 1);
    assert!(snapshot.actual_seconds >= 7);
    let saved: i64 = sqlx::query_scalar("SELECT actual_seconds FROM focus_sessions WHERE id=?")
        .bind(focus)
        .fetch_one(&service.db.pool)
        .await
        .unwrap();
    assert_eq!(snapshot.actual_seconds, saved);
    service.db.pool.close().await;
}
#[tokio::test]
async fn restart_and_sleep_preserve_checkpoint_without_counting_offline_time() {
    let (folder, service) = fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    let focus = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: id.clone(),
            input: input(),
        })
        .await
        .unwrap();
    service
        .act(Action::Start {
            id: focus.clone(),
            task_id: id,
        })
        .await
        .unwrap();
    sqlx::query(
        "UPDATE focus_sessions SET elapsed_ms=9000,last_activity_at=?,updated_at=? WHERE id=?",
    )
    .bind(
        (Utc::now() - chrono::Duration::hours(2))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
    )
    .bind(
        (Utc::now() - chrono::Duration::hours(2))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
    )
    .bind(&focus)
    .execute(&service.db.pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms,provenance) SELECT id,0,session_date,started_at,last_activity_at,9000,'legacy_total' FROM focus_sessions WHERE id=?")
        .bind(&focus).execute(&service.db.pool).await.unwrap();
    service.db.pool.close().await;
    drop(service);
    let db = Database::open(folder.path()).await.unwrap();
    let reopened = TodayService::open(db).await.unwrap();
    let recovered = reopened.snapshot().await.unwrap().session.unwrap();
    assert_eq!(recovered.state, "interrupted");
    assert_eq!(recovered.elapsed_ms, 9000);
    assert!(recovered.interrupted_ms >= 7_000_000);
    reopened
        .act(Action::Session {
            id: focus.clone(),
            operation: "resume".into(),
        })
        .await
        .unwrap();
    {
        let mut clock = reopened.clock.lock().await;
        clock.instant = Instant::now() - std::time::Duration::from_secs(30);
        clock.wall = Utc::now() - chrono::Duration::seconds(30);
    }
    let asleep = reopened.snapshot().await.unwrap().session.unwrap();
    assert_eq!(asleep.state, "interrupted");
    assert!(asleep.elapsed_ms < 10_000);
    reopened
        .act(Action::Session {
            id: focus,
            operation: "finish".into(),
        })
        .await
        .unwrap();
    assert_eq!(reopened.snapshot().await.unwrap().actual_seconds, 9);
    reopened.db.pool.close().await;
}
#[tokio::test]
async fn cancel_and_manual_completion_never_invent_focus_metrics() {
    let (_folder, service) = fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    let focus = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: id.clone(),
            input: input(),
        })
        .await
        .unwrap();
    service
        .act(Action::Start {
            id: focus.clone(),
            task_id: id.clone(),
        })
        .await
        .unwrap();
    for _ in 0..2 {
        service
            .act(Action::Session {
                id: focus.clone(),
                operation: "cancel".into(),
            })
            .await
            .unwrap();
    }
    let revision = service.snapshot().await.unwrap().tasks[0].revision;
    service
        .act(Action::TaskState {
            id,
            revision,
            state: "done".into(),
        })
        .await
        .unwrap();
    assert_eq!(service.snapshot().await.unwrap().actual_seconds, 0);
    assert!(service.snapshot().await.unwrap().timeline.is_empty());
    service.db.pool.close().await;
}

#[tokio::test]
async fn invalid_input_receipt_conflicts_and_single_writer_are_rejected() {
    let (_folder, service) = fixture().await;
    assert!(
        matches!(TodayService::open(service.db.clone()).await, Err(e) if e.code == "DATABASE_BUSY")
    );
    for date in ["2026-02-30", "2026-1-1"] {
        let mut invalid = input();
        invalid.due_date = Some(date.into());
        assert!(
            matches!(service.act(Action::Create { id: uuid::Uuid::new_v4().to_string(), input: invalid }).await, Err(e) if e.code == "INVALID_INPUT")
        );
    }
    let id = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: id.clone(),
            input: input(),
        })
        .await
        .unwrap();
    let mut changed = input();
    changed.title = "different retry payload".into();
    assert!(
        matches!(service.act(Action::Create { id, input: changed }).await, Err(e) if e.code == "STATE_CONFLICT")
    );
    assert_eq!(service.snapshot().await.unwrap().tasks.len(), 1);
    service.db.pool.close().await;
}

#[tokio::test]
async fn today_excludes_historical_done_and_future_planned_but_keeps_overdue() {
    let (_folder, service) = fixture().await;
    let old = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: old.clone(),
            input: input(),
        })
        .await
        .unwrap();
    service
        .act(Action::TaskState {
            id: old.clone(),
            revision: 0,
            state: "done".into(),
        })
        .await
        .unwrap();
    sqlx::query("UPDATE tasks SET updated_at='2000-01-01T00:00:00Z' WHERE id=?")
        .bind(old)
        .execute(&service.db.pool)
        .await
        .unwrap();
    let mut future = input();
    future.due_date = Some("2099-01-01".into());
    let future_id = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: future_id.clone(),
            input: future,
        })
        .await
        .unwrap();
    let mut overdue = input();
    overdue.due_date = Some("2000-01-01".into());
    let overdue_id = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: overdue_id.clone(),
            input: overdue,
        })
        .await
        .unwrap();
    service.act(Action::Capacity { minutes: 90 }).await.unwrap();
    let snapshot = service.snapshot().await.unwrap();
    assert_eq!(snapshot.tasks.len(), 1);
    assert_eq!(snapshot.planned_minutes, 25);
    assert_eq!(snapshot.capacity_minutes, 90);
    assert_eq!(snapshot.actual_seconds, 0);
    let journey = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO journeys(id,title,created_at,updated_at) VALUES(?,?,?,?)")
        .bind(&journey)
        .bind("isolated Journey")
        .bind(now())
        .bind(now())
        .execute(&service.db.pool)
        .await
        .unwrap();
    sqlx::query("UPDATE tasks SET journey_id=? WHERE id=?")
        .bind(&journey)
        .bind(future_id)
        .execute(&service.db.pool)
        .await
        .unwrap();
    assert!(service.snapshot().await.unwrap().journey.is_none());
    sqlx::query("UPDATE tasks SET journey_id=? WHERE id=?")
        .bind(journey)
        .bind(&overdue_id)
        .execute(&service.db.pool)
        .await
        .unwrap();
    assert_eq!(
        service.snapshot().await.unwrap().journey.as_deref(),
        Some("isolated Journey")
    );
    for (revision, state) in ["skipped", "moved", "cancelled", "planned"]
        .into_iter()
        .enumerate()
    {
        let changed = service
            .act(Action::TaskState {
                id: overdue_id.clone(),
                revision: revision as i64,
                state: state.into(),
            })
            .await
            .unwrap();
        assert_eq!(changed.tasks[0].lifecycle, state);
        assert_eq!(changed.actual_seconds, 0);
    }
    service.db.pool.close().await;
}

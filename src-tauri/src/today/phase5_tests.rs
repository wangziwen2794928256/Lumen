use super::*;
use chrono::{FixedOffset, TimeZone};
use native_events::SystemEvent;

async fn running() -> (tempfile::TempDir, TodayService, String) {
    let (folder, service) = tests::fixture().await;
    let task = uuid::Uuid::new_v4().to_string();
    let session = uuid::Uuid::new_v4().to_string();
    service
        .act(Action::Create {
            id: task.clone(),
            input: tests::input(),
        })
        .await
        .unwrap();
    service
        .act(Action::Start {
            id: session.clone(),
            task_id: task,
        })
        .await
        .unwrap();
    (folder, service, session)
}

#[test]
fn midnight_split_preserves_monotonic_total() {
    let start = Utc.with_ymd_and_hms(2026, 10, 3, 15, 59, 59).unwrap();
    let parts = intervals::split(
        start,
        start + chrono::Duration::seconds(3),
        3001,
        &FixedOffset::east_opt(28800).unwrap(),
    );
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].date, "2026-10-03");
    assert_eq!(parts[1].date, "2026-10-04");
    assert_eq!(parts[0].elapsed_ms, 1000);
    assert_eq!(parts[1].elapsed_ms, 2001);
}

#[tokio::test]
async fn native_latch_blocks_resume_until_all_system_conditions_clear() {
    let (_folder, service, id) = running().await;
    service.signal(SystemEvent::Lock);
    service.signal(SystemEvent::Suspend);
    service.signal(SystemEvent::Unlock);
    let blocked = service.snapshot().await.unwrap();
    assert!(blocked.system_blocked);
    assert_eq!(blocked.session.unwrap().state, "interrupted");
    assert_eq!(
        service
            .act(Action::Session {
                id: id.clone(),
                operation: "resume".into()
            })
            .await
            .err()
            .unwrap()
            .code,
        "STATE_CONFLICT"
    );
    service.signal(SystemEvent::Resume);
    let unblocked = service.snapshot().await.unwrap();
    assert!(!unblocked.system_blocked);
    assert_eq!(unblocked.session.unwrap().state, "interrupted");
    service
        .act(Action::Session {
            id: id.clone(),
            operation: "resume".into(),
        })
        .await
        .unwrap();
    service.signal(SystemEvent::Lock);
    service.signal(SystemEvent::Unlock);
    let fast = service.snapshot().await.unwrap();
    assert_eq!(
        fast.session.unwrap().interruption_reason.as_deref(),
        Some("screen_lock")
    );
    service
        .act(Action::Session {
            id,
            operation: "cancel".into(),
        })
        .await
        .unwrap();
    assert_eq!(service.snapshot().await.unwrap().actual_seconds, 0);
    service.db.pool.close().await;
}

#[tokio::test]
async fn pause_separates_actual_intervals_and_terminal_guards_preserve_total() {
    let (_folder, service, id) = running().await;
    for operation in ["pause", "resume", "finish"] {
        {
            let mut clock = service.clock.lock().await;
            clock.instant = Instant::now() - std::time::Duration::from_secs(2);
            clock.wall = Utc::now() - chrono::Duration::seconds(2);
        }
        service
            .act(Action::Session {
                id: id.clone(),
                operation: operation.into(),
            })
            .await
            .unwrap();
    }
    let rows: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT segment_no,elapsed_ms FROM focus_intervals WHERE session_id=? ORDER BY segment_no",
    )
    .bind(&id)
    .fetch_all(&service.db.pool)
    .await
    .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|r| r.1 >= 2000));
    let elapsed: i64 = sqlx::query_scalar("SELECT elapsed_ms FROM focus_sessions WHERE id=?")
        .bind(&id)
        .fetch_one(&service.db.pool)
        .await
        .unwrap();
    assert_eq!(rows.iter().map(|r| r.1).sum::<i64>(), elapsed);
    for operation in ["pause", "resume", "cancel", "interrupt"] {
        assert_eq!(
            service
                .act(Action::Session {
                    id: id.clone(),
                    operation: operation.into()
                })
                .await
                .err()
                .unwrap()
                .code,
            "STATE_CONFLICT"
        );
    }
    service
        .act(Action::Session {
            id,
            operation: "finish".into(),
        })
        .await
        .unwrap();
    assert_eq!(
        service.snapshot().await.unwrap().actual_seconds,
        elapsed / 1000
    );
    service.db.pool.close().await;
}

#[tokio::test]
async fn calendar_crud_checks_revision_retry_overlap_and_soft_restore() {
    let (_folder, service) = tests::fixture().await;
    let id = uuid::Uuid::new_v4().to_string();
    let input = calendar::EventInput {
        title: "跨日考试 fixture".into(),
        event_type: "exam".into(),
        event_date: None,
        starts_at: Some("2026-10-03T23:30:00+08:00".into()),
        ends_at: Some("2026-10-04T00:30:00+08:00".into()),
        task_id: None,
    };
    for _ in 0..2 {
        service
            .act(Action::CalendarCreate {
                id: id.clone(),
                input: input.clone(),
            })
            .await
            .unwrap();
    }
    let snapshot = service
        .calendar_snapshot("2026-10-03", "2026-10-05")
        .await
        .unwrap();
    assert_eq!(snapshot.events.len(), 1);
    assert!(snapshot.events[0]
        .starts_at
        .as_ref()
        .unwrap()
        .ends_with('Z'));
    assert!(snapshot.actual.is_empty());
    service
        .act(Action::CalendarEdit {
            id: id.clone(),
            revision: 0,
            input: input.clone(),
        })
        .await
        .unwrap();
    assert_eq!(
        service
            .act(Action::CalendarEdit {
                id: id.clone(),
                revision: 0,
                input: input.clone()
            })
            .await
            .err()
            .unwrap()
            .code,
        "STATE_CONFLICT"
    );
    service
        .act(Action::CalendarTrash {
            id: id.clone(),
            revision: 1,
            deleted: true,
        })
        .await
        .unwrap();
    let trashed = service
        .calendar_snapshot("2026-10-03", "2026-10-05")
        .await
        .unwrap();
    assert!(trashed.events.is_empty());
    assert_eq!(trashed.trash.len(), 1);
    service
        .act(Action::CalendarTrash {
            id,
            revision: 2,
            deleted: false,
        })
        .await
        .unwrap();
    assert_eq!(
        service
            .calendar_snapshot("2026-10-03", "2026-10-05")
            .await
            .unwrap()
            .events
            .len(),
        1
    );
    let mut invalid = input;
    invalid.ends_at = invalid.starts_at.clone();
    assert!(calendar::validate_event(&invalid).is_err());
    invalid.starts_at = None;
    invalid.ends_at = None;
    invalid.event_date = Some("2026-02-30".into());
    assert!(calendar::validate_event(&invalid).is_err());
    assert!(service
        .calendar_snapshot("2026-10-05", "2026-10-03")
        .await
        .is_err());
    service.db.pool.close().await;
}

#[tokio::test]
async fn finish_rebuilds_each_local_day_without_double_counting() {
    let (_folder, service, id) = running().await;
    sqlx::query("UPDATE focus_sessions SET state='paused',elapsed_ms=18000 WHERE id=?")
        .bind(&id)
        .execute(&service.db.pool)
        .await
        .unwrap();
    for (segment, day, start, end) in [
        (
            1,
            "2026-10-03",
            "2026-10-03T15:59:51.000Z",
            "2026-10-03T16:00:00.000Z",
        ),
        (
            2,
            "2026-10-04",
            "2026-10-03T16:00:00.000Z",
            "2026-10-03T16:00:09.000Z",
        ),
    ] {
        sqlx::query("INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms) VALUES(?,?,?,?,?,9000)").bind(&id).bind(segment).bind(day).bind(start).bind(end).execute(&service.db.pool).await.unwrap();
    }
    for _ in 0..2 {
        service
            .act(Action::Session {
                id: id.clone(),
                operation: "finish".into(),
            })
            .await
            .unwrap();
    }
    let totals:Vec<(String,f64)>=sqlx::query_as("SELECT metric_date,value FROM daily_metrics WHERE metric_key='focus_seconds' AND metric_date IN ('2026-10-03','2026-10-04') ORDER BY metric_date").fetch_all(&service.db.pool).await.unwrap();
    assert_eq!(
        totals,
        vec![("2026-10-03".into(), 9.0), ("2026-10-04".into(), 9.0)]
    );
    assert_eq!(
        service
            .calendar_snapshot("2026-10-03", "2026-10-05")
            .await
            .unwrap()
            .actual
            .len(),
        2
    );
    service.db.pool.close().await;
}

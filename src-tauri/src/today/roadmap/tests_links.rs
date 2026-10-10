use super::{model::*, *};
use chrono::{Datelike, Duration};
#[tokio::test]
async fn accepted_blocks_link_today_capacity_and_task_progress_without_changing_deadline() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let service = TodayService::open(db.clone()).await.unwrap();
    let today = Local::now().date_naive();
    let week = (today - Duration::days(today.weekday().num_days_from_monday().into())).to_string();
    let deadline = (today + Duration::days(6)).to_string();
    let journey = uuid::Uuid::new_v4().to_string();
    service
        .roadmap_action(
            &week,
            Action::Save {
                id: journey.clone(),
                kind: "journey".into(),
                revision: None,
                input: EntityInput {
                    title: "隔离关联".into(),
                    journey_id: None,
                    data: Details {
                        progress_mode: "tasks".into(),
                        ..Details::default()
                    },
                },
            },
        )
        .await
        .unwrap();
    let task = uuid::Uuid::new_v4().to_string();
    service
        .act(super::super::Action::Create {
            id: task.clone(),
            input: super::super::TaskInput {
                title: "未来到期任务".into(),
                body: "".into(),
                due_date: Some(deadline.clone()),
                estimated_minutes: 25,
                priority: 1,
            },
        })
        .await
        .unwrap();
    service
        .roadmap_action(
            &week,
            Action::TaskPlan {
                id: task.clone(),
                revision: 0,
                journey_id: Some(journey.clone()),
                subject_id: None,
                quest_id: None,
                input: TaskOptions::default(),
            },
        )
        .await
        .unwrap();
    let decision = uuid::Uuid::new_v4().to_string();
    service
        .roadmap_action(
            &week,
            Action::Propose {
                id: decision.clone(),
            },
        )
        .await
        .unwrap();
    service
        .roadmap_action(
            &week,
            Action::Decide {
                id: decision,
                operation: "accept".into(),
                allocations: None,
            },
        )
        .await
        .unwrap();
    let current = service.snapshot().await.unwrap();
    assert!(current.tasks.iter().any(|t| t.id == task));
    assert_eq!(current.planned_minutes, 25);
    assert_eq!(current.journey.as_deref(), Some("隔离关联"));
    assert_eq!(
        current.tasks[0].due_date.as_deref(),
        Some(deadline.as_str())
    );
    service
        .roadmap_action(
            &week,
            Action::Capacity {
                input: CapacityDay {
                    date: today.to_string(),
                    minutes: 0,
                    status: "sick".into(),
                },
            },
        )
        .await
        .unwrap();
    assert_eq!(service.snapshot().await.unwrap().capacity_minutes, 0);
    let snapshot = service.roadmap_snapshot(&week).await.unwrap();
    let budget = snapshot
        .budgets
        .iter()
        .find(|d| d.date == today.to_string())
        .unwrap();
    assert_eq!(budget.available, 0);
    assert!(!snapshot.blocks.is_empty());
    service
        .act(super::super::Action::TaskState {
            id: task,
            revision: 1,
            state: "done".into(),
        })
        .await
        .unwrap();
    let snapshot = service.roadmap_snapshot(&week).await.unwrap();
    assert_eq!(
        snapshot
            .entities
            .iter()
            .find(|e| e.id == journey)
            .unwrap()
            .data
            .progress,
        100
    );
    db.pool.close().await;
}
#[tokio::test]
async fn parent_cycle_stale_revision_and_archived_dependencies_cannot_enter_planner() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let service = TodayService::open(db.clone()).await.unwrap();
    let week = "2090-10-02";
    let parent = uuid::Uuid::new_v4().to_string();
    let child = uuid::Uuid::new_v4().to_string();
    for (id, parent_id) in [
        (parent.clone(), None),
        (child.clone(), Some(parent.clone())),
    ] {
        service
            .roadmap_action(
                week,
                Action::Save {
                    id,
                    kind: "journey".into(),
                    revision: None,
                    input: EntityInput {
                        title: "关联层级".into(),
                        journey_id: None,
                        data: Details {
                            parent_id,
                            ..Details::default()
                        },
                    },
                },
            )
            .await
            .unwrap();
    }
    assert!(service
        .roadmap_action(
            week,
            Action::Save {
                id: parent.clone(),
                kind: "journey".into(),
                revision: Some(0),
                input: EntityInput {
                    title: "循环应拒绝".into(),
                    journey_id: None,
                    data: Details {
                        parent_id: Some(child.clone()),
                        ..Details::default()
                    }
                }
            }
        )
        .await
        .is_err());
    service
        .roadmap_action(
            week,
            Action::Lifecycle {
                id: parent.clone(),
                kind: "journey".into(),
                revision: 0,
                operation: "archive".into(),
            },
        )
        .await
        .unwrap();
    let snapshot = service.roadmap_snapshot(week).await.unwrap();
    assert!(store::live_entities(&snapshot).is_empty());
    assert!(service
        .roadmap_action(
            week,
            Action::Lifecycle {
                id: parent,
                kind: "journey".into(),
                revision: 0,
                operation: "unarchive".into()
            }
        )
        .await
        .is_err());
    db.pool.close().await;
}

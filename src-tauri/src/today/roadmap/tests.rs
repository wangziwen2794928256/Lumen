use super::{model::*, planner, store, *};
use chrono::TimeZone;
fn task(id: &str, minutes: i64, deadline: Option<&str>) -> PlanningTask {
    PlanningTask {
        id: id.into(),
        title: id.into(),
        remaining_minutes: minutes,
        priority: 1,
        deadline: deadline.map(str::to_string),
        journey_id: None,
        subject_id: None,
        quest_id: None,
        revision: 0,
        options: TaskOptions::default(),
    }
}
fn days() -> Vec<DayBudget> {
    (0..7)
        .map(|n| DayBudget {
            date: format!("2090-10-{:02}", 2 + n),
            capacity: 120,
            fixed_minutes: 0,
            available: 120,
            status: "normal".into(),
        })
        .collect()
}
#[test]
fn ten_planner_scenarios_conserve_capacity_and_leave_infeasible_work_visible() {
    for scenario in [
        "normal week",
        "overloaded week",
        "competition week",
        "paper deadline",
        "sick day",
        "missed day",
        "zero capacity day",
        "multiple deadlines",
        "task unsplittable",
        "task overdue",
    ] {
        let mut budget = days();
        let mut tasks = vec![task("a", 240, Some("2090-10-08"))];
        let side = 60;
        match scenario {
            "overloaded week" => tasks[0].remaining_minutes = 1200,
            "competition week" => tasks[0].quest_id = Some("q".into()),
            "paper deadline" => tasks[0].deadline = Some("2090-10-02".into()),
            "sick day" | "missed day" | "zero capacity day" => {
                budget[0].available = 0;
                budget[0].capacity = 0;
                budget[0].status = "sick".into()
            }
            "multiple deadlines" => tasks.push(task("b", 120, Some("2090-10-02"))),
            "task unsplittable" => {
                tasks[0].remaining_minutes = 180;
                tasks[0].options.splittable = false
            }
            "task overdue" => tasks[0].deadline = Some("2090-10-01".into()),
            _ => {}
        }
        let plan = planner::plan(&tasks, &[], budget.clone(), side);
        planner::validate_allocations(&tasks, &[], &budget, side, &plan.allocations).unwrap();
        assert_eq!(
            plan.days.iter().map(|d| d.capacity).sum::<i64>(),
            budget.iter().map(|d| d.capacity).sum::<i64>(),
            "{scenario}"
        );
        assert!(
            plan.allocations.iter().map(|a| a.minutes).sum::<i64>()
                <= budget.iter().map(|d| d.available).sum::<i64>()
        );
        if [
            "overloaded week",
            "competition week",
            "paper deadline",
            "task unsplittable",
            "task overdue",
        ]
        .contains(&scenario)
        {
            assert!(!plan.unscheduled.is_empty(), "{scenario}")
        }
        if scenario == "normal week" {
            assert!(plan.unscheduled.is_empty())
        }
    }
}
#[test]
fn minimum_reservations_survive_urgent_work_and_manual_overspend_is_rejected() {
    let mut tasks = vec![
        task("urgent", 840, Some("2090-10-08")),
        task("protected", 300, Some("2090-10-08")),
    ];
    tasks[1].subject_id = Some("s".into());
    let entity = Entity {
        id: "s".into(),
        kind: "subject".into(),
        title: "保护学科".into(),
        journey_id: None,
        data: Details {
            minimum_minutes: 300,
            preferred_minutes: 300,
            maximum_minutes: 360,
            ..Details::default()
        },
        revision: 0,
        deleted_at: None,
        archived_at: None,
    };
    let plan = planner::plan(&tasks, std::slice::from_ref(&entity), days(), 100);
    assert_eq!(
        plan.allocations
            .iter()
            .filter(|a| a.task_id == "protected")
            .map(|a| a.minutes)
            .sum::<i64>(),
        300
    );
    planner::validate_allocations(&tasks, &[entity], &plan.days, 100, &plan.allocations).unwrap();
    let invalid = vec![Allocation {
        task_id: "urgent".into(),
        date: "2090-10-02".into(),
        minutes: 121,
    }];
    assert!(planner::validate_allocations(&tasks, &[], &days(), 100, &invalid).is_err());
}
#[tokio::test]
async fn template_is_explicit_idempotent_editable_and_soft_deleted() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let service = TodayService::open(db.clone()).await.unwrap();
    let week = "2090-10-02";
    assert!(service
        .roadmap_snapshot(week)
        .await
        .unwrap()
        .entities
        .is_empty());
    let id = uuid::Uuid::new_v4().to_string();
    service
        .roadmap_action(week, Action::Template { id: id.clone() })
        .await
        .unwrap();
    let snapshot = service
        .roadmap_action(week, Action::Template { id: id.clone() })
        .await
        .unwrap();
    assert_eq!(snapshot.entities.len(), 21);
    let journey = snapshot.entities.iter().find(|e| e.id == id).unwrap();
    assert_eq!(journey.data.planning_target, Some(400));
    let input = EntityInput {
        title: "自己的目标".into(),
        journey_id: None,
        data: journey.data.clone(),
    };
    service
        .roadmap_action(
            week,
            Action::Save {
                id: id.clone(),
                kind: "journey".into(),
                revision: Some(0),
                input,
            },
        )
        .await
        .unwrap();
    let snapshot = service
        .roadmap_action(
            week,
            Action::Lifecycle {
                id: id.clone(),
                kind: "journey".into(),
                revision: 1,
                operation: "trash".into(),
            },
        )
        .await
        .unwrap();
    assert!(snapshot
        .entities
        .iter()
        .find(|e| e.id == id)
        .unwrap()
        .deleted_at
        .is_some());
    service
        .roadmap_action(
            week,
            Action::Lifecycle {
                id,
                kind: "journey".into(),
                revision: 2,
                operation: "restore".into(),
            },
        )
        .await
        .unwrap();
    db.pool.close().await;
}
#[tokio::test]
async fn decisions_are_atomic_reject_modify_undo_and_detect_stale_basis() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let service = TodayService::open(db.clone()).await.unwrap();
    let week = "2090-10-02";
    let tid = uuid::Uuid::new_v4().to_string();
    service
        .act(super::super::Action::Create {
            id: tid.clone(),
            input: super::super::TaskInput {
                title: "隔离规划测试".into(),
                body: "".into(),
                due_date: Some("2090-10-08".into()),
                estimated_minutes: 100,
                priority: 1,
            },
        })
        .await
        .unwrap();
    let propose = |id: String| Action::Propose { id };
    let decide = |id: String, op: &str, allocations| Action::Decide {
        id,
        operation: op.into(),
        allocations,
    };
    let id = uuid::Uuid::new_v4().to_string();
    let snapshot = service
        .roadmap_action(week, propose(id.clone()))
        .await
        .unwrap();
    assert!(snapshot.blocks.is_empty());
    assert_eq!(snapshot.decisions[0].status, "proposed");
    service
        .roadmap_action(week, decide(id.clone(), "accept", None))
        .await
        .unwrap();
    let snapshot = service
        .roadmap_action(week, decide(id.clone(), "accept", None))
        .await
        .unwrap();
    assert_eq!(snapshot.blocks.iter().map(|a| a.minutes).sum::<i64>(), 100);
    let snapshot = service
        .roadmap_action(week, decide(id, "undo", None))
        .await
        .unwrap();
    assert!(snapshot.blocks.is_empty());
    let id = uuid::Uuid::new_v4().to_string();
    service
        .roadmap_action(week, propose(id.clone()))
        .await
        .unwrap();
    let snapshot = service
        .roadmap_action(week, decide(id, "reject", None))
        .await
        .unwrap();
    assert!(snapshot.blocks.is_empty());
    let id = uuid::Uuid::new_v4().to_string();
    service
        .roadmap_action(week, propose(id.clone()))
        .await
        .unwrap();
    let allocation = vec![Allocation {
        task_id: tid,
        date: week.into(),
        minutes: 50,
    }];
    let snapshot = service
        .roadmap_action(week, decide(id.clone(), "modify", Some(allocation)))
        .await
        .unwrap();
    assert_eq!(snapshot.blocks[0].minutes, 50);
    service
        .roadmap_action(week, decide(id, "undo", None))
        .await
        .unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    service
        .roadmap_action(week, propose(id.clone()))
        .await
        .unwrap();
    service
        .roadmap_action(
            week,
            Action::Capacity {
                input: CapacityDay {
                    date: week.into(),
                    minutes: 0,
                    status: "rest".into(),
                },
            },
        )
        .await
        .unwrap();
    assert_eq!(
        service
            .roadmap_action(week, decide(id, "accept", None))
            .await
            .unwrap_err()
            .code,
        "STATE_CONFLICT"
    );
    db.pool.close().await;
}
#[tokio::test]
async fn fixed_events_union_and_rule_hash_versions_are_verified() {
    let folder = tempfile::tempdir().unwrap();
    let db = Database::open(folder.path()).await.unwrap();
    let service = TodayService::open(db.clone()).await.unwrap();
    let week = "2090-10-02";
    for (start, end) in [("09:00:00", "10:00:00"), ("09:30:00", "10:30:00")] {
        let local = |time: &str| {
            Local
                .from_local_datetime(
                    &chrono::NaiveDateTime::parse_from_str(
                        &format!("{week}T{time}"),
                        "%Y-%m-%dT%H:%M:%S",
                    )
                    .unwrap(),
                )
                .earliest()
                .unwrap()
                .with_timezone(&Utc)
                .to_rfc3339()
        };
        service
            .act(super::super::Action::CalendarCreate {
                id: uuid::Uuid::new_v4().to_string(),
                input: super::super::calendar::EventInput {
                    title: "固定事件".into(),
                    event_type: "class".into(),
                    event_date: None,
                    starts_at: Some(local(start)),
                    ends_at: Some(local(end)),
                    task_id: None,
                },
            })
            .await
            .unwrap();
    }
    let snapshot = service.roadmap_snapshot(week).await.unwrap();
    let mut c = db.pool.acquire().await.unwrap();
    let budget = store::budgets(&mut c, &snapshot).await.unwrap();
    assert_eq!(budget[0].fixed_minutes, 90);
    drop(c);
    let id = uuid::Uuid::new_v4().to_string();
    let input = RuleInput {
        journey_id: None,
        school: "复旦大学".into(),
        college: "".into(),
        major: "".into(),
        year: 2027,
        written_exam: "".into(),
        interview: "".into(),
        source_url: "https://gsao.fudan.edu.cn/15015/list.htm".into(),
        body: "隔离用户摘要".into(),
        notes: "不是 2028 政策".into(),
    };
    let snapshot = service
        .roadmap_action(
            week,
            Action::Rule {
                id: id.clone(),
                input: input.clone(),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        snapshot.rules[0].content_hash,
        checksum(input.body.as_bytes())
    );
    assert_eq!(snapshot.rules[0].input.year, 2027);
    let mut bad = input;
    bad.source_url = "https://gsao.fudan.edu.cn.evil.example/".into();
    assert!(service
        .roadmap_action(
            week,
            Action::Rule {
                id: uuid::Uuid::new_v4().to_string(),
                input: bad
            }
        )
        .await
        .is_err());
    service
        .roadmap_action(week, Action::RuleTrash { id, deleted: true })
        .await
        .unwrap();
    db.pool.close().await;
}

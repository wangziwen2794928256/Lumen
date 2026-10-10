use super::model::conflict;
use super::{model::*, planner, store, *};
fn canonical(mut rows: Vec<Allocation>) -> Vec<Allocation> {
    rows.sort_by(|a, b| (&a.date, &a.task_id).cmp(&(&b.date, &b.task_id)));
    rows
}
fn basis(snapshot: &Snapshot, days: &[DayBudget]) -> Result<String> {
    Ok(checksum(
        &serde_json::to_vec(&(
            Local::now().date_naive().to_string(),
            &snapshot.entities,
            &snapshot.tasks,
            &snapshot.preferences,
            &snapshot.days,
            &snapshot.blocks,
            days,
        ))
        .map_err(|_| DbError::invalid())?,
    ))
}
async fn blocks(
    tx: &mut Transaction<'_, Sqlite>,
    week: &str,
    id: &str,
    rows: &[Allocation],
) -> Result<()> {
    let until = (store::week(week)? + chrono::Duration::days(7)).to_string();
    // Only replace derived planning allocations; original tasks and immutable audit remain.
    sqlx::query("DELETE FROM planning_blocks WHERE date>=? AND date<?")
        .bind(week)
        .bind(until)
        .execute(&mut **tx)
        .await?;
    for a in rows {
        sqlx::query(
            "INSERT INTO planning_blocks(id,decision_id,task_id,date,minutes) VALUES(?,?,?,?,?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(id)
        .bind(&a.task_id)
        .bind(&a.date)
        .bind(a.minutes)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
pub(super) async fn write(
    tx: &mut Transaction<'_, Sqlite>,
    week: &str,
    action: &Action,
    timestamp: &str,
) -> Result<()> {
    let snapshot = store::load(tx, week).await?;
    let days = snapshot.budgets.clone();
    let tasks = store::live_tasks(&snapshot);
    let entities = store::live_entities(&snapshot);
    let hash = basis(&snapshot, &days)?;
    match action {
        Action::Propose { id } => {
            validate_id(id)?;
            if !super::super::receipt(tx, id, &(week, action)).await? {
                return Ok(());
            }
            let plan = planner::plan(&tasks, &entities, days, snapshot.preferences.side_quest_max);
            planner::validate_allocations(
                &tasks,
                &entities,
                &plan.days,
                snapshot.preferences.side_quest_max,
                &plan.allocations,
            )?;
            sqlx::query("INSERT INTO planning_decisions(id,week_start,status,reason,before_json,after_json,basis_hash,created_at,updated_at) VALUES(?,?,'proposed',?,?,?,?,?,?)").bind(id).bind(week).bind(&plan.reason).bind(serde_json::to_string(&snapshot.blocks).map_err(|_|DbError::invalid())?).bind(serde_json::to_string(&plan).map_err(|_|DbError::invalid())?).bind(hash).bind(timestamp).bind(timestamp).execute(&mut **tx).await?;
        }
        Action::Decide {
            id,
            operation,
            allocations,
        } => {
            validate_id(id)?;
            let record = snapshot
                .decisions
                .iter()
                .find(|d| &d.id == id)
                .ok_or_else(conflict)?;
            if (operation == "accept" && record.status == "accepted")
                || (operation == "reject" && record.status == "rejected")
                || (operation == "undo" && record.status == "undone")
            {
                return Ok(());
            }
            if operation == "modify"
                && record.status == "modified"
                && allocations.as_ref().is_some_and(|a| {
                    canonical(a.clone()) == canonical(record.after.allocations.clone())
                })
            {
                return Ok(());
            }
            if operation == "undo" {
                if !["accepted", "modified"].contains(&record.status.as_str())
                    || canonical(snapshot.blocks.clone())
                        != canonical(record.after.allocations.clone())
                {
                    return Err(conflict());
                }
                let latest:Option<String>=sqlx::query_scalar("SELECT id FROM planning_decisions WHERE week_start=? AND status IN ('accepted','modified') ORDER BY updated_at DESC,rowid DESC LIMIT 1").bind(week).fetch_optional(&mut **tx).await?;
                if latest.as_deref() != Some(id) {
                    return Err(conflict());
                }
                planner::validate_allocations(
                    &tasks,
                    &entities,
                    &days,
                    snapshot.preferences.side_quest_max,
                    &record.before,
                )?;
                blocks(tx, week, id, &record.before).await?;
                sqlx::query(
                    "UPDATE planning_decisions SET status='undone',updated_at=? WHERE id=?",
                )
                .bind(timestamp)
                .bind(id)
                .execute(&mut **tx)
                .await?;
                return Ok(());
            }
            if record.status != "proposed" {
                return Err(conflict());
            }
            if operation == "reject" {
                sqlx::query(
                    "UPDATE planning_decisions SET status='rejected',updated_at=? WHERE id=?",
                )
                .bind(timestamp)
                .bind(id)
                .execute(&mut **tx)
                .await?;
                return Ok(());
            }
            let old_hash: String =
                sqlx::query_scalar("SELECT basis_hash FROM planning_decisions WHERE id=?")
                    .bind(id)
                    .fetch_one(&mut **tx)
                    .await?;
            if old_hash != hash {
                return Err(conflict());
            }
            let mut after = record.after.clone();
            let status = match operation.as_str() {
                "accept" => "accepted",
                "modify" => {
                    after.allocations = allocations.clone().ok_or_else(DbError::invalid)?;
                    after.reason = format!("用户修改建议。{}", after.reason);
                    "modified"
                }
                _ => return Err(DbError::invalid()),
            };
            planner::validate_allocations(
                &tasks,
                &entities,
                &days,
                snapshot.preferences.side_quest_max,
                &after.allocations,
            )?;
            if status == "modified" {
                let original = planner::plan(
                    &tasks,
                    &entities,
                    days.clone(),
                    snapshot.preferences.side_quest_max,
                );
                let planned: std::collections::BTreeMap<_, _> = tasks
                    .iter()
                    .map(|t| {
                        (
                            t.id.clone(),
                            after
                                .allocations
                                .iter()
                                .filter(|a| a.task_id == t.id)
                                .map(|a| a.minutes)
                                .sum::<i64>(),
                        )
                    })
                    .collect();
                for e in &entities {
                    let belongs = |t: &PlanningTask| match e.kind.as_str() {
                        "journey" => t.journey_id.as_deref() == Some(&e.id),
                        "subject" => t.subject_id.as_deref() == Some(&e.id),
                        "quest" => t.quest_id.as_deref() == Some(&e.id),
                        _ => false,
                    };
                    let before: i64 = original
                        .allocations
                        .iter()
                        .filter(|a| tasks.iter().any(|t| t.id == a.task_id && belongs(t)))
                        .map(|a| a.minutes)
                        .sum();
                    let now: i64 = tasks
                        .iter()
                        .filter(|t| belongs(t))
                        .map(|t| planned.get(&t.id).copied().unwrap_or(0))
                        .sum();
                    if now < before.min(e.data.minimum_minutes) {
                        return Err(DbError::new(
                            "MINIMUM_PROTECTED",
                            "修改压缩了已可满足的最低分配，请修改容量或目标约束后重新生成。",
                        ));
                    }
                }
                after.days = days;
                after.unscheduled = tasks
                    .iter()
                    .filter_map(|t| {
                        let n = t.remaining_minutes - planned.get(&t.id).copied().unwrap_or(0);
                        (n > 0).then(|| (t.id.clone(), n))
                    })
                    .collect();
                after.warnings = original.warnings;
                let available: i64 = after.days.iter().map(|d| d.available).sum();
                let used: i64 = after.allocations.iter().map(|a| a.minutes).sum();
                after.reason=format!("用户修改建议：可用 {available} 分钟，安排 {used} 分钟，剩余 {} 分钟；容量未增加。",available-used);
            }
            blocks(tx, week, id, &after.allocations).await?;
            sqlx::query("UPDATE planning_decisions SET status=?,reason=?,after_json=?,updated_at=? WHERE id=?").bind(status).bind(&after.reason).bind(serde_json::to_string(&after).map_err(|_|DbError::invalid())?).bind(timestamp).bind(id).execute(&mut **tx).await?;
        }
        _ => return Err(DbError::invalid()),
    }
    Ok(())
}

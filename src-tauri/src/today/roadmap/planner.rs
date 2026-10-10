use super::model::*;
use crate::database::{DbError, Result};
use std::collections::{BTreeMap, BTreeSet};

fn group(task: &PlanningTask, entity: &Entity) -> bool {
    match entity.kind.as_str() {
        "journey" => task.journey_id.as_deref() == Some(&entity.id),
        "subject" => task.subject_id.as_deref() == Some(&entity.id),
        "quest" => task.quest_id.as_deref() == Some(&entity.id),
        _ => false,
    }
}
fn eligible(task: &PlanningTask, day: &DayBudget) -> bool {
    task.options
        .earliest_start
        .as_ref()
        .is_none_or(|d| d <= &day.date)
        && task.deadline.as_ref().is_none_or(|d| d >= &day.date)
        && !task.options.fixed_time
}
pub fn validate_allocations(
    tasks: &[PlanningTask],
    entities: &[Entity],
    days: &[DayBudget],
    side_max: i64,
    allocations: &[Allocation],
) -> Result<()> {
    let mut seen = BTreeSet::new();
    let mut daily = BTreeMap::<String, i64>::new();
    let mut total = BTreeMap::<String, i64>::new();
    if allocations.len() > 1400 {
        return Err(DbError::invalid());
    }
    for a in allocations {
        let task = tasks
            .iter()
            .find(|t| t.id == a.task_id)
            .ok_or_else(DbError::invalid)?;
        let day = days
            .iter()
            .find(|d| d.date == a.date)
            .ok_or_else(DbError::invalid)?;
        if a.minutes <= 0
            || a.minutes > 1440
            || !eligible(task, day)
            || !seen.insert((&a.task_id, &a.date))
        {
            return Err(DbError::invalid());
        }
        *daily.entry(a.date.clone()).or_default() += a.minutes;
        *total.entry(a.task_id.clone()).or_default() += a.minutes;
    }
    for day in days {
        if daily.get(&day.date).copied().unwrap_or(0) > day.available {
            return Err(DbError::new(
                "BUDGET_EXCEEDED",
                "安排超出可用容量，修改不会增加时间预算。",
            ));
        }
    }
    for task in tasks {
        let n = total.get(&task.id).copied().unwrap_or(0);
        if n > task.remaining_minutes
            || (!task.options.splittable
                && n != 0
                && (n != task.remaining_minutes
                    || allocations.iter().filter(|a| a.task_id == task.id).count() != 1))
        {
            return Err(DbError::invalid());
        }
        let parts: Vec<_> = allocations
            .iter()
            .filter(|a| a.task_id == task.id)
            .collect();
        if task.options.splittable
            && parts
                .iter()
                .filter(|a| a.minutes < task.options.minimum_session)
                .count()
                > usize::from(n == task.remaining_minutes)
        {
            return Err(DbError::invalid());
        }
    }
    for entity in entities {
        let n: i64 = tasks
            .iter()
            .filter(|t| group(t, entity))
            .map(|t| total.get(&t.id).copied().unwrap_or(0))
            .sum();
        if n > entity.data.maximum_minutes {
            return Err(DbError::new(
                "BUDGET_EXCEEDED",
                "安排超过目标、学科或支线每周上限。",
            ));
        }
    }
    let side: i64 = tasks
        .iter()
        .filter(|t| t.quest_id.is_some())
        .map(|t| total.get(&t.id).copied().unwrap_or(0))
        .sum();
    if side > side_max {
        return Err(DbError::new("BUDGET_EXCEEDED", "支线投入超过每周上限。"));
    }
    Ok(())
}
pub fn plan(
    tasks: &[PlanningTask],
    entities: &[Entity],
    days: Vec<DayBudget>,
    side_max: i64,
) -> Plan {
    let mut allocations = Vec::<Allocation>::new();
    let mut ordered: Vec<_> = tasks.iter().collect();
    ordered.sort_by_key(|t| {
        let available: i64 = days
            .iter()
            .filter(|d| eligible(t, d))
            .map(|d| d.available)
            .sum();
        (
            t.deadline.clone().unwrap_or_else(|| "9999-12-31".into()),
            available - t.remaining_minutes,
            -t.priority,
            t.id.clone(),
        )
    });
    // Reserve minima, then preferred budgets, then remaining eligible work. Never enlarge capacity.
    for tier in 0..3 {
        loop {
            let count_before: i64 = allocations.iter().map(|a| a.minutes).sum();
            for task in &ordered {
                let used: i64 = allocations
                    .iter()
                    .filter(|a| a.task_id == task.id)
                    .map(|a| a.minutes)
                    .sum();
                let mut remaining = task.remaining_minutes - used;
                if remaining <= 0 {
                    continue;
                }
                if tier < 2
                    && !entities.iter().any(|e| {
                        group(task, e) && {
                            let goal = if tier == 0 {
                                e.data.minimum_minutes
                            } else {
                                e.data.preferred_minutes
                            };
                            let n: i64 = allocations
                                .iter()
                                .filter(|a| tasks.iter().any(|t| t.id == a.task_id && group(t, e)))
                                .map(|a| a.minutes)
                                .sum();
                            n < goal
                        }
                    })
                {
                    continue;
                }
                for day in &days {
                    if !eligible(task, day) {
                        continue;
                    }
                    let free = day.available
                        - allocations
                            .iter()
                            .filter(|a| a.date == day.date)
                            .map(|a| a.minutes)
                            .sum::<i64>();
                    let mut amount =
                        remaining
                            .min(free)
                            .min(if task.options.splittable && tier < 2 {
                                task.options.preferred_session
                            } else {
                                remaining
                            });
                    for e in entities.iter().filter(|e| group(task, e)) {
                        let spent: i64 = allocations
                            .iter()
                            .filter(|a| tasks.iter().any(|t| t.id == a.task_id && group(t, e)))
                            .map(|a| a.minutes)
                            .sum();
                        let goal = if tier == 0 {
                            e.data.minimum_minutes
                        } else {
                            e.data.preferred_minutes
                        };
                        let cap = if tier < 2 && goal > spent {
                            goal
                        } else {
                            e.data.maximum_minutes
                        };
                        amount = amount.min((cap - spent).max(0));
                    }
                    if task.quest_id.is_some() {
                        let spent: i64 = allocations
                            .iter()
                            .filter(|a| {
                                tasks
                                    .iter()
                                    .any(|t| t.id == a.task_id && t.quest_id.is_some())
                            })
                            .map(|a| a.minutes)
                            .sum();
                        amount = amount.min((side_max - spent).max(0))
                    }
                    if amount <= 0
                        || (!task.options.splittable && amount != remaining)
                        || (task.options.splittable
                            && amount < task.options.minimum_session
                            && amount != remaining)
                    {
                        continue;
                    }
                    if let Some(a) = allocations
                        .iter_mut()
                        .find(|a| a.task_id == task.id && a.date == day.date)
                    {
                        a.minutes += amount
                    } else {
                        allocations.push(Allocation {
                            task_id: task.id.clone(),
                            date: day.date.clone(),
                            minutes: amount,
                        })
                    }
                    remaining -= amount;
                    if remaining == 0 {
                        break;
                    }
                }
            }
            if allocations.iter().map(|a| a.minutes).sum::<i64>() == count_before {
                break;
            }
        }
    }
    let unscheduled: Vec<_> = tasks
        .iter()
        .filter_map(|t| {
            let left = t.remaining_minutes
                - allocations
                    .iter()
                    .filter(|a| a.task_id == t.id)
                    .map(|a| a.minutes)
                    .sum::<i64>();
            (left > 0).then(|| (t.id.clone(), left))
        })
        .collect();
    let mut warnings = Vec::new();
    for e in entities {
        let n: i64 = allocations
            .iter()
            .filter(|a| tasks.iter().any(|t| t.id == a.task_id && group(t, e)))
            .map(|a| a.minutes)
            .sum();
        if n < e.data.minimum_minutes {
            warnings.push(format!(
                "{}：最低分配不足 {} 分钟；未增加容量。",
                e.title,
                e.data.minimum_minutes - n
            ))
        }
    }
    if !unscheduled.is_empty() {
        warnings.push("有任务未完全安排：请调整期限、容量或任务范围；不会自动顺延逾期任务。".into())
    }
    let available: i64 = days.iter().map(|d| d.available).sum();
    let used: i64 = allocations.iter().map(|a| a.minutes).sum();
    Plan{days,allocations,unscheduled,warnings,reason:format!("先保留最低分配，再按偏好与截止日期/slack 安排。固定事件扣除后可用 {available} 分钟，安排 {used} 分钟，空余 {} 分钟。支线上限 {side_max} 分钟；总容量不增加。",available-used)}
}

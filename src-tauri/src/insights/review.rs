use super::*;
pub(super) async fn action(db: &Database, a: Action) -> Result<()> {
    crate::database::repository::validate_id(a.request_id())?;
    let payload = serde_json::to_vec(&a).map_err(|_| DbError::invalid())?;
    let hash = checksum(&payload);
    let mut tx = db.pool.begin_with("BEGIN IMMEDIATE").await?;
    let old: Option<String> =
        sqlx::query_scalar("SELECT payload_hash FROM insights_receipts WHERE id=?")
            .bind(a.request_id())
            .fetch_optional(&mut *tx)
            .await?;
    if let Some(old) = old {
        if old != hash {
            return Err(conflict());
        }
        tx.commit().await?;
        return Ok(());
    }
    match &a {
        Action::Preferences {
            revision, value, ..
        } => {
            if value
                .weights
                .iter()
                .any(|v| !v.is_finite() || *v < 0.0 || *v > 100.0)
                || value
                    .thresholds
                    .iter()
                    .any(|v| !v.is_finite() || *v <= 0.0 || *v > 100000.0)
                || value.thresholds.windows(2).any(|w| w[0] >= w[1])
                || value.stop_words.len() > 500
                || value
                    .stop_words
                    .iter()
                    .any(|s| s.trim().is_empty() || s.len() > 100)
            {
                return Err(DbError::invalid());
            }
            let current: i64 = sqlx::query_scalar(
                "SELECT COALESCE((SELECT revision FROM insights_preferences WHERE id=1),0)",
            )
            .fetch_one(&mut *tx)
            .await?;
            if current != *revision {
                return Err(conflict());
            }
            sqlx::query("INSERT INTO insights_preferences(id,payload,revision,updated_at) VALUES(1,?,1,?) ON CONFLICT(id) DO UPDATE SET payload=excluded.payload,revision=insights_preferences.revision+1,updated_at=excluded.updated_at").bind(serde_json::to_string(value).map_err(|_|DbError::invalid())?).bind(now()).execute(&mut *tx).await?;
        }
        Action::Day {
            date: d,
            revision,
            status,
            ..
        } => {
            date(d)?;
            if ![
                "normal",
                "rest",
                "sick",
                "exam",
                "travel",
                "paper_deadline",
                "special",
            ]
            .contains(&status.as_str())
            {
                return Err(DbError::invalid());
            }
            let current: i64 = sqlx::query_scalar(
                "SELECT COALESCE((SELECT revision FROM insights_days WHERE date=?),0)",
            )
            .bind(d)
            .fetch_one(&mut *tx)
            .await?;
            if current != *revision {
                return Err(conflict());
            }
            sqlx::query("INSERT INTO insights_days(date,status,revision,updated_at) VALUES(?,?,1,?) ON CONFLICT(date) DO UPDATE SET status=excluded.status,revision=insights_days.revision+1,updated_at=excluded.updated_at").bind(d).bind(status).bind(now()).execute(&mut *tx).await?;
        }
        Action::Review {
            week,
            revision,
            body,
            next_focus,
            ..
        } => {
            let week = monday(week)?;
            if body.len() > 100000 || next_focus.len() > 10000 {
                return Err(DbError::invalid());
            }
            let current: i64 = sqlx::query_scalar(
                "SELECT COALESCE((SELECT revision FROM weekly_reviews WHERE week_start_date=?),0)",
            )
            .bind(&week)
            .fetch_one(&mut *tx)
            .await?;
            if current != *revision {
                return Err(conflict());
            }
            sqlx::query("INSERT INTO weekly_reviews(id,week_start_date,body,next_focus,revision,created_at,updated_at) VALUES(?,?,?,?,1,?,?) ON CONFLICT(week_start_date) DO UPDATE SET body=excluded.body,next_focus=excluded.next_focus,revision=weekly_reviews.revision+1,updated_at=excluded.updated_at,deleted_at=NULL").bind(uuid::Uuid::new_v4().to_string()).bind(week).bind(body).bind(next_focus).bind(now()).bind(now()).execute(&mut *tx).await?;
        }
    }
    sqlx::query("INSERT INTO insights_receipts(id,payload_hash,created_at) VALUES(?,?,?)")
        .bind(a.request_id())
        .bind(hash)
        .bind(now())
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
pub(super) async fn get(db: &Database, week: &str) -> Result<Value> {
    let week = monday(week)?;
    let until = (date(&week)? + Duration::days(6)).to_string();
    let facts = db
        .insights_snapshot(Filter {
            from: week.clone(),
            until,
            today: week.clone(),
        })
        .await?;
    let user=sqlx::query("SELECT body,next_focus,revision FROM weekly_reviews WHERE week_start_date=? AND deleted_at IS NULL").bind(&week).fetch_optional(&db.pool).await?;
    Ok(
        json!({"week":week,"facts":facts,"body":user.as_ref().map(|r|r.get::<String,_>("body")).unwrap_or_default(),"next_focus":user.as_ref().map(|r|r.get::<String,_>("next_focus")).unwrap_or_default(),"revision":user.map(|r|r.get::<i64,_>("revision")).unwrap_or(0)}),
    )
}
pub(super) async fn month(db: &Database, month: &str, today: &str) -> Result<Value> {
    let from = date(&format!("{month}-01"))?;
    let today = date(today)?;
    if from > today {
        return Err(DbError::invalid());
    }
    let next = if from.month() == 12 {
        NaiveDate::from_ymd_opt(from.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(from.year(), from.month() + 1, 1)
    }
    .ok_or_else(DbError::invalid)?;
    let until = (next - Duration::days(1)).min(today);
    if next <= today {
        if let Some(old) = sqlx::query_scalar::<_, String>(
            "SELECT summary FROM monthly_snapshots WHERE month=? AND deleted_at IS NULL",
        )
        .bind(month)
        .fetch_optional(&db.pool)
        .await?
        {
            return serde_json::from_str(&old).map_err(|_| DbError::invalid());
        }
    }
    let facts = db
        .insights_snapshot(Filter {
            from: from.to_string(),
            until: until.to_string(),
            today: today.to_string(),
        })
        .await?;
    // Private export whitelist: aggregate labels/words only; no bodies, record titles, UUIDs, source links or reflection.
    let counts=aggregate::values(db,"SELECT json_object('competitions',(SELECT count(*) FROM competitions WHERE deleted_at IS NULL AND event_date BETWEEN r.a AND r.b),'achievements',(SELECT count(*) FROM artifacts WHERE deleted_at IS NULL AND kind='achievement' AND date(created_at,'localtime') BETWEEN r.a AND r.b),'projects',(SELECT count(*) FROM artifacts WHERE deleted_at IS NULL AND kind NOT IN ('achievement','photo') AND date(created_at,'localtime') BETWEEN r.a AND r.b),'photos',(SELECT count(*) FROM artifacts WHERE deleted_at IS NULL AND kind='photo' AND date(created_at,'localtime') BETWEEN r.a AND r.b)) FROM (SELECT ? a,? b) r",&from.to_string(),&until.to_string()).await?.into_iter().next().unwrap_or(json!({}));
    let summary = json!({"version":1,"visibility":"private","month":month,"from":from.to_string(),"until":until.to_string(),"generated_at":now(),"closed":next<=today,"totals":{"deep_minutes":facts.days.iter().map(|d|d.deep_minutes).sum::<f64>(),"coding_minutes":facts.days.iter().map(|d|d.coding_minutes).sum::<f64>(),"planned_minutes":facts.days.iter().map(|d|d.planned_minutes).sum::<f64>(),"journals":facts.days.iter().map(|d|d.journal_count).sum::<i64>(),"tasks_completed":facts.days.iter().map(|d|d.tasks_completed).sum::<i64>()},"days":facts.days.iter().map(|d|json!({"date":d.date,"deep_minutes":d.deep_minutes,"coding_minutes":d.coding_minutes,"tasks_completed":d.tasks_completed,"journal_count":d.journal_count,"problem_count":d.problem_count,"music_count":d.music_count,"planned_minutes":d.planned_minutes,"capacity_minutes":d.capacity_minutes,"status":d.status,"intensity":d.intensity})).collect::<Vec<_>>(),"subjects":facts.subjects,"topics":facts.topics,"music":facts.music,"words":facts.words.iter().map(|w|json!({"word":w.word,"count":w.count})).collect::<Vec<_>>(),"counts":counts,"task_summary":facts.task_summary});
    let mut summary = summary;
    let highlights=aggregate::values(db,"SELECT json_object('kind','journal','id',id,'date',entry_date,'title',title) FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0 AND entry_date BETWEEN ? AND ? ORDER BY entry_date DESC,id LIMIT 20",&from.to_string(),&until.to_string()).await?;
    summary["highlights"] = json!(highlights);
    let encoded = serde_json::to_string(&summary).map_err(|_| DbError::invalid())?;
    sqlx::query("INSERT INTO monthly_snapshots(id,month,summary,revision,created_at,updated_at) VALUES(?,?,?,1,?,?) ON CONFLICT(month) DO UPDATE SET summary=excluded.summary,revision=monthly_snapshots.revision+1,updated_at=excluded.updated_at,deleted_at=NULL WHERE json_extract(monthly_snapshots.summary,'$.closed') IS NOT 1").bind(uuid::Uuid::new_v4().to_string()).bind(month).bind(encoded).bind(now()).bind(now()).execute(&db.pool).await?;
    let stored: String = sqlx::query_scalar("SELECT summary FROM monthly_snapshots WHERE month=?")
        .bind(month)
        .fetch_one(&db.pool)
        .await?;
    serde_json::from_str(&stored).map_err(|_| DbError::invalid())
}

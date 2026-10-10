use super::*;
use std::collections::BTreeMap;
pub(super) async fn preferences(db: &Database) -> Result<(Preferences, i64)> {
    let row = sqlx::query("SELECT payload,revision FROM insights_preferences WHERE id=1")
        .fetch_optional(&db.pool)
        .await?;
    match row {
        Some(r) => Ok((
            serde_json::from_str(&r.get::<String, _>("payload")).map_err(|_| DbError::invalid())?,
            r.get("revision"),
        )),
        None => Ok((Preferences::default(), 0)),
    }
}
pub(super) async fn values(
    db: &Database,
    sql: &str,
    from: &str,
    until: &str,
) -> Result<Vec<Value>> {
    sqlx::query_scalar::<_, String>(sql)
        .bind(from)
        .bind(until)
        .fetch_all(&db.pool)
        .await?
        .into_iter()
        .map(|s| serde_json::from_str(&s).map_err(|_| DbError::invalid()))
        .collect()
}
pub(super) async fn days(
    db: &Database,
    from: &str,
    until: &str,
    p: &Preferences,
) -> Result<Vec<Day>> {
    let a = date(from)?;
    let b = date(until)?;
    let default: Option<String> = sqlx::query_scalar(
        "SELECT value_text FROM settings WHERE key='daily_capacity_minutes' AND deleted_at IS NULL",
    )
    .fetch_optional(&db.pool)
    .await?;
    let default = default.and_then(|s| s.parse::<i64>().ok()).unwrap_or(210);
    let capacity =
        sqlx::query("SELECT weekday_minutes,weekend_minutes FROM planning_preferences WHERE id=1")
            .fetch_optional(&db.pool)
            .await?;
    let mut map = BTreeMap::new();
    for i in 0..=(b - a).num_days() {
        let d = a + Duration::days(i);
        let day = d.to_string();
        let c = capacity
            .as_ref()
            .map(|r| {
                r.get(if d.weekday().num_days_from_monday() < 5 {
                    "weekday_minutes"
                } else {
                    "weekend_minutes"
                })
            })
            .unwrap_or(default);
        map.insert(
            day.clone(),
            Day {
                date: day,
                status: "normal".into(),
                capacity_minutes: c,
                ..Day::default()
            },
        );
    }
    // Aggregate facts first, then combine by day. Planned time is never actual time.
    let sql="WITH signal AS (
 SELECT i.local_date d,0 k,SUM(i.elapsed_ms)/60000.0 v FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id WHERE f.deleted_at IS NULL GROUP BY i.local_date
 UNION ALL SELECT i.local_date,1,SUM(i.elapsed_ms)/60000.0 FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id JOIN coding_attempts a ON a.focus_session_id=f.id JOIN coding_problems p ON p.id=a.problem_id WHERE f.deleted_at IS NULL AND a.deleted_at IS NULL AND p.deleted_at IS NULL GROUP BY i.local_date
 UNION ALL SELECT a.attempted_on,1,SUM(a.manual_seconds)/60.0 FROM coding_attempts a JOIN coding_problems p ON p.id=a.problem_id WHERE a.deleted_at IS NULL AND p.deleted_at IS NULL GROUP BY a.attempted_on
 UNION ALL SELECT due_date,2,COUNT(*) FROM tasks WHERE deleted_at IS NULL AND lifecycle='done' GROUP BY due_date
 UNION ALL SELECT entry_date,3,COUNT(*) FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0 GROUP BY entry_date
 UNION ALL SELECT a.attempted_on,4,COUNT(DISTINCT a.problem_id) FROM coding_attempts a JOIN coding_problems p ON p.id=a.problem_id WHERE a.deleted_at IS NULL AND p.deleted_at IS NULL GROUP BY a.attempted_on
 UNION ALL SELECT date(m.played_at,'localtime'),5,COUNT(*) FROM music_plays m JOIN music_tracks t ON t.id=m.track_id WHERE m.deleted_at IS NULL AND t.deleted_at IS NULL GROUP BY date(m.played_at,'localtime')
 UNION ALL SELECT due_date,6,SUM(estimated_minutes) FROM tasks WHERE deleted_at IS NULL AND lifecycle!='cancelled' GROUP BY due_date
 ) SELECT d,k,CAST(SUM(v) AS REAL) v FROM signal WHERE d BETWEEN ? AND ? GROUP BY d,k";
    for r in sqlx::query(sql)
        .bind(from)
        .bind(until)
        .fetch_all(&db.pool)
        .await?
    {
        if let Some(d) = map.get_mut(&r.get::<String, _>("d")) {
            let v: f64 = r.get("v");
            match r.get::<i64, _>("k") {
                0 => d.deep_minutes = v,
                1 => d.coding_minutes = v,
                2 => d.tasks_completed = v as i64,
                3 => d.journal_count = v as i64,
                4 => d.problem_count = v as i64,
                5 => d.music_count = v as i64,
                6 => d.planned_minutes = v,
                _ => {}
            }
        }
    }
    for r in sqlx::query("SELECT date,minutes,status FROM capacity_days WHERE date BETWEEN ? AND ?")
        .bind(from)
        .bind(until)
        .fetch_all(&db.pool)
        .await?
    {
        if let Some(d) = map.get_mut(&r.get::<String, _>("date")) {
            d.capacity_minutes = r.get("minutes");
            d.status = r.get("status");
        }
    }
    for r in
        sqlx::query("SELECT date,status,revision FROM insights_days WHERE date BETWEEN ? AND ?")
            .bind(from)
            .bind(until)
            .fetch_all(&db.pool)
            .await?
    {
        if let Some(d) = map.get_mut(&r.get::<String, _>("date")) {
            d.status = r.get("status");
            d.status_revision = r.get("revision");
        }
    }
    for d in map.values_mut() {
        let signals = [
            d.deep_minutes,
            d.coding_minutes,
            d.tasks_completed as f64,
            d.journal_count as f64,
            d.problem_count as f64,
            d.music_count as f64,
        ];
        let score: f64 = signals.iter().zip(p.weights).map(|(v, w)| v * w).sum();
        d.intensity = p.thresholds.iter().filter(|t| score >= **t).count() as u8;
    }
    Ok(map.into_values().collect())
}
pub(super) async fn records(db: &Database, from: &str, until: &str) -> Result<Vec<Record>> {
    let sql = RECORD_SQL;
    Ok(sqlx::query(sql)
        .bind(from)
        .bind(until)
        .fetch_all(&db.pool)
        .await?
        .into_iter()
        .map(|r| Record {
            kind: r.get("kind"),
            id: r.get("id"),
            date: r.get("date"),
            title: r.get("title"),
        })
        .collect())
}
const RECORD_SQL:&str="WITH records AS (
 SELECT 'journal' kind,id,entry_date date,title FROM journal_entries WHERE deleted_at IS NULL AND is_draft=0
 UNION ALL SELECT 'task',id,due_date,title FROM tasks WHERE deleted_at IS NULL
 UNION ALL SELECT 'event',id,event_date,title FROM calendar_events WHERE deleted_at IS NULL
 UNION ALL SELECT 'contest',id,event_date,title FROM competitions WHERE deleted_at IS NULL
 UNION ALL SELECT 'problem',a.id,a.attempted_on,p.title FROM coding_attempts a JOIN coding_problems p ON p.id=a.problem_id WHERE a.deleted_at IS NULL AND p.deleted_at IS NULL
 UNION ALL SELECT 'music',p.id,date(p.played_at,'localtime'),t.title FROM music_plays p JOIN music_tracks t ON t.id=p.track_id WHERE p.deleted_at IS NULL AND t.deleted_at IS NULL
 UNION ALL SELECT 'study',f.id,i.local_date,COALESCE(t.title,'独立专注') FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id LEFT JOIN tasks t ON t.id=f.task_id WHERE f.deleted_at IS NULL
 UNION ALL SELECT CASE WHEN kind='photo' THEN 'photo' WHEN kind='achievement' THEN 'achievement' ELSE 'project' END,id,date(created_at,'localtime'),title FROM artifacts WHERE deleted_at IS NULL
 ) SELECT DISTINCT kind,id,date,title FROM records WHERE date BETWEEN ? AND ? ORDER BY date DESC,kind,id LIMIT 500";
pub(super) async fn memories(db: &Database, today: &str) -> Result<Vec<Record>> {
    date(today)?;
    if !preferences(db).await?.0.memories {
        return Ok(vec![]);
    }
    let sql=RECORD_SQL.replace("date BETWEEN ? AND ?","date<? AND substr(date,6,5)=? AND kind IN ('journal','music','study','achievement','photo')").replace("LIMIT 500","LIMIT 5");
    Ok(sqlx::query(&sql)
        .bind(today)
        .bind(&today[5..])
        .fetch_all(&db.pool)
        .await?
        .into_iter()
        .map(|r| Record {
            kind: r.get("kind"),
            id: r.get("id"),
            date: r.get("date"),
            title: r.get("title"),
        })
        .collect())
}
fn sums(days: &[Day]) -> Value {
    json!({"deep_minutes":days.iter().map(|d|d.deep_minutes).sum::<f64>(),"coding_minutes":days.iter().map(|d|d.coding_minutes).sum::<f64>(),"planned_minutes":days.iter().map(|d|d.planned_minutes).sum::<f64>(),"tasks_completed":days.iter().map(|d|d.tasks_completed).sum::<i64>(),"journals":days.iter().map(|d|d.journal_count).sum::<i64>()})
}
pub(super) async fn snapshot(db: &Database, f: Filter) -> Result<Snapshot> {
    range(&f)?;
    let (p, revision) = preferences(db).await?;
    let today = date(&f.today)?;
    let series = days(db, &f.from, &f.until, &p).await?;
    let heatmap = days(
        db,
        &format!("{}-01-01", today.year()),
        &format!("{}-12-31", today.year()),
        &p,
    )
    .await?;
    let recent = days(db, &(today - Duration::days(27)).to_string(), &f.today, &p).await?;
    let consistency = |count: usize| {
        let slice = &recent[28 - count..];
        let eligible: Vec<_> = slice.iter().filter(|d| d.status == "normal").collect();
        json!({"active":eligible.iter().filter(|d|d.deep_minutes>0.0||d.coding_minutes>0.0||d.tasks_completed+d.journal_count+d.problem_count+d.music_count>0).count(),"eligible":eligible.len(),"protected":slice.len()-eligible.len()})
    };
    let subjects=values(db,"SELECT json_object('label',COALESCE(s.title,'未分科'),'minutes',SUM(i.elapsed_ms)/60000.0) FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id LEFT JOIN tasks t ON t.id=f.task_id LEFT JOIN subjects s ON s.id=t.subject_id WHERE f.deleted_at IS NULL AND i.local_date BETWEEN ? AND ? GROUP BY s.id ORDER BY SUM(i.elapsed_ms) DESC",&f.from,&f.until).await?;
    let journeys=values(db,"SELECT json_object('id',j.id,'label',j.title,'total',COUNT(t.id),'completed',COALESCE(SUM(t.lifecycle='done'),0)) FROM journeys j LEFT JOIN tasks t ON t.journey_id=j.id AND t.deleted_at IS NULL AND t.due_date BETWEEN ? AND ? WHERE j.deleted_at IS NULL AND j.archived_at IS NULL GROUP BY j.id ORDER BY j.title",&f.from,&f.until).await?;
    let music=values(db,"SELECT json_object('label',t.title,'artist',t.artist,'album',t.album,'count',COUNT(*),'seconds',COALESCE(SUM(p.duration_seconds),0)) FROM music_plays p JOIN music_tracks t ON t.id=p.track_id WHERE p.deleted_at IS NULL AND t.deleted_at IS NULL AND date(p.played_at,'localtime') BETWEEN ? AND ? GROUP BY t.id ORDER BY COUNT(*) DESC,t.id",&f.from,&f.until).await?;
    let topics=values(db,"SELECT json_object('label',j.value,'count',COUNT(DISTINCT a.problem_id)) FROM coding_attempts a JOIN coding_problems p ON p.id=a.problem_id,json_each(p.tags_json) j WHERE a.deleted_at IS NULL AND p.deleted_at IS NULL AND a.attempted_on BETWEEN ? AND ? GROUP BY j.value ORDER BY COUNT(DISTINCT a.problem_id) DESC LIMIT 30",&f.from,&f.until).await?;
    let task_summary=values(db,"SELECT json_object('total',COUNT(*),'completed',COALESCE(SUM(lifecycle='done'),0),'unfinished',COALESCE(SUM(lifecycle IN ('planned','active')),0),'moved',COALESCE(SUM(lifecycle='moved'),0)) FROM tasks WHERE deleted_at IS NULL AND lifecycle!='cancelled' AND due_date BETWEEN ? AND ?",&f.from,&f.until).await?.into_iter().next().unwrap_or(json!({}));
    let span = (date(&f.until)? - date(&f.from)?).num_days() + 1;
    let end = date(&f.from)? - Duration::days(1);
    let start = end - Duration::days(span - 1);
    let previous = sums(&days(db, &start.to_string(), &end.to_string(), &p).await?);
    let records = records(db, &f.from, &f.until).await?;
    let words = words::read(db, &f.from, &f.until, &p).await?;
    Ok(Snapshot {
        from: f.from,
        until: f.until,
        days: series,
        heatmap,
        preferences: p,
        preferences_revision: revision,
        words,
        subjects,
        journeys,
        music,
        topics,
        records,
        task_summary,
        consistency: json!({"days7":consistency(7),"days28":consistency(28)}),
        previous,
    })
}

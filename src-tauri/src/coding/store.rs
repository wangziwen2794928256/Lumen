use super::*;
fn problem(r: sqlx::sqlite::SqliteRow) -> Result<Problem> {
    Ok(Problem {
        id: r.get("id"),
        revision: r.get("revision"),
        deleted_at: r.get("deleted_at"),
        input: ProblemInput {
            title: r.get("title"),
            platform: r.get("platform"),
            external_id: r.get("external_id"),
            source_url: r.get::<Option<String>, _>("source_url").unwrap_or_default(),
            difficulty: r
                .get::<Option<String>, _>("difficulty")
                .unwrap_or_else(|| "unknown".into()),
            tags: serde_json::from_str(r.get::<String, _>("tags_json").as_str())
                .map_err(|_| DbError::invalid())?,
            notes: r.get("notes"),
            journey_id: r.get("journey_id"),
            task_id: r.get("task_id"),
            competition_id: r.get("competition_id"),
        },
        attempt_count: r.get("attempt_count"),
        solved: r.get::<i64, _>("solved") > 0,
        first_attempt: r.get("first_attempt"),
        solved_at: r.get("solved_at"),
        language: r.get("language"),
        manual_seconds: r.get("manual_seconds"),
        focus_seconds: r.get("focus_seconds"),
    })
}
pub(super) fn validate(f: &Filter) -> Result<()> {
    date(&f.week)?;
    if f.query.len() > 500
        || f.tag.len() > 100
        || !(0..=1000000).contains(&f.page)
        || !["", "unattempted", "unsolved", "solved"].contains(&f.status.as_str())
    {
        return Err(DbError::invalid());
    }
    if !f.platform.is_empty() {
        platform(&f.platform)?;
    }
    if let Some(s) = &f.selected {
        id(s)?;
    }
    Ok(())
}
fn predicate(f: &Filter) -> sqlx::QueryBuilder<'_, Sqlite> {
    let mut q = sqlx::QueryBuilder::new(" FROM coding_problems p WHERE ");
    q.push(if f.trash {
        "p.deleted_at IS NOT NULL"
    } else {
        "p.deleted_at IS NULL"
    });
    if !f.query.is_empty() {
        let s = format!(
            "%{}%",
            f.query
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_")
        );
        q.push(" AND (p.title LIKE ")
            .push_bind(s.clone())
            .push(" ESCAPE '\\' OR p.external_id LIKE ")
            .push_bind(s)
            .push(" ESCAPE '\\')");
    }
    if !f.platform.is_empty() {
        q.push(" AND p.platform=").push_bind(&f.platform);
    }
    if !f.tag.is_empty() {
        q.push(" AND EXISTS(SELECT 1 FROM json_each(p.tags_json) WHERE value=")
            .push_bind(&f.tag)
            .push(")");
    }
    match f.status.as_str() {
        "solved" => {
            q.push(" AND EXISTS(SELECT 1 FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL AND a.outcome='accepted')");
        }
        "unsolved" => {
            q.push(" AND NOT EXISTS(SELECT 1 FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL AND a.outcome='accepted') AND EXISTS(SELECT 1 FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL)");
        }
        "unattempted" => {
            q.push(" AND NOT EXISTS(SELECT 1 FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL)");
        }
        _ => {}
    }
    q
}
impl Database {
    pub async fn coding_snapshot(&self, f: Filter) -> Result<Snapshot> {
        validate(&f)?;
        let mut count = sqlx::QueryBuilder::<Sqlite>::new("SELECT count(*)");
        let mut pred = predicate(&f);
        let predicate_sql = pred.sql().to_owned();
        let mut built = pred.build();
        use sqlx::Execute;
        let args = built
            .take_arguments()
            .map_err(|_| DbError::invalid())?
            .unwrap();
        count.push(&predicate_sql);
        let total: i64 = sqlx::query_scalar_with(count.sql(), args.clone())
            .fetch_one(&self.pool)
            .await?;
        let query=format!("SELECT p.*,(SELECT count(*) FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL) attempt_count,(SELECT count(*) FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL AND outcome='accepted') solved,(SELECT MIN(started_at) FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL) first_attempt,(SELECT MIN(started_at) FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL AND outcome='accepted') solved_at,(SELECT language FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL ORDER BY attempted_on DESC,started_at DESC,id DESC LIMIT 1) language,(SELECT COALESCE(SUM(manual_seconds),0) FROM coding_attempts a WHERE a.problem_id=p.id AND a.deleted_at IS NULL) manual_seconds,(SELECT COALESCE(SUM(f.actual_seconds),0) FROM coding_attempts a JOIN focus_sessions f ON f.id=a.focus_session_id AND f.state='finished' WHERE a.problem_id=p.id AND a.deleted_at IS NULL) focus_seconds {} ORDER BY p.created_at DESC,p.id LIMIT 30 OFFSET ?",predicate_sql);
        use sqlx::Arguments;
        let mut args = args;
        args.add(f.page * 30).map_err(|_| DbError::invalid())?;
        let problems = sqlx::query_with(&query, args)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(problem)
            .collect::<Result<Vec<_>>>()?;
        let attempts = if let Some(key) = &f.selected {
            sqlx::query("SELECT a.*,COALESCE(f.actual_seconds,0) focus_seconds FROM coding_attempts a LEFT JOIN focus_sessions f ON f.id=a.focus_session_id AND f.state='finished' WHERE a.problem_id=? ORDER BY a.attempted_on DESC,a.started_at DESC,a.id DESC LIMIT 200").bind(key).fetch_all(&self.pool).await?.iter().map(|r|Attempt{id:r.get("id"),revision:r.get("revision"),deleted_at:r.get("deleted_at"),input:AttemptInput{problem_id:r.get("problem_id"),language:r.get::<Option<String>,_>("language").unwrap_or_default(),outcome:r.get::<Option<String>,_>("outcome").unwrap_or_else(||"unfinished".into()),notes:r.get("notes"),attempted_on:r.get::<Option<String>,_>("attempted_on").unwrap_or_default(),started_at:r.get::<Option<String>,_>("started_at").unwrap_or_default(),manual_seconds:r.get("manual_seconds"),focus_session_id:r.get("focus_session_id")},focus_seconds:r.get("focus_seconds")}).collect()
        } else {
            vec![]
        };
        let contests =
            sqlx::query("SELECT * FROM competitions ORDER BY created_at DESC,id LIMIT 500")
                .fetch_all(&self.pool)
                .await?
                .iter()
                .map(|r| Contest {
                    id: r.get("id"),
                    revision: r.get("revision"),
                    deleted_at: r.get("deleted_at"),
                    input: ContestInput {
                        title: r.get("title"),
                        platform: r.get("platform"),
                        event_date: r.get::<Option<String>, _>("event_date").unwrap_or_default(),
                        source_url: r.get::<Option<String>, _>("source_url").unwrap_or_default(),
                        participated: r.get::<i64, _>("participated") > 0,
                        notes: r.get("notes"),
                    },
                })
                .collect();
        let journeys = choices(&self.pool, "journeys").await?;
        let tasks = choices(&self.pool, "tasks").await?;
        let focus=sqlx::query("SELECT f.id,f.task_id,f.session_date,f.actual_seconds,t.title FROM focus_sessions f LEFT JOIN tasks t ON t.id=f.task_id WHERE f.state='finished' ORDER BY f.started_at DESC LIMIT 500").fetch_all(&self.pool).await?.iter().map(|r|Choice{id:r.get("id"),task_id:r.get("task_id"),title:format!("{} · {} · {}秒",r.get::<Option<String>,_>("session_date").unwrap_or_default(),r.get::<Option<String>,_>("title").unwrap_or_else(||"未关联任务".into()),r.get::<i64,_>("actual_seconds"))}).collect();
        Ok(Snapshot {
            problems,
            total,
            attempts,
            contests,
            journeys,
            tasks,
            focus,
            analytics: analytics::read(&self.pool, &f.week).await?,
        })
    }
}
async fn choices(pool: &sqlx::SqlitePool, table: &str) -> Result<Vec<Choice>> {
    Ok(sqlx::query(&format!("SELECT id,title FROM {table} WHERE deleted_at IS NULL AND archived_at IS NULL ORDER BY updated_at DESC LIMIT 500")).fetch_all(pool).await?.iter().map(|r|Choice{id:r.get("id"),title:r.get("title"),task_id:None}).collect())
}

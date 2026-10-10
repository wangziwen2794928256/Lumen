use super::*;
fn table(kind: &str) -> Result<&'static str> {
    match kind {
        "problem" => Ok("coding_problems"),
        "attempt" => Ok("coding_attempts"),
        "contest" => Ok("competitions"),
        _ => Err(DbError::invalid()),
    }
}
async fn reference(
    tx: &mut Transaction<'_, Sqlite>,
    table: &str,
    value: &Option<String>,
) -> Result<()> {
    if let Some(v) = value {
        id(v)?;
        let n:i64=sqlx::query_scalar(&format!("SELECT count(*) FROM {table} WHERE id=? AND deleted_at IS NULL AND archived_at IS NULL")).bind(v).fetch_one(&mut **tx).await?;
        if n != 1 {
            return Err(DbError::invalid());
        }
    }
    Ok(())
}
async fn prepare(
    tx: &mut Transaction<'_, Sqlite>,
    kind: &str,
    key: &str,
    revision: Option<i64>,
) -> Result<()> {
    id(key)?;
    let row = sqlx::query(&format!(
        "SELECT revision,deleted_at FROM {} WHERE id=?",
        table(kind)?
    ))
    .bind(key)
    .fetch_optional(&mut **tx)
    .await?;
    match (row, revision) {
        (None, None) => Ok(()),
        (Some(r), Some(rev))
            if r.get::<i64, _>("revision") == rev
                && r.get::<Option<String>, _>("deleted_at").is_none() =>
        {
            Ok(())
        }
        _ => Err(conflict()),
    }
}
impl Database {
    pub async fn coding_action(&self, action: Action) -> Result<()> {
        id(action.request_id())?;
        let hash = crate::database::checksum(
            &serde_json::to_vec(&action).map_err(|_| DbError::invalid())?,
        );
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        if let Some(saved) = sqlx::query_scalar::<_, String>(
            "SELECT payload_hash FROM coding_receipts WHERE request_id=?",
        )
        .bind(action.request_id())
        .fetch_optional(&mut *tx)
        .await?
        {
            if saved != hash {
                return Err(conflict());
            }
            return Ok(());
        }
        let stamp = now();
        match &action {
            Action::Problem {
                id: key,
                revision,
                input: p,
                ..
            } => {
                prepare(&mut tx, "problem", key, *revision).await?;
                platform(&p.platform)?;
                url(&p.source_url)?;
                if p.title.trim().is_empty()
                    || p.title.len() > 1000
                    || p.external_id.trim().is_empty()
                    || p.external_id.len() > 100
                    || p.notes.len() > 100000
                    || p.tags.len() > 30
                    || p.tags.iter().any(|s| s.trim().is_empty() || s.len() > 100)
                    || !["unknown", "easy", "medium", "hard"].contains(&p.difficulty.as_str())
                {
                    return Err(DbError::invalid());
                }
                reference(&mut tx, "journeys", &p.journey_id).await?;
                reference(&mut tx, "tasks", &p.task_id).await?;
                reference(&mut tx, "competitions", &p.competition_id).await?;
                if let Some(task) = &p.task_id {
                    let mismatch:i64=sqlx::query_scalar("SELECT count(*) FROM coding_attempts a JOIN focus_sessions f ON f.id=a.focus_session_id WHERE a.problem_id=? AND a.deleted_at IS NULL AND (f.task_id IS NULL OR f.task_id<>?)").bind(key).bind(task).fetch_one(&mut *tx).await?;
                    if mismatch > 0 {
                        return Err(DbError::invalid());
                    }
                }
                let duplicate:i64=sqlx::query_scalar("SELECT count(*) FROM coding_problems WHERE platform=? AND external_id=? COLLATE NOCASE AND deleted_at IS NULL AND id<>?").bind(&p.platform).bind(p.external_id.trim()).bind(key).fetch_one(&mut *tx).await?;
                if duplicate > 0 {
                    return Err(DbError::new(
                        "CODING_DUPLICATE",
                        "该平台与题号已存在，请打开原题增加尝试。",
                    ));
                }
                let tags: std::collections::BTreeSet<_> = p.tags.iter().map(|s| s.trim()).collect();
                let tags = serde_json::to_string(&tags).map_err(|_| DbError::invalid())?;
                if revision.is_none() {
                    sqlx::query("INSERT INTO coding_problems(id,title,created_at,updated_at) VALUES(?,?,?,?)").bind(key).bind(p.title.trim()).bind(&stamp).bind(&stamp).execute(&mut *tx).await?;
                }
                sqlx::query("UPDATE coding_problems SET title=?,platform=?,external_id=?,source_url=?,difficulty=?,tags_json=?,notes=?,journey_id=?,task_id=?,competition_id=?,revision=revision+1,updated_at=? WHERE id=?").bind(p.title.trim()).bind(&p.platform).bind(p.external_id.trim()).bind(&p.source_url).bind(&p.difficulty).bind(tags).bind(&p.notes).bind(&p.journey_id).bind(&p.task_id).bind(&p.competition_id).bind(&stamp).bind(key).execute(&mut *tx).await?;
            }
            Action::Attempt {
                id: key,
                revision,
                input: a,
                ..
            } => {
                prepare(&mut tx, "attempt", key, *revision).await?;
                date(&a.attempted_on)?;
                let started = chrono::DateTime::parse_from_rfc3339(&a.started_at)
                    .map_err(|_| DbError::invalid())?;
                if !a.started_at.ends_with('Z')
                    || a.started_at.len() > 40
                    || started.timestamp() < 0
                    || (date(&a.attempted_on)? - started.date_naive())
                        .num_days()
                        .abs()
                        > 1
                    || a.language.trim().is_empty()
                    || a.language.len() > 100
                    || a.notes.len() > 100000
                    || !(0..=604800).contains(&a.manual_seconds)
                    || ![
                        "accepted",
                        "wrong_answer",
                        "time_limit",
                        "runtime_error",
                        "compile_error",
                        "unfinished",
                    ]
                    .contains(&a.outcome.as_str())
                {
                    return Err(DbError::invalid());
                }
                reference(&mut tx, "coding_problems", &Some(a.problem_id.clone())).await?;
                if let Some(f) = &a.focus_session_id {
                    id(f)?;
                    let task: Option<String> = sqlx::query_scalar(
                        "SELECT task_id FROM focus_sessions WHERE id=? AND state='finished'",
                    )
                    .bind(f)
                    .fetch_optional(&mut *tx)
                    .await?
                    .ok_or_else(DbError::invalid)?;
                    let linked: Option<String> =
                        sqlx::query_scalar("SELECT task_id FROM coding_problems WHERE id=?")
                            .bind(&a.problem_id)
                            .fetch_one(&mut *tx)
                            .await?;
                    if linked.is_some() && linked != task {
                        return Err(DbError::invalid());
                    }
                    let n:i64=sqlx::query_scalar("SELECT count(*) FROM coding_attempts WHERE focus_session_id=? AND deleted_at IS NULL AND id<>?").bind(f).bind(key).fetch_one(&mut *tx).await?;
                    if n > 0 {
                        return Err(DbError::new(
                            "CODING_FOCUS_USED",
                            "这段专注已关联另一条尝试，不能重复计时。",
                        ));
                    }
                }
                if revision.is_none() {
                    sqlx::query("INSERT INTO coding_attempts(id,problem_id,created_at,updated_at) VALUES(?,?,?,?)").bind(key).bind(&a.problem_id).bind(&stamp).bind(&stamp).execute(&mut *tx).await?;
                } else {
                    let original: String =
                        sqlx::query_scalar("SELECT problem_id FROM coding_attempts WHERE id=?")
                            .bind(key)
                            .fetch_one(&mut *tx)
                            .await?;
                    if original != a.problem_id {
                        return Err(DbError::invalid());
                    }
                }
                sqlx::query("UPDATE coding_attempts SET language=?,outcome=?,notes=?,attempted_on=?,started_at=?,manual_seconds=?,focus_session_id=?,revision=revision+1,updated_at=? WHERE id=?").bind(a.language.trim()).bind(&a.outcome).bind(&a.notes).bind(&a.attempted_on).bind(&a.started_at).bind(a.manual_seconds).bind(&a.focus_session_id).bind(&stamp).bind(key).execute(&mut *tx).await?;
            }
            Action::Contest {
                id: key,
                revision,
                input: c,
                ..
            } => {
                prepare(&mut tx, "contest", key, *revision).await?;
                platform(&c.platform)?;
                date(&c.event_date)?;
                url(&c.source_url)?;
                if c.title.trim().is_empty() || c.title.len() > 1000 || c.notes.len() > 100000 {
                    return Err(DbError::invalid());
                }
                if revision.is_none() {
                    sqlx::query(
                        "INSERT INTO competitions(id,title,created_at,updated_at) VALUES(?,?,?,?)",
                    )
                    .bind(key)
                    .bind(c.title.trim())
                    .bind(&stamp)
                    .bind(&stamp)
                    .execute(&mut *tx)
                    .await?;
                }
                sqlx::query("UPDATE competitions SET title=?,platform=?,event_date=?,source_url=?,participated=?,notes=?,revision=revision+1,updated_at=? WHERE id=?").bind(c.title.trim()).bind(&c.platform).bind(&c.event_date).bind(&c.source_url).bind(c.participated).bind(&c.notes).bind(&stamp).bind(key).execute(&mut *tx).await?;
            }
            Action::Trash {
                id: key,
                kind,
                revision,
                restore,
                ..
            } => {
                id(key)?;
                let t = table(kind)?;
                let r = sqlx::query(&format!("SELECT revision,deleted_at FROM {t} WHERE id=?"))
                    .bind(key)
                    .fetch_optional(&mut *tx)
                    .await?
                    .ok_or_else(conflict)?;
                if r.get::<i64, _>("revision") != *revision
                    || r.get::<Option<String>, _>("deleted_at").is_some() != *restore
                {
                    return Err(conflict());
                }
                if *restore && kind == "problem" {
                    let p =
                        sqlx::query("SELECT platform,external_id FROM coding_problems WHERE id=?")
                            .bind(key)
                            .fetch_one(&mut *tx)
                            .await?;
                    let n:i64=sqlx::query_scalar("SELECT count(*) FROM coding_problems WHERE platform=? AND external_id=? COLLATE NOCASE AND external_id<>'' AND deleted_at IS NULL").bind(p.get::<String,_>("platform")).bind(p.get::<String,_>("external_id")).fetch_one(&mut *tx).await?;
                    if n > 0 {
                        return Err(DbError::new(
                            "CODING_DUPLICATE",
                            "相同平台题号已有活动题目，未覆盖，恢复被阻止。",
                        ));
                    }
                }
                if *restore && kind == "attempt" {
                    let r = sqlx::query(
                        "SELECT problem_id,focus_session_id FROM coding_attempts WHERE id=?",
                    )
                    .bind(key)
                    .fetch_one(&mut *tx)
                    .await?;
                    reference(&mut tx, "coding_problems", &Some(r.get("problem_id"))).await?;
                    if let Some(f) = r.get::<Option<String>, _>("focus_session_id") {
                        let n:i64=sqlx::query_scalar("SELECT count(*) FROM coding_attempts WHERE focus_session_id=? AND deleted_at IS NULL").bind(f).fetch_one(&mut *tx).await?;
                        if n > 0 {
                            return Err(DbError::new(
                                "CODING_FOCUS_USED",
                                "这段专注已关联另一条尝试，不能重复计时。",
                            ));
                        }
                    }
                }
                sqlx::query(&format!(
                    "UPDATE {t} SET deleted_at=?,updated_at=?,revision=revision+1 WHERE id=?"
                ))
                .bind(if *restore { None } else { Some(&stamp) })
                .bind(&stamp)
                .bind(key)
                .execute(&mut *tx)
                .await?;
            }
        }
        sqlx::query(
            "INSERT INTO coding_receipts(request_id,payload_hash,created_at) VALUES(?,?,?)",
        )
        .bind(action.request_id())
        .bind(hash)
        .bind(stamp)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }
}

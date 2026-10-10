use super::*;
use chrono::Datelike;
const LIVE:&str="WITH live AS (SELECT a.*,COALESCE(f.actual_seconds,0) focus_seconds,p.difficulty,p.tags_json FROM coding_attempts a JOIN coding_problems p ON p.id=a.problem_id LEFT JOIN focus_sessions f ON f.id=a.focus_session_id AND f.state='finished' WHERE a.deleted_at IS NULL AND p.deleted_at IS NULL AND a.attempted_on IS NOT NULL), ranked AS (SELECT *,ROW_NUMBER() OVER(PARTITION BY problem_id ORDER BY attempted_on,started_at,id) rn FROM live), solved AS (SELECT problem_id,MIN(attempted_on) solved_on FROM live WHERE outcome='accepted' GROUP BY problem_id)";
pub(super) async fn read(pool: &sqlx::SqlitePool, week: &str) -> Result<Analytics> {
    let day = date(week)?;
    let from = day - chrono::Duration::days(day.weekday().num_days_from_monday() as i64);
    let until = from + chrono::Duration::days(6);
    let from = from.to_string();
    let until = until.to_string();
    let r=sqlx::query(&format!("{LIVE} SELECT COUNT(DISTINCT problem_id) problems,COUNT(*) attempts,COALESCE(SUM(manual_seconds),0) manual_seconds,COALESCE(SUM(focus_seconds),0) focus_seconds FROM live WHERE attempted_on BETWEEN ? AND ?")).bind(&from).bind(&until).fetch_one(pool).await?;
    let first=sqlx::query(&format!("{LIVE} SELECT COUNT(*) denominator,COALESCE(SUM(outcome='accepted'),0) numerator FROM ranked WHERE rn=1 AND attempted_on BETWEEN ? AND ?")).bind(&from).bind(&until).fetch_one(pool).await?;
    let solved: i64 = sqlx::query_scalar(&format!(
        "{LIVE} SELECT count(*) FROM solved WHERE solved_on BETWEEN ? AND ?"
    ))
    .bind(&from)
    .bind(&until)
    .fetch_one(pool)
    .await?;
    let contests:i64=sqlx::query_scalar("SELECT count(*) FROM competitions WHERE deleted_at IS NULL AND participated=1 AND event_date BETWEEN ? AND ?").bind(&from).bind(&until).fetch_one(pool).await?;
    let difficulty=sqlx::query(&format!("{LIVE} SELECT COALESCE(difficulty,'unknown') label,COUNT(DISTINCT problem_id) n FROM live WHERE attempted_on BETWEEN ? AND ? GROUP BY difficulty ORDER BY label")).bind(&from).bind(&until).fetch_all(pool).await?.iter().map(|r|Distribution{label:r.get("label"),count:r.get("n")}).collect();
    let topics=sqlx::query(&format!("{LIVE} SELECT j.value label,COUNT(DISTINCT l.problem_id) n FROM live l,json_each(l.tags_json) j WHERE attempted_on BETWEEN ? AND ? GROUP BY j.value ORDER BY n DESC,j.value")).bind(&from).bind(&until).fetch_all(pool).await?.iter().map(|r|Distribution{label:r.get("label"),count:r.get("n")}).collect();
    Ok(Analytics {
        from,
        until,
        problems: r.get("problems"),
        attempts: r.get("attempts"),
        manual_seconds: r.get("manual_seconds"),
        focus_seconds: r.get("focus_seconds"),
        solved,
        contests,
        first_pass_denominator: first.get("denominator"),
        first_pass_numerator: first.get("numerator"),
        difficulty,
        topics,
    })
}

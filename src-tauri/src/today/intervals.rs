use super::*;
use chrono::{LocalResult, TimeZone};

pub(super) struct Part {
    pub date: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub elapsed_ms: i64,
}

// Allocate monotonic duration at local midnight; UTC bounds remain unambiguous.
pub(super) fn split<T: TimeZone>(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    elapsed: i64,
    zone: &T,
) -> Vec<Part> {
    if elapsed <= 0 || end <= start {
        return vec![];
    }
    let total = (end - start).num_milliseconds().max(1);
    let mut cursor = start;
    let mut allocated = 0;
    let mut parts = vec![];
    while cursor < end {
        let date = cursor.with_timezone(zone).date_naive();
        let tomorrow = date
            .succ_opt()
            .unwrap_or(date)
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let mut midnight = None;
        // Some zones skip midnight on DST transitions. Use the first valid instant.
        for minute in 0..=1440 {
            match zone.from_local_datetime(&(tomorrow + chrono::Duration::minutes(minute))) {
                LocalResult::Single(v) => {
                    midnight = Some(v.with_timezone(&Utc));
                    break;
                }
                LocalResult::Ambiguous(a, b) => {
                    midnight = Some(a.min(b).with_timezone(&Utc));
                    break;
                }
                LocalResult::None => {}
            }
        }
        let bound = midnight.filter(|v| *v > cursor).unwrap_or(end).min(end);
        let cumulative = if bound == end {
            elapsed
        } else {
            ((bound - start).num_milliseconds() as i128 * elapsed as i128 / total as i128) as i64
        };
        parts.push(Part {
            date: date.to_string(),
            start: cursor,
            end: bound,
            elapsed_ms: cumulative - allocated,
        });
        allocated = cumulative;
        cursor = bound;
    }
    parts
}

pub(super) async fn record(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    segment: i64,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    elapsed: i64,
) -> Result<()> {
    for part in split(start, end, elapsed, &Local) {
        sqlx::query("INSERT INTO focus_intervals(session_id,segment_no,local_date,started_at,ended_at,elapsed_ms) VALUES(?,?,?,?,?,?) ON CONFLICT(session_id,segment_no,local_date) DO UPDATE SET ended_at=excluded.ended_at,elapsed_ms=focus_intervals.elapsed_ms+excluded.elapsed_ms")
            .bind(id).bind(segment).bind(part.date).bind(part.start.to_rfc3339_opts(chrono::SecondsFormat::Millis,true)).bind(part.end.to_rfc3339_opts(chrono::SecondsFormat::Millis,true)).bind(part.elapsed_ms).execute(&mut **tx).await?;
    }
    Ok(())
}

pub(crate) async fn rebuild_metrics(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    timestamp: &str,
) -> Result<()> {
    let dates:Vec<String>=sqlx::query_scalar("SELECT local_date FROM focus_intervals WHERE session_id=? UNION SELECT session_date FROM focus_sessions WHERE id=? AND session_date IS NOT NULL")
        .bind(id).bind(id).fetch_all(&mut **tx).await?;
    for date in dates {
        sqlx::query("INSERT INTO daily_metrics(id,metric_date,metric_key,value,created_at,updated_at) VALUES(?,?,'focus_seconds',(SELECT CAST(COALESCE(SUM(i.elapsed_ms),0)/1000 AS INTEGER) FROM focus_intervals i JOIN focus_sessions f ON f.id=i.session_id WHERE i.local_date=? AND f.state='finished' AND f.deleted_at IS NULL),?,?) ON CONFLICT(metric_date,metric_key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at,deleted_at=NULL")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&date).bind(&date).bind(timestamp).bind(timestamp).execute(&mut **tx).await?;
    }
    Ok(())
}

use super::*;
use sqlx::{Execute, QueryBuilder};
impl Database {
    pub async fn journal_get(&self, entry_id: &str) -> Result<Entry> {
        id(entry_id)?;
        let row = sqlx::query("SELECT * FROM journal_entries WHERE id=?")
            .bind(entry_id)
            .fetch_one(&self.pool)
            .await?;
        let tags = sqlx::query_scalar(
            "SELECT name FROM journal_tags WHERE entry_id=? AND deleted_at IS NULL ORDER BY name",
        )
        .bind(entry_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(Entry {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
            date: row.get("entry_date"),
            entry_type: row.get("entry_type"),
            is_draft: row.get::<i64, _>("is_draft") != 0,
            revision: row.get("revision"),
            journey_id: row.get("journey_id"),
            task_id: row.get("task_id"),
            music_id: row.get("music_id"),
            tags,
            deleted_at: row.get("deleted_at"),
            updated_at: row.get("updated_at"),
        })
    }
    pub async fn journal_page(&self, filter: Filter) -> Result<Page> {
        if filter.query.len() > 500
            || filter.tag.len() > 100
            || !(0..=1000000).contains(&filter.page)
            || !["", "published", "drafts", "trash"].contains(&filter.mode.as_str())
        {
            return Err(DbError::invalid());
        }
        for d in [&filter.from, &filter.until] {
            if !d.is_empty() {
                date(d)?;
            }
        }
        if !filter.from.is_empty() && !filter.until.is_empty() && filter.from > filter.until {
            return Err(DbError::invalid());
        }
        let mut q = QueryBuilder::<sqlx::Sqlite>::new("SELECT id FROM journal_entries e WHERE ");
        q.push(if filter.mode == "trash" {
            "e.deleted_at IS NOT NULL"
        } else {
            "e.deleted_at IS NULL"
        });
        if filter.mode != "trash" {
            q.push(" AND e.is_draft=")
                .push_bind(i64::from(filter.mode == "drafts"));
        }
        if !filter.query.trim().is_empty() {
            if filter.query.chars().count() >= 3
                && filter.mode != "trash"
                && filter.mode != "drafts"
            {
                q.push(
                    " AND e.id IN (SELECT entry_id FROM journal_search WHERE journal_search MATCH ",
                )
                .push_bind(format!("\"{}\"", filter.query.replace('"', "\"\"")))
                .push(")");
            } else {
                let like = format!(
                    "%{}%",
                    filter
                        .query
                        .replace('\\', "\\\\")
                        .replace('%', "\\%")
                        .replace('_', "\\_")
                );
                q.push(" AND (e.title LIKE ")
                    .push_bind(like.clone())
                    .push(" ESCAPE '\\' OR e.body LIKE ")
                    .push_bind(like)
                    .push(" ESCAPE '\\')");
            }
        }
        for (column, value) in [
            ("e.entry_date>=", &filter.from),
            ("e.entry_date<=", &filter.until),
            ("e.entry_type=", &filter.entry_type),
        ] {
            if !value.is_empty() {
                q.push(" AND ").push(column).push_bind(value.clone());
            }
        }
        if !filter.tag.is_empty() {
            q.push(" AND EXISTS(SELECT 1 FROM journal_tags t WHERE t.entry_id=e.id AND t.deleted_at IS NULL AND t.name=").push_bind(filter.tag).push(")");
        }
        let sql = q.sql().to_owned();
        let count_sql = sql.replacen("SELECT id", "SELECT COUNT(*)", 1);
        let mut arguments = q
            .build()
            .take_arguments()
            .map_err(|_| DbError::invalid())?
            .ok_or_else(DbError::invalid)?;
        let total: i64 = sqlx::query_scalar_with(&count_sql, arguments.clone())
            .fetch_one(&self.pool)
            .await?;
        sqlx::Arguments::add(&mut arguments, filter.page * 30).map_err(|_| DbError::invalid())?;
        let page_sql = format!(
            "{sql} ORDER BY e.entry_date DESC,e.created_at DESC,e.id DESC LIMIT 30 OFFSET ?"
        );
        let ids: Vec<String> = sqlx::query_scalar_with(&page_sql, arguments)
            .fetch_all(&self.pool)
            .await?;
        let mut entries = Vec::new();
        for key in ids {
            entries.push(self.journal_get(&key).await?);
        }
        let raw = sqlx::query(
            "SELECT payload,updated_at FROM journal_recovery ORDER BY updated_at DESC LIMIT 50",
        )
        .fetch_all(&self.pool)
        .await?;
        let mut recoveries = Vec::new();
        for r in raw {
            recoveries.push(Recovery {
                payload: serde_json::from_str(&r.get::<String, _>("payload"))
                    .map_err(|_| DbError::invalid())?,
                updated_at: r.get("updated_at"),
            });
        }
        async fn choices(db: &Database, table: &str) -> Result<Vec<Choice>> {
            let sql=format!("SELECT id,title FROM {table} WHERE deleted_at IS NULL ORDER BY updated_at DESC LIMIT 500");
            Ok(sqlx::query(&sql)
                .fetch_all(&db.pool)
                .await?
                .into_iter()
                .map(|r| Choice {
                    id: r.get("id"),
                    title: r.get("title"),
                })
                .collect())
        }
        Ok(Page {
            entries,
            total,
            page: filter.page,
            recoveries,
            journeys: choices(self, "journeys").await?,
            tasks: choices(self, "tasks").await?,
            music: choices(self, "music_tracks").await?,
        })
    }
}

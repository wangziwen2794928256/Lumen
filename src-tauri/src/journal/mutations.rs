use super::*;
use crate::database::now;
impl Database {
    pub async fn journal_save(&self, input: Save) -> Result<Entry> {
        id(&input.id)?;
        id(&input.client_id)?;
        date(&input.date)?;
        if input.title.len() > 1000
            || input.body.len() > 1_000_000
            || input.sequence < 1
            || input.revision < 0
            || input.tags.len() > 30
            || input
                .tags
                .iter()
                .any(|t| t.trim().is_empty() || t.len() > 100)
            || ![
                "reflection",
                "quick_note",
                "study_note",
                "memory",
                "mood",
                "music_note",
                "competition_note",
                "research_note",
            ]
            .contains(&input.entry_type.as_str())
            || (input.publish && input.title.trim().is_empty() && input.body.trim().is_empty())
        {
            return Err(DbError::invalid());
        }
        for key in [&input.journey_id, &input.task_id, &input.music_id]
            .into_iter()
            .flatten()
        {
            id(key)?;
        }
        let payload = serde_json::to_string(&input).map_err(|_| DbError::invalid())?;
        let prior: Option<String> = sqlx::query_scalar(
            "SELECT payload_hash FROM journal_save_receipts WHERE client_id=? AND sequence=?",
        )
        .bind(&input.client_id)
        .bind(input.sequence)
        .fetch_optional(&self.pool)
        .await?;
        if let Some(previous) = prior {
            if previous != crate::database::checksum(payload.as_bytes()) {
                return Err(conflict());
            }
            return self.journal_get(&input.id).await;
        }
        // Independent durable transaction: conflicts must not roll back the user's recovery copy.
        let recovery=sqlx::query("INSERT INTO journal_recovery(client_id,entry_id,sequence,payload,updated_at) VALUES(?,?,?,?,?) ON CONFLICT(client_id) DO UPDATE SET entry_id=excluded.entry_id,sequence=excluded.sequence,payload=excluded.payload,updated_at=excluded.updated_at WHERE excluded.sequence>journal_recovery.sequence").bind(&input.client_id).bind(&input.id).bind(input.sequence).bind(&payload).bind(now()).execute(&self.pool).await?;
        if recovery.rows_affected() == 0 {
            let saved: Option<(String, i64)> =
                sqlx::query_as("SELECT payload,sequence FROM journal_recovery WHERE client_id=?")
                    .bind(&input.client_id)
                    .fetch_optional(&self.pool)
                    .await?;
            if saved
                .as_ref()
                .is_some_and(|(p, s)| p != &payload || *s != input.sequence)
            {
                return Err(conflict());
            }
        }
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;
        let receipt: Option<String> = sqlx::query_scalar(
            "SELECT payload_hash FROM journal_save_receipts WHERE client_id=? AND sequence=?",
        )
        .bind(&input.client_id)
        .bind(input.sequence)
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(previous) = receipt {
            if previous != crate::database::checksum(payload.as_bytes()) {
                return Err(conflict());
            }
            tx.commit().await?;
            return self.journal_get(&input.id).await;
        }
        let old: Option<(i64, Option<String>, i64)> =
            sqlx::query_as("SELECT revision,deleted_at,is_draft FROM journal_entries WHERE id=?")
                .bind(&input.id)
                .fetch_optional(&mut *tx)
                .await?;
        if old
            .as_ref()
            .is_some_and(|(r, d, _)| *r != input.revision || d.is_some())
            || (old.is_none() && input.revision != 0)
        {
            return Err(conflict());
        }
        for (table, key) in [
            ("journeys", &input.journey_id),
            ("tasks", &input.task_id),
            ("music_tracks", &input.music_id),
        ] {
            if let Some(key) = key {
                let sql = format!("SELECT count(*) FROM {table} WHERE id=? AND deleted_at IS NULL");
                let n: i64 = sqlx::query_scalar(&sql)
                    .bind(key)
                    .fetch_one(&mut *tx)
                    .await?;
                if n != 1 {
                    return Err(DbError::invalid());
                }
            }
        }
        let timestamp = now();
        let title = if input.title.trim().is_empty() {
            input
                .body
                .lines()
                .find(|l| !l.trim().is_empty())
                .unwrap_or("未命名草稿")
                .chars()
                .take(60)
                .collect::<String>()
        } else {
            input.title.trim().to_owned()
        };
        let draft = !(input.publish || old.as_ref().is_some_and(|(_, _, d)| *d == 0));
        sqlx::query("INSERT INTO journal_entries(id,title,body,entry_date,entry_type,is_draft,revision,journey_id,task_id,music_id,created_at,updated_at) VALUES(?,?,?,?,?,?,1,?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET title=excluded.title,body=excluded.body,entry_date=excluded.entry_date,entry_type=excluded.entry_type,is_draft=excluded.is_draft,revision=journal_entries.revision+1,journey_id=excluded.journey_id,task_id=excluded.task_id,music_id=excluded.music_id,updated_at=excluded.updated_at").bind(&input.id).bind(title).bind(&input.body).bind(&input.date).bind(&input.entry_type).bind(i64::from(draft)).bind(&input.journey_id).bind(&input.task_id).bind(&input.music_id).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
        sqlx::query("UPDATE journal_tags SET deleted_at=?,updated_at=? WHERE entry_id=? AND deleted_at IS NULL").bind(&timestamp).bind(&timestamp).bind(&input.id).execute(&mut *tx).await?;
        let mut tags = input.tags.clone();
        tags.sort();
        tags.dedup();
        for tag in tags {
            sqlx::query("INSERT INTO journal_tags(id,entry_id,name,created_at,updated_at) VALUES(?,?,?,?,?) ON CONFLICT(entry_id,name) DO UPDATE SET deleted_at=NULL,updated_at=excluded.updated_at").bind(uuid::Uuid::new_v4().to_string()).bind(&input.id).bind(tag.trim()).bind(&timestamp).bind(&timestamp).execute(&mut *tx).await?;
        }
        sqlx::query(
            "INSERT INTO journal_save_receipts(client_id,sequence,payload_hash) VALUES(?,?,?)",
        )
        .bind(&input.client_id)
        .bind(input.sequence)
        .bind(crate::database::checksum(payload.as_bytes()))
        .execute(&mut *tx)
        .await?;
        sqlx::query("DELETE FROM journal_recovery WHERE client_id=? AND sequence=?")
            .bind(&input.client_id)
            .bind(input.sequence)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        self.journal_get(&input.id).await
    }
    pub async fn journal_trash(
        &self,
        entry_id: &str,
        revision: i64,
        restore: bool,
    ) -> Result<Entry> {
        id(entry_id)?;
        let result=sqlx::query("UPDATE journal_entries SET deleted_at=?,revision=revision+1,updated_at=? WHERE id=? AND revision=? AND (deleted_at IS NULL)=?").bind(if restore{None}else{Some(now())}).bind(now()).bind(entry_id).bind(revision).bind(i64::from(!restore)).execute(&self.pool).await?;
        if result.rows_affected() != 1 {
            return Err(conflict());
        }
        self.journal_get(entry_id).await
    }
}

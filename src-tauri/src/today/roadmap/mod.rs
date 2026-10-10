mod decisions;
pub mod model;
pub(crate) mod mutations;
pub mod planner;
mod store;
mod template;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_links;
use super::*;
pub use model::{Action, Snapshot};
impl TodayService {
    pub async fn roadmap_snapshot(&self, week: &str) -> Result<Snapshot> {
        let mut clock = self.clock.lock().await;
        self.tick(&mut clock).await?;
        store::snapshot(&self.db.pool, week).await
    }
    pub async fn roadmap_action(&self, week: &str, action: Action) -> Result<Snapshot> {
        store::week(week)?;
        let mut clock = self.clock.lock().await;
        self.tick(&mut clock).await?;
        let mut tx = self.db.pool.begin_with("BEGIN IMMEDIATE").await?;
        let timestamp = now();
        match &action {
            Action::Propose { .. } | Action::Decide { .. } => {
                decisions::write(&mut tx, week, &action, &timestamp).await?
            }
            Action::Template { id } => {
                validate_id(id)?;
                if super::receipt(&mut tx, id, &action).await? {
                    template::create(&mut tx, id, &timestamp).await?
                }
            }
            _ => mutations::write(&mut tx, &action, &timestamp).await?,
        }
        tx.commit().await?;
        clock.instant = Instant::now();
        clock.wall = Utc::now();
        store::snapshot(&self.db.pool, week).await
    }
}

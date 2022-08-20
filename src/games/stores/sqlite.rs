use crate::games::stores::{GamesStore, GamesStoreError};
use crate::games::{Game, GameId};
use crate::platforms::PlatformId;
use async_trait::async_trait;

pub struct GamesStoreSqlite {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[async_trait]
impl GamesStore for GamesStoreSqlite {
    async fn get_all_by_platform(
        &self,
        platform_id: PlatformId,
        first: Option<usize>,
        after: Option<String>,
    ) -> Result<Vec<Game>, GamesStoreError> {
        let first = first.unwrap_or(10) as u32;
        let after = after.unwrap_or_default();

        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: GameId", name as "name!", platform_id as "platform_id!: PlatformId"
            FROM games
            WHERE platform_id = ?
            AND name > ?
            ORDER BY name ASC
            LIMIT ?
            "#,
            platform_id.0,
            after,
            first
        )
        .fetch_all(&self.pool)
        .await
        .map_err(GamesStoreError::Sqlx)
    }
}

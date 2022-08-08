use crate::games::stores::{GamesStore, GamesStoreError};
use crate::games::Game;
use async_trait::async_trait;

pub struct SqliteGamesStore {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[async_trait]
impl GamesStore for SqliteGamesStore {
    async fn get_all(&self) -> Result<Vec<Game>, GamesStoreError> {
        sqlx::query_as!(
            Game,
            r#"
            SELECT id as "id!: _", name as "name!", platform_id as "platform_id!: _"
            FROM games
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(GamesStoreError::Sqlx)
    }
}

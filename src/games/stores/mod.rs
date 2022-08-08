pub mod sqlite;

use crate::games::Game;
use async_trait::async_trait;

#[async_trait]
trait GamesStore {
    async fn get_all(&self) -> Result<Vec<Game>, GamesStoreError>;
}

pub enum GamesStoreError {
    Sqlx(sqlx::Error),
}

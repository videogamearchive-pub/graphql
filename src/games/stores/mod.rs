pub mod sqlite;

use crate::games::Game;
use async_trait::async_trait;

#[async_trait]
pub trait GamesStore {
    async fn get_all(&self) -> Result<Vec<Game>, GamesStoreError>;
}

#[derive(Debug)]
pub enum GamesStoreError {
    Sqlx(sqlx::Error),
}

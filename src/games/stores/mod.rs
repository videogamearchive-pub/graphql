pub mod sqlite;

use crate::games::Game;
use crate::platforms::PlatformId;
use async_trait::async_trait;

#[async_trait]
pub trait GamesStore {
    async fn get_all_by_platform(
        &self,
        platform_id: PlatformId,
        first: Option<usize>,
        after: Option<String>,
    ) -> Result<Vec<Game>, GamesStoreError>;
}

#[derive(Debug)]
pub enum GamesStoreError {
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for GamesStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Sqlx(e) => write!(f, "{}", e),
        }
    }
}

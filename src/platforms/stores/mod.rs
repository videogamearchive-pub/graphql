pub mod sqlite;

use crate::platforms::Platform;
use async_trait::async_trait;

#[async_trait]
pub trait PlatformsStore {
    async fn get_all(
        &self,
        first: Option<usize>,
        after: Option<String>,
    ) -> Result<Vec<Platform>, PlatformsStoreError>;
}

#[derive(Debug)]
pub enum PlatformsStoreError {
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for PlatformsStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Sqlx(e) => write!(f, "{}", e),
        }
    }
}

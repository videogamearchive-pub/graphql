pub mod sqlite;

use crate::platforms::Platform;
use async_trait::async_trait;

#[async_trait]
pub trait PlatformsStore {
    async fn get_all(&self) -> Result<Vec<Platform>, PlatformsStoreError>;
}

#[derive(Debug)]
pub enum PlatformsStoreError {
    Sqlx(sqlx::Error),
}

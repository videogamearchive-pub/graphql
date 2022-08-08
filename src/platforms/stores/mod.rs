pub mod sqlite;

use crate::platforms::Platform;
use async_trait::async_trait;

#[async_trait]
trait PlatformsStore {
    async fn get_all(&self) -> Result<Vec<Platform>, PlatformsStoreError>;
}

pub enum PlatformsStoreError {
    Sqlx(sqlx::Error),
}

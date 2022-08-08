use crate::platforms::stores::{PlatformsStore, PlatformsStoreError};
use crate::platforms::Platform;
use async_trait::async_trait;

pub struct SqlitePlatformsStore {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[async_trait]
impl PlatformsStore for SqlitePlatformsStore {
    async fn get_all(&self) -> Result<Vec<Platform>, PlatformsStoreError> {
        sqlx::query_as!(
            Platform,
            r#"
            SELECT id as "id!: _", name as "name!"
            FROM platforms
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(PlatformsStoreError::Sqlx)
    }
}

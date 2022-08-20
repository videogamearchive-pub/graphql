use crate::platforms::stores::{PlatformsStore, PlatformsStoreError};
use crate::platforms::{Platform, PlatformId};
use async_trait::async_trait;

pub struct PlatformsStoreSqlite {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[async_trait]
impl PlatformsStore for PlatformsStoreSqlite {
    async fn get_all(
        &self,
        first: Option<usize>,
        after: Option<String>,
    ) -> Result<Vec<Platform>, PlatformsStoreError> {
        let first = first.unwrap_or(10) as u32;
        let after = after.unwrap_or_default();

        sqlx::query_as!(
            Platform,
            r#"
            SELECT id as "id!: PlatformId", name as "name!"
            FROM platforms
            WHERE name > ?
            ORDER BY name ASC
            LIMIT ?
            "#,
            after,
            first
        )
        .fetch_all(&self.pool)
        .await
        .map_err(PlatformsStoreError::Sqlx)
    }
}

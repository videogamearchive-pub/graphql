pub mod graphql;
pub mod stores;

use async_graphql::{scalar, SimpleObject};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, sqlx::Decode)]
pub struct PlatformId(pub u32);
scalar!(PlatformId);

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Platform {
    pub id: PlatformId,
    pub name: String,
}

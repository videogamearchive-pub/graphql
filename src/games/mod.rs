pub mod stores;

use crate::platforms::PlatformId;
use async_graphql::{scalar, SimpleObject};

#[derive(serde::Serialize, serde::Deserialize, sqlx::Decode)]
pub struct GameId(pub u32);
scalar!(GameId);

#[derive(SimpleObject)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub platform_id: PlatformId,
}

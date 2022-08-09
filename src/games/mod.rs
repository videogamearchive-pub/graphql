pub mod stores;

use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Game {
    id: u32,
    name: String,
    platform_id: u32,
}

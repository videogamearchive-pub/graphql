pub mod graphql;
pub mod stores;

use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Platform {
    pub id: u32,
    pub name: String,
}

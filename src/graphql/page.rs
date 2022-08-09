use crate::games::Game;
use crate::platforms::Platform;
use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
#[graphql(concrete(name = "PagePlatform", params(Platform)))]
#[graphql(concrete(name = "PageGame", params(Game)))]
pub struct Page<A: OutputType> {
    pub nodes: Vec<A>,
}

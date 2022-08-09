use crate::games::stores::sqlite::GamesStoreSqlite;
use crate::games::stores::GamesStore;
use crate::games::Game;
use crate::graphql::page::Page;
use crate::platforms::stores::sqlite::PlatformsStoreSqlite;
use crate::platforms::stores::PlatformsStore;
use crate::platforms::Platform;
use async_graphql::{ComplexObject, Context, Object};

pub struct Query;

#[Object]
impl Query {
    async fn platforms(&self, ctx: &Context<'_>) -> Page<Platform> {
        ctx.data_unchecked::<PlatformsStoreSqlite>()
            .get_all()
            .await
            .map_or_else(
                |err| {
                    println!("Failed: {:?}", err);
                    Page { nodes: vec![] }
                },
                |nodes| Page { nodes },
            )
    }
}

#[ComplexObject]
impl Platform {
    async fn games(&self, ctx: &Context<'_>) -> Page<Game> {
        ctx.data_unchecked::<GamesStoreSqlite>()
            .get_all()
            .await
            .map_or_else(
                |err| {
                    println!("Failed: {:?}", err);
                    Page { nodes: vec![] }
                },
                |nodes| Page { nodes },
            )
    }
}

use crate::games::stores::sqlite::GamesStoreSqlite;
use crate::games::stores::GamesStore;
use crate::games::Game;
use crate::graphql::Base64Cursor;
use crate::platforms::stores::sqlite::PlatformsStoreSqlite;
use crate::platforms::stores::PlatformsStore;
use crate::platforms::Platform;
use async_graphql::connection::{query, Connection, Edge};
use async_graphql::{ComplexObject, Context, Error, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn platforms(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        #[graphql(validator(maximum = 25))] first: Option<i32>,
    ) -> Result<Connection<Base64Cursor<String>, Platform>> {
        query(
            after,
            None,
            first,
            None,
            |after: Option<Base64Cursor<String>>, _, first, _| async move {
                let platforms = ctx
                    .data_unchecked::<PlatformsStoreSqlite>()
                    .get_all(first, after.map(|c| c.0))
                    .await?;

                let has_next_page = !platforms.is_empty();

                let mut connection = Connection::new(false, has_next_page);
                connection.edges = platforms
                    .into_iter()
                    .map(|platform| Edge::new(Base64Cursor(platform.name.clone()), platform))
                    .collect();

                Ok::<_, Error>(connection)
            },
        )
        .await
    }
}

#[ComplexObject]
impl Platform {
    async fn games(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        #[graphql(validator(maximum = 25))] first: Option<i32>,
    ) -> Result<Connection<Base64Cursor<String>, Game>> {
        query(
            after,
            None,
            first,
            None,
            |after: Option<Base64Cursor<String>>, _, first, _| async move {
                let games = ctx
                    .data_unchecked::<GamesStoreSqlite>()
                    .get_all_by_platform(self.id, first, after.map(|c| c.0))
                    .await?;

                let has_next_page = !games.is_empty();

                let mut connection = Connection::new(false, has_next_page);
                connection.edges = games
                    .into_iter()
                    .map(|platform| Edge::new(Base64Cursor(platform.name.clone()), platform))
                    .collect();

                Ok::<_, Error>(connection)
            },
        )
        .await
    }
}

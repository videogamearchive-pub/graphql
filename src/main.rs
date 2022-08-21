pub mod environment;
pub mod games;
pub mod graphql;
pub mod platforms;
pub mod utils;

use crate::games::stores::sqlite::GamesStoreSqlite;
use crate::platforms::graphql::Query;
use crate::platforms::stores::sqlite::PlatformsStoreSqlite;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use sqlx::sqlite::SqlitePool;
use tide::{http::mime, Body, Response, StatusCode};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = SqlitePool::connect(&environment::DATABASE_URL).await?;

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(PlatformsStoreSqlite { pool: pool.clone() })
        .data(GamesStoreSqlite { pool: pool.clone() })
        .finish();

    let mut app = tide::new();

    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);

        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        )));

        resp.set_content_type(mime::HTML);

        Ok(resp)
    });

    app.at("/graphql").post(async_graphql_tide::graphql(schema));

    println!("Listening on http://{}", *environment::HTTP_ADDRESS);
    app.listen(environment::HTTP_ADDRESS.clone()).await?;

    Ok(())
}

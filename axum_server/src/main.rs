use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::Router;
use sqlx::postgres::PgPoolOptions;

mod error;

#[path = "database/db.rs"]
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "postgres://postgres:postgres@localhost:5432/izb";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let routes_all = Router::new().merge(db::db_routes(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}


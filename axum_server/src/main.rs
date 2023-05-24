use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::Router;
use local_ip_address::local_ip;
use sqlx::postgres::PgPoolOptions;

mod error;

#[path = "database/db.rs"]
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    let url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let routes_all = Router::new().merge(db::db_routes(pool));

    //let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let addr = format!("{}:{}", local_ip().unwrap(), 8080);
    let addr: SocketAddr = addr.parse().expect("Wrong address");

    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

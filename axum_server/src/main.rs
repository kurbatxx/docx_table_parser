use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::{http::Method, Router};
use local_ip_address::local_ip;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

mod error;

#[path = "database/db.rs"]
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    let url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = dotenvy::var("PORT").expect("PORT must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes_all = Router::new().merge(db::db_routes(pool).layer(cors));

    //let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let addr = format!("{}:{}", local_ip().unwrap(), port);
    let addr: SocketAddr = addr.parse().expect("Wrong address");

    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

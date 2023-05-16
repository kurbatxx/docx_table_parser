use std::net::SocketAddr;

pub use self::error::{Error, Result};
use axum::{debug_handler, extract::State, routing::get, Json, Router};

use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "postgres://postgres:postgres@localhost:5432/izb";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let routes_all = Router::new().merge(db_routes(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn db_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/root", get(root_nodes))
        //.route("/hello2/:name", get(handler_hello2))
        .with_state(pool)
}

#[derive(FromRow, Serialize)]
struct Node {
    node_id: i32,
    parrent_id: i32,
    node_name: String,
}

#[debug_handler]
async fn root_nodes(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<Node>>> {
    let q = "SELECT node_id, parrent_id, node_name FROM node";
    let query = sqlx::query_as::<_, Node>(q);

    let nodes = query.fetch_all(&pool).await?;
    Ok(Json(nodes))
}

// async fn handler_hello2(
//     State(pool): State<Pool<Postgres>>,
//     Path(name): Path<String>,
// ) -> Result<()> {
//     println!("-> {:<12} - handler_hello - {name:?}", "HANDLER");

//     Html(format!("Hello <strong>{name}</strong>"));
//     Ok(())
// }

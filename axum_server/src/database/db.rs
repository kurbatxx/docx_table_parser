use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};

use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};

pub use self::error::{Error, Result};
#[path = "../error.rs"]
mod error;

pub fn db_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/nodes", get(nodes))
        .route("/node/:p_id", get(node))
        .with_state(pool)
}

#[derive(FromRow, Serialize)]
struct Node {
    node_id: i32,
    parrent_id: i32,
    node_name: String,
}

async fn nodes(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<Node>>> {
    let q = r#"
    SELECT node_id, parrent_id, node_name 
    FROM node
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let nodes = query.fetch_all(&pool).await?;
    Ok(Json(nodes))
}

async fn node(
    State(pool): State<Pool<Postgres>>,
    Path(p_id): Path<i32>,
) -> Result<Json<Vec<Node>>> {
    dbg!(p_id);
    let q = r#"
    SELECT node_id, 
        parrent_id, 
        node_name 
    FROM node 
    WHERE parrent_id = $1
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let nodes = query.bind(p_id).fetch_all(&pool).await?;
    Ok(Json(nodes))
}

async fn create_node(
    State(pool): State<Pool<Postgres>>,
    Query(p_id): Query<i32>,
) -> Result<Json<Vec<Node>>> {
    dbg!(p_id);
    let q = r#"
    SELECT node_id, 
        parrent_id, 
        node_name 
    FROM node 
    WHERE parrent_id = $1
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let nodes = query.bind(p_id).fetch_all(&pool).await?;
    Ok(Json(nodes))
}

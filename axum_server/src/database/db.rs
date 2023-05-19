use axum::{
    extract::{self, Path, State},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

pub use self::error::{Error, Result};
#[path = "../error.rs"]
mod error;

pub fn db_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/nodes", get(nodes))
        .route("/node/:p_id", get(node))
        .route("/node_with_nest/:p_id", get(node_with_nest))
        .route("/create_node", post(create_node))
        .with_state(pool)
}

#[derive(Debug, FromRow, Serialize)]
struct Node {
    node_id: i32,
    parrent_id: i32,
    node_name: String,
    nested: bool,
}

#[derive(Debug, FromRow, Serialize)]
struct SimpleNode {
    node_id: i32,
    parrent_id: i32,
    node_name: String,
}

async fn nodes(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<SimpleNode>>> {
    let q = r#"
    SELECT node_id, parrent_id, node_name 
    FROM node
    "#;

    let query = sqlx::query_as::<_, SimpleNode>(q);

    let nodes = query.fetch_all(&pool).await?;
    Ok(Json(nodes))
}

async fn node(
    State(pool): State<Pool<Postgres>>,
    Path(p_id): Path<i32>,
) -> Result<Json<Vec<SimpleNode>>> {
    dbg!(p_id);
    let q = r#"
    SELECT node_id, 
        parrent_id, 
        node_name 
    FROM node 
    WHERE parrent_id = $1
    "#;

    let query = sqlx::query_as::<_, SimpleNode>(q);

    let nodes = query.bind(p_id).fetch_all(&pool).await?;
    Ok(Json(nodes))
}

async fn node_with_nest(
    State(pool): State<Pool<Postgres>>,
    Path(p_id): Path<i32>,
) -> Result<Json<Vec<Node>>> {
    dbg!(p_id);
    let q = r#"
    SELECT n1.node_id,
    n1.parrent_id,
    n1.node_name,
    CASE
        WHEN n.parrent_id > 0 THEN true
        ELSE false
    END as nested
    FROM node as n
    RIGHT JOIN (
        SELECT node_id,
            parrent_id,
            node_name
        FROM node
        WHERE parrent_id = $1
    ) as n1 ON n1.node_id = n.parrent_id
    GROUP BY n1.node_id,
    n1.node_name,
    n1.parrent_id,
    nested
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let nodes = query.bind(p_id).fetch_all(&pool).await?;
    Ok(Json(nodes))
}

#[derive(Debug, Deserialize)]
struct CreateNode {
    parrent_id: i32,
    node_name: String,
}
async fn create_node(
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<CreateNode>,
) -> Result<Json<Node>> {
    dbg!(&payload);
    let q = r#"
    INSERT INTO node (parrent_id, node_name)
    VALUES ($1, $2) 
    returning node_id, parrent_id, node_name
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let node = query
        .bind(&payload.parrent_id)
        .bind(&payload.node_name)
        .fetch_one(&pool)
        .await?;
    Ok(Json(node))
}

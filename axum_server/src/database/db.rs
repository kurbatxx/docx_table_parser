use axum::{
    extract::{self, Path, State},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

use uuid::Uuid;

pub use self::error::{Error, Result};
#[path = "../error.rs"]
mod error;

pub fn db_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/nodes", get(nodes))
        .route("/node/:p_id", get(node))
        .route("/node_with_nest/:p_id", get(node_with_nest))
        .route("/create_node", post(create_node))
        .route("/create_street", post(create_street))
        .route("/get_streets/:uuid", get(get_streets))
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

#[derive(Debug, Deserialize)]
struct CreateStreet {
    node_id: i32,
    street_name: String,
}

#[derive(Debug, FromRow, Serialize)]
struct Street {
    id: i32,
    street_uuid: String,
    street_name: String,
}

async fn create_street(
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<CreateStreet>,
) -> Result<Json<Street>> {
    let mut tnx = pool.begin().await?;

    let streets_uuid_q = r#"
    UPDATE node
    SET streets = COALESCE(streets, uuid_generate_v4())
    WHERE node_id = $1
    returning streets
    "#;

    let create_street_q = r#"
    INSERT INTO street (street_uuid, street_name)
    VALUES ($1, $2)
    retirning id, street_uuid, street_name
    "#;

    let streets_uuid: (String,) = sqlx::query_as(streets_uuid_q)
        .bind(&payload.node_id)
        .fetch_one(&mut tnx)
        .await?;

    let street = sqlx::query_as::<_, Street>(create_street_q)
        .bind(&streets_uuid.0)
        .bind(&payload.street_name)
        .fetch_one(&mut tnx)
        .await?;

    tnx.commit().await?;

    Ok(Json(street))
}

async fn get_streets(
    State(pool): State<Pool<Postgres>>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Vec<Street>>> {
    let streets_q = r#"
    SELECT street_id, street_uuid, street_name
    FROM street
    WHERE street_uuid = $1
    "#;

    let streets = sqlx::query_as::<_, Street>(streets_q)
        .bind(uuid)
        .fetch_all(&pool)
        .await?;

    Ok(Json(streets))
}

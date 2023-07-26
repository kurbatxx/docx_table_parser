use axum::{
    extract::{self, Path, State},
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres, Row};

use strum_macros::EnumString;
use uuid::Uuid;

pub use self::error::{Error, Result};
#[path = "../error.rs"]
mod error;

pub fn db_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/nodes", get(nodes))
        .route("/node/:p_id", get(node))
        .route("/get_nodes/:parrent_id", get(get_nodes))
        .route("/create_node", post(create_node))
        .route("/drop_node/:node_id", post(drop_node))
        .route("/update_name", post(update_node_name))
        .route("/create_street", post(create_street))
        .route("/get_streets/:uuid", get(get_streets))
        .route("/create_building", post(create_building))
        .route("/get_buildings/:street_id", get(get_buildings))
        .with_state(pool)
}

#[derive(Debug, FromRow, Serialize)]
struct Node {
    node_id: i32,
    node_type: NodeType,
    parrent_id: i32,
    node_name: String,
    #[sqlx(default)]
    has_nest: Option<bool>,
    #[sqlx(default)]
    deputat_uuid: Option<Uuid>,
}

#[derive(Debug, FromRow, Serialize)]
struct SimpleNode {
    node_id: i32,
    parrent_id: i32,
    node_name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct RenameNode {
    node_id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, EnumString, sqlx::Type)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
enum NodeType {
    #[sqlx(rename = "address")]
    Address,
    #[sqlx(rename = "street")]
    Street,
    #[sqlx(rename = "building")]
    Building,
}

#[derive(Serialize, FromRow)]
struct Id {
    id: i32,
}

async fn update_node_name(
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<RenameNode>,
) -> Result<Json<Node>> {
    let q = r#"
            UPDATE node
            SET node_name = $1
            WHERE node_id = $2
            RETURNING *;
            "#;

    let query = sqlx::query_as::<_, Node>(q);
    let node = query
        .bind(payload.name)
        .bind(payload.node_id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(node))
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

async fn get_nodes(
    State(pool): State<Pool<Postgres>>,
    Path(parrent_id): Path<i32>,
) -> Result<Json<Vec<Node>>> {
    let q = r#"
    WITH root AS (
	SELECT node_id,
		node_type,
		parrent_id,
		node_name,
		deputat_uuid
	FROM node
	WHERE parrent_id = $1
    )
    SELECT root.node_id,
        root.node_type,
        root.parrent_id,
        root.node_name,
        CASE
            WHEN COUNT(node.node_id) > 0 THEN TRUE
            ELSE FALSE
        END AS has_nest,
        root.deputat_uuid
    FROM root
        LEFT JOIN node ON node.parrent_id = root.node_id
    GROUP BY root.node_id,
        root.node_type,
        root.parrent_id,
        root.node_name,
        root.deputat_uuid
    ORDER BY root.node_name
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let nodes = query.bind(parrent_id).fetch_all(&pool).await?;
    Ok(Json(nodes))
}

#[derive(Debug, Deserialize)]
struct CreateNode {
    parrent_id: i32,
    node_name: String,
    node_type: NodeType,
}
async fn create_node(
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<CreateNode>,
) -> Result<Json<Node>> {
    let q = r#"
    INSERT INTO node (parrent_id, node_name, node_type)
    VALUES ($1, $2, $3) 
    RETURNING *
    "#;

    let query = sqlx::query_as::<_, Node>(q);

    let node = query
        .bind(&payload.parrent_id)
        .bind(&payload.node_name.trim())
        .bind(&payload.node_type)
        .fetch_one(&pool)
        .await?;
    Ok(Json(node))
}

#[derive(Debug, FromRow, Serialize)]
struct Remove {
    elements_count: i64,
    parrent_id: i32,
}

async fn drop_node(
    State(pool): State<Pool<Postgres>>,
    Path(node_id): Path<i32>,
) -> Result<Json<Remove>> {
    let mut tnx = pool.begin().await?;

    let parrent_q = r#"
    SELECT
        parrent_id
    FROM
        node
    WHERE
        node_id = $1
    "#;

    let query = sqlx::query(parrent_q);
    let row = query.bind(node_id).fetch_one(&mut tnx).await?;
    let parrent_id: i32 = row.get("parrent_id");

    let mut pp = 0;

    if parrent_id > 0 {
        let pp_q = r#"
        SELECT
            parrent_id
        FROM
            node
        WHERE
            node_id = $1
        "#;

        let query = sqlx::query(pp_q);
        let row = query.bind(&parrent_id).fetch_one(&mut tnx).await?;
        pp = row.get("parrent_id")
    }

    let count_q = r#"
    SELECT
        COUNT(node_name)
    FROM
        node
    WHERE
        parrent_id = $1
    "#;

    let query = sqlx::query(count_q);
    let row = query.bind(&parrent_id).fetch_one(&mut tnx).await?;
    let count: i64 = row.get("count");

    let q = r#"
    DELETE FROM
        node
    WHERE
        node_id = $1
        AND (
            SELECT
                COUNT(node_name)
            FROM
                node
            WHERE
                parrent_id = $1
        ) = 0
    returning node_id, parrent_id, node_name
    "#;

    let query = sqlx::query_as::<_, SimpleNode>(q);
    let _node = query.bind(node_id).fetch_one(&mut tnx).await?;

    let remove = Remove {
        elements_count: count - 1,
        parrent_id: pp,
    };

    tnx.commit().await?;

    Ok(Json(remove))
}

#[derive(Debug, Deserialize)]
struct CreateStreet {
    node_id: i32,
    street_name: String,
}

#[derive(Debug, FromRow, Serialize)]
struct Street {
    street_id: i32,
    street_uuid: Uuid,
    street_name: String,
    nested: bool,
}

async fn create_street(
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<CreateStreet>,
) -> Result<Json<Street>> {
    let mut tnx = pool.begin().await?;

    let streets_uuid_q = r#"
    UPDATE node
    SET streets_uuid = COALESCE(streets_uuid, uuid_generate_v4())
    WHERE node_id = $1
    returning streets_uuid
    "#;

    let create_street_q = r#"
    INSERT INTO street (street_uuid, street_name)
    VALUES ($1, $2)
    returning street_id, street_uuid, street_name
    "#;

    let streets_uuid: (Uuid,) = sqlx::query_as(streets_uuid_q)
        .bind(&payload.node_id)
        .fetch_one(&mut tnx)
        .await?;

    dbg!(&streets_uuid);

    let street = sqlx::query_as::<_, Street>(create_street_q)
        .bind(&streets_uuid.0)
        .bind(&payload.street_name.trim_end())
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
    SELECT s.street_id,
    s.street_name,
    s.street_uuid,
    CASE
        WHEN building.street_id >= 0 THEN true
        ELSE false
    END as nested
    FROM street as s
    LEFT JOIN building ON building.street_id = s.street_id
    WHERE s.street_uuid = $1
    GROUP BY s.street_id,
    s.street_uuid,
    s.street_name,
    nested;
    "#;

    // SELECT street_id, street_uuid, street_name
    // FROM street
    // WHERE street_uuid = $1

    let streets = sqlx::query_as::<_, Street>(streets_q)
        .bind(&uuid)
        .fetch_all(&pool)
        .await?;

    Ok(Json(streets))
}

#[derive(Debug, Deserialize)]
struct CreateBuilding {
    street_id: i32,
    building_name: String,
}

#[derive(Debug, FromRow, Serialize)]
struct Building {
    building_id: i32,
    street_id: i32,
    building_name: String,
}

async fn create_building(
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<CreateBuilding>,
) -> Result<Json<Building>> {
    let create_building_q = r#"
    INSERT INTO building (street_id, building_name)
    VALUES ($1, $2),
    returning building_id, street_id, building_name
    "#;

    let building = sqlx::query_as::<_, Building>(create_building_q)
        .bind(&payload.street_id)
        .bind(&payload.building_name.trim())
        .fetch_one(&pool)
        .await?;

    Ok(Json(building))
}

async fn get_buildings(
    State(pool): State<Pool<Postgres>>,
    Path(street_id): Path<i32>,
) -> Result<Json<Vec<Building>>> {
    let buildings_q = r#"
    SELECT building_id, street_id, building_name
    FROM building
    WHERE street_id = $1
    "#;

    let buildings = sqlx::query_as::<_, Building>(buildings_q)
        .bind(&street_id)
        .fetch_all(&pool)
        .await?;

    Ok(Json(buildings))
}

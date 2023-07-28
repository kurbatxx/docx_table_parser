mod model;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

use strum_macros::EnumString;
use tokio::fs;
use uuid::Uuid;

use crate::model::{District, Raion};

#[derive(Debug, FromRow, Serialize)]
pub struct Node {
    node_id: i32,
    node_type: NodeType,
    parrent_id: i32,
    node_name: String,
    #[sqlx(default)]
    has_nest: Option<bool>,
    #[sqlx(default)]
    deputat_uuid: Option<Uuid>,
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

#[tokio::main]
async fn main() -> Result<()> {
    let file = fs::read("resut_v2.json").await.unwrap();
    let raion: Raion = serde_json::from_slice(&file).unwrap();

    let url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    create_dep_table(&pool).await?;

    // raion.districts.into_iter().for_each(|dis| {
    //     println!("{}", &dis.candidate);
    //     let d = tokio::spawn(async move { insert_dep(&pool, &dis).await });

    //     println!("***")
    // });

    for dis in raion.districts {
        println!("{}", &dis.candidate);
        let d = insert_dep(&pool, &dis).await?;
        println!("{}", &d.deputat_uuid);

        insert_streets(&pool, &dis, 19, &d.deputat_uuid).await?
    }

    Ok(())
}

async fn create_dep_table(pool: &PgPool) -> Result<()> {
    let q = r#" 
        DROP TABLE IF EXISTS deputat
        "#;

    let query = sqlx::query(q);
    let _ = query.fetch_optional(pool).await?;

    let q = r#"
    CREATE TABLE deputat(
        deputat_uuid uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
        deputat_name text)
    "#;

    let query = sqlx::query(q);
    let _ = query.fetch_optional(pool).await?;

    Ok(())
}

#[derive(FromRow)]
struct Deputat {
    deputat_uuid: Uuid,
    deputat_name: String,
}

async fn insert_dep(pool: &PgPool, dis: &District) -> Result<Deputat> {
    let q = r#"
    INSERT INTO deputat(deputat_name)
    VALUES($1)
    RETURNING *
    "#;

    let query = sqlx::query_as::<_, Deputat>(q);
    let dep = query.bind(&dis.candidate).fetch_one(pool).await?;

    Ok(dep)
}

async fn insert_streets(
    pool: &PgPool,
    dis: &District,
    parrent_id: i32,
    dep_uuid: &Uuid,
) -> Result<()> {
    let q = r#"
    INSERT INTO node(node_name, parrent_id, node_type)
    VALUES($1, $2, $3)
    RETURNING *
    "#;

    for item in &dis.streets {
        let query = sqlx::query_as::<_, Node>(q);
        let node = query
            .bind(&item.name)
            .bind(parrent_id)
            .bind(NodeType::Street)
            .fetch_one(pool)
            .await;

        match node {
            Ok(node) => match item.numbers.clone() {
                Some(numbers) => {
                    for n in numbers {
                        instert_building(pool, &n, node.node_id, dep_uuid).await
                    }
                }
                None => {
                    dbg!("NO BUILDINGS");
                }
            },
            Err(err) => {
                println!("{}", item.name);
            }
        }
    }
    Ok(())
}

async fn instert_building(pool: &PgPool, name: &str, parrent_id: i32, deputat: &Uuid) {
    let q = r#"
    INSERT INTO node(node_name, parrent_id, node_type, deputat_uuid)
    VALUES($1, $2, $3, $4)
    RETURNING *
    "#;

    let query = sqlx::query_as::<_, Node>(q);
    let node = query
        .bind(&name)
        .bind(parrent_id)
        .bind(NodeType::Building)
        .bind(deputat)
        .fetch_one(pool)
        .await;
}
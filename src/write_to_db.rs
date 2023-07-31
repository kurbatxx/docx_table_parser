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
    deputat_id: Option<i32>,
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
        println!("{}", &d.deputat_id);

        insert_streets(&pool, &dis, 19, d.deputat_id).await?
    }

    Ok(())
}

async fn create_dep_table(pool: &PgPool) -> Result<()> {
    let q = r#" 
        DROP TABLE IF EXISTS deputat_info
        "#;

    let query = sqlx::query(q);
    let _ = query.fetch_optional(pool).await?;

    let q = r#"
    CREATE TABLE deputat_info(
        deputat_id SERIAL PRIMARY KEY,
        deputat_name text,
        uch_number integer)
    "#;

    let query = sqlx::query(q);
    let _ = query.fetch_optional(pool).await?;

    Ok(())
}

#[derive(FromRow)]
struct Deputat {
    deputat_id: i32,
    deputat_name: String,
    uch_number: i32,
}

async fn insert_dep(pool: &PgPool, dis: &District) -> Result<Deputat> {
    let q = r#"
    INSERT INTO deputat_info(deputat_name, uch_number)
    VALUES($1, $2)
    RETURNING *
    "#;

    let query = sqlx::query_as::<_, Deputat>(q);
    let dep = query
        .bind(&dis.candidate)
        .bind(
            &dis.num
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or_default(),
        )
        .fetch_one(pool)
        .await?;

    Ok(dep)
}

async fn insert_streets(pool: &PgPool, dis: &District, parrent_id: i32, dep_id: i32) -> Result<()> {
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
                        instert_building(pool, &n, node.node_id, dep_id).await
                    }
                }
                None => {
                    dbg!("NO BUILDINGS");
                }
            },
            Err(_err) => {
                println!("{}, {}", item.name, parrent_id);

                let q = r#"
                SELECT * FROM node
                WHERE parrent_id = $1 AND node_name = $2
                LIMIT 1;
                "#;

                let query = sqlx::query_as::<_, Node>(q);
                let node = query
                    .bind(parrent_id)
                    .bind(&item.name)
                    .fetch_one(pool)
                    .await;

                match node {
                    Ok(node) => match item.numbers.clone() {
                        Some(numbers) => {
                            for n in numbers {
                                instert_building(pool, &n, node.node_id, dep_id).await
                            }
                        }
                        None => {
                            dbg!("NO BUILDINGS");
                        }
                    },
                    Err(_) => {
                        dbg!("dublicate not found");
                    }
                }
            }
        }
    }
    Ok(())
}

async fn instert_building(pool: &PgPool, name: &str, parrent_id: i32, deputat: i32) {
    let q = r#"
    INSERT INTO node(node_name, parrent_id, node_type, deputat_id)
    VALUES($1, $2, $3, $4)
    RETURNING *
    "#;

    let query = sqlx::query_as::<_, Node>(q);
    let _node = query
        .bind(&name)
        .bind(parrent_id)
        .bind(NodeType::Building)
        .bind(deputat)
        .fetch_one(pool)
        .await;
}

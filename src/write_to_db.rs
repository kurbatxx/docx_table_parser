mod model;

use anyhow::{Ok, Result};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use tokio::fs;
use uuid::Uuid;

use crate::model::{District, Raion};

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

    raion.districts.into_iter().for_each(|dis| {
        println!("{}", &dis.candidate);
        let d = tokio::spawn(async move { insert_dep(&pool, &dis).await });

        println!("***")
    });

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
    INSERT INTO deputat(
        deputat_name = $1
    ) RETURNING *
    "#;

    let query = sqlx::query_as::<_, Deputat>(q);
    let dep = query.bind(&dis.candidate).fetch_one(pool).await?;

    Ok(dep)
}

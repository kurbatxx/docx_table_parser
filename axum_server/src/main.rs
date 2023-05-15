use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "";

    let mut conn = sqlx::postgres::PgConnection::connect(url).await?;

    let routes_all = Router::new().merge(db_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn db_routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

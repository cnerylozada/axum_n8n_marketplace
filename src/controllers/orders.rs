use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Serialize, FromRow)]
struct Token {
    chain: String,
}

async fn get_foo(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<Token>>, String> {
    let tokens = sqlx::query_as::<_, Token>("SELECT * FROM tokens")
        .fetch_all(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(tokens))
}

pub fn orders_routes() -> Router<Pool<Postgres>> {
    Router::new().route("/", get(get_foo))
}

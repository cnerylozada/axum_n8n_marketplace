use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Serialize, FromRow)]
struct User {
    name: String,
    age: i64,
}

async fn get_inventory(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<User>>, String> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(users))
}

pub fn inventory_routes() -> Router<Pool<Postgres>> {
    Router::new().route("/", get(get_inventory))
}

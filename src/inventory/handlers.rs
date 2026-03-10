use crate::inventory::models::User;
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};

pub async fn get_inventory(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<User>>, String> {
    let users = sqlx::query_as::<_, User>("SELECT * from users")
        .fetch_all(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(users))
}

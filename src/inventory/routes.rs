use crate::inventory::handlers::get_inventory;
use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn inventory_routes() -> Router<Pool<Postgres>> {
    Router::new().route("/", get(get_inventory))
}

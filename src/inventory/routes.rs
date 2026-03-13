use crate::inventory::handlers::{get_inventory, update_inventory};
use axum::{
    Router,
    routing::{get, put},
};
use sqlx::{Pool, Postgres};

pub fn inventory_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(get_inventory))
        .route("/{raw_product_id}", put(update_inventory))
}

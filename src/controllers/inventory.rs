use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Inventory {
    name: String,
    age: u8,
}

async fn get_inventory() -> Json<Inventory> {
    let inventory = Inventory {
        name: String::from("Cristh"),
        age: 21,
    };
    Json(inventory)
}

pub fn inventory_routes() -> Router {
    Router::new().route("/", get(get_inventory))
}

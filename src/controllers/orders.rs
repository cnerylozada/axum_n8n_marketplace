use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Order {
    owner: String,
}

async fn get_foo() -> Json<Order> {
    let order = Order {
        owner: String::from("Lucciano"),
    };
    Json(order)
}

pub fn orders_routes() -> Router {
    Router::new().route("/", get(get_foo))
}

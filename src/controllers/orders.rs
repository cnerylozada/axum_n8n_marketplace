use axum::{Router, routing::get};

async fn get_foo() -> String {
    "orders".to_string()
}

pub fn orders_routes() -> Router {
    Router::new().route("/", get(get_foo))
}

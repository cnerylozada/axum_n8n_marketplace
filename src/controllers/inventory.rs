use axum::{Router, routing::get};

async fn get_foo() -> String {
    "inventory".to_string()
}

pub fn inventory_routes() -> Router {
    Router::new().route("/", get(get_foo))
}

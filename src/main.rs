use axum::{Router, routing::get};
mod controllers;

use controllers::inventory::inventory_routes;
use controllers::orders::orders_routes;

#[tokio::main]
async fn main() {
    // build our application with a single route

    let api_routes = Router::new()
        .nest("/inventory", inventory_routes())
        .nest("/orders", orders_routes());

    let app = Router::new()
        .route("/", get(|| async { "Welcome to my Axum API!" }))
        .nest("/api/v1", api_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

mod employees;
mod inventory;
use employees::routes::employees_routes;
use inventory::routes::inventory_routes;

async fn database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = database_connection().await.unwrap();

    let api_routes = Router::new()
        .nest("/inventory_variants", inventory_routes())
        .nest("/employees", employees_routes())
        .with_state(pool);

    let app = Router::new()
        .route("/", get(|| async { "Welcome to my Axum API!" }))
        .nest("/api/v1", api_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

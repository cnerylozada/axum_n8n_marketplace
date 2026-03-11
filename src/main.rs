use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

mod inventory;
use inventory::routes::inventory_routes;

async fn database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            "postgresql://postgres.jhlasmhscmscolxkayvd:19467381Abc.@aws-1-us-east-1.pooler.supabase.com:5432/postgres"
        )
        .await?;

    Ok(pool)
}

#[tokio::main]
async fn main() {
    let pool = database_connection().await.unwrap();

    let api_routes = Router::new()
        .nest("/inventory_variants", inventory_routes())
        .with_state(pool);

    let app = Router::new()
        .route("/", get(|| async { "Welcome to my Axum API!" }))
        .nest("/api/v1", api_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

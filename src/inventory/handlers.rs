use crate::inventory::models::{InventoryVariant, UpdateStockPayload};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::{Pool, Postgres, types::Uuid};
use validator::Validate;

pub async fn get_inventory(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<InventoryVariant>>, String> {
    let query = r#"
        SELECT 
            iv.id,
            p.name AS product_name,
            iv.size,
            iv.stock_quantity
        FROM inventory_variants iv
        JOIN products p ON iv.product_id = p.id;
    "#;
    let inventory_variant_list = sqlx::query_as::<_, InventoryVariant>(query)
        .fetch_all(&pool)
        .await
        .map_err(|error| error.to_string())?;

    Ok(Json(inventory_variant_list))
}

pub async fn update_inventory(
    State(pool): State<Pool<Postgres>>,
    Path(raw_product_id): Path<String>,
    Json(payload): Json<UpdateStockPayload>,
) -> Result<(), String> {
    let product_id = Uuid::parse_str(&raw_product_id).map_err(|error| error.to_string())?;
    payload.validate().map_err(|error| error.to_string())?;

    Ok(())
}

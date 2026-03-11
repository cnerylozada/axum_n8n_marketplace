use crate::inventory::models::InventoryVariant;
use axum::{Json, extract::State};
use sqlx::{Pool, Postgres};

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

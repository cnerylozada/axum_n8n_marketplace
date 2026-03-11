use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct InventoryVariant {
    id: Uuid,
    product_name: String,
    size: String,
    stock_quantity: i32,
}

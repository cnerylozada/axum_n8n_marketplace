use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, FromRow)]
pub struct InventoryVariant {
    id: Uuid,
    product_name: String,
    size: String,
    stock_quantity: i32,
}

#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateStockPayload {
    #[validate(range(min = 0, max = 5))]
    pub stock_quantity: i32,
    #[validate(length(min = 2, max = 4))]
    pub size: String,
}

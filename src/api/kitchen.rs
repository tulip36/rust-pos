use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct KitchenOrder {
    pub id: String,
    pub table_id: Option<String>,
    pub status: String,
    pub items: Vec<KitchenOrderItem>,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct KitchenOrderItem {
    pub id: String,
    pub product_name: String,
    pub quantity: i32,
    pub notes: Option<String>,
    pub status: String,
}

pub async fn get_kitchen_orders() -> Json<Vec<KitchenOrder>> {
    Json(vec![])
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderRequest {
    pub order_id: Option<String>,
    pub item_id: Option<String>,
    pub status: Option<String>,
}

pub async fn update_kitchen_order(
    Json(payload): Json<UpdateOrderRequest>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": true,
    }))
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct KitchenOrder {
    pub id: String,
    pub table_id: Option<String>,
    pub status: String,
    pub items: Vec<KitchenOrderItem>,
    pub created_at: i64,
    pub elapsed_minutes: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct KitchenOrderItem {
    pub id: String,
    pub product_name: String,
    pub quantity: i32,
    pub notes: Option<String>,
    pub status: String,
}

// 获取厨房订单队列
pub async fn get_kitchen_orders() -> Json<Vec<KitchenOrder>> {
    let orders = vec![];
    Json(orders)
}

// 更新厨房订单状态
#[derive(Debug, Deserialize)]
pub struct UpdateKitchenRequest {
    pub order_id: Option<String>,
    pub item_id: Option<String>,
    pub status: Option<String>,
}

pub async fn update_kitchen_order(
    Json(payload): Json<UpdateKitchenRequest>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": true,
    }))
}

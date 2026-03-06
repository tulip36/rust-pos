use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CartItem {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<CartItem>,
    pub table_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub success: bool,
    pub order_id: String,
    pub order: Order,
}

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub id: String,
    pub status: String,
    pub subtotal: f64,
    pub tax: f64,
    pub tip: f64,
    pub total: f64,
}

pub async fn create_order(
    Json(payload): Json<CreateOrderRequest>,
) -> Json<OrderResponse> {
    let order_id = uuid::Uuid::new_v4().to_string();
    
    // 计算价格
    let subtotal: f64 = payload.items.iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();
    
    // 税率 8.875%
    let tax = subtotal * 0.08875;
    let total = subtotal + tax;
    
    Json(OrderResponse {
        success: true,
        order_id: order_id.clone(),
        order: Order {
            id: order_id,
            status: "pending".to_string(),
            subtotal,
            tax,
            tip: 0.0,
            total,
        },
    })
}

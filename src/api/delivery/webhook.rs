use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeliveryOrderRequest {
    pub platform: String, // "doordash", "ubereats", "grubhub"
    pub order_id: String,
    pub customer_name: String,
    pub delivery_address: String,
    pub items: Vec<serde_json::Value>,
}

pub async fn receive_order(
    Json(payload): Json<DeliveryOrderRequest>,
) -> Json<serde_json::Value> {
    let order_id = uuid::Uuid::new_v4().to_string();
    Json(serde_json::json!({
        "success": true,
        "order_id": order_id,
        "platform": payload.platform,
    }))
}

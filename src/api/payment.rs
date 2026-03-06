use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub order_id: String,
    pub payment_method: String,
    pub tip: Option<f64>,
}

pub async fn process_payment(
    Json(payload): Json<PaymentRequest>,
) -> Json<serde_json::Value> {
    // 简化版：直接返回成功
    // 实际需要对接 Stripe API
    Json(serde_json::json!({
        "success": true,
        "payment_id": uuid::Uuid::new_v4().to_string(),
    }))
}

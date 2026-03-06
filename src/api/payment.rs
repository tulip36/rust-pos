use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub order_id: String,
    pub payment_method: String, // "card", "apple_pay", "google_pay", "cash"
    pub tip_percent: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessPaymentRequest {
    pub order_id: String,
    pub payment_method: String,
    pub tip: Option<f64>,
}

pub async fn get_payment_methods() -> Json<Vec<serde_json::Value>> {
    Json(vec![
        serde_json::json!({
            "id": "card",
            "name": "信用卡/借记卡",
            "icon": "credit-card"
        }),
        serde_json::json!({
            "id": "apple_pay",
            "name": "Apple Pay",
            "icon": "apple"
        }),
        serde_json::json!({
            "id": "google_pay",
            "name": "Google Pay",
            "icon": "google"
        }),
        serde_json::json!({
            "id": "cash",
            "name": "现金",
            "icon": "wallet"
        }),
    ])
}

pub async fn process_payment(
    Json(payload): Json<ProcessPaymentRequest>,
) -> Json<serde_json::Value> {
    // 简化版：直接返回成功
    // 实际需要对接 Stripe API
    Json(serde_json::json!({
        "success": true,
        "payment_id": uuid::Uuid::new_v4().to_string(),
        "message": "支付成功"
    }))
}

pub async fn get_tip_options() -> Json<Vec<serde_json::Value>> {
    Json(vec![
        serde_json::json!({"percent": 15, "label": "15%"}),
        serde_json::json!({"percent": 18, "label": "18%"}),
        serde_json::json!({"percent": 20, "label": "20%"}),
        serde_json::json!({"percent": 25, "label": "25%"}),
    ])
}

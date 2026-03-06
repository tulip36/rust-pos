use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Order {
    pub id: String,
    pub status: String,
    pub subtotal: f64,
    pub tax: f64,
    pub tip: f64,
    pub total: f64,
    pub created_at: i64,
}

pub async fn get_orders() -> Json<Vec<Order>> {
    Json(vec![])
}

pub async fn get_order(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<Option<Order>> {
    Json(Some(Order {
        id,
        status: "pending".to_string(),
        subtotal: 50.0,
        tax: 4.44,
        tip: 0.0,
        total: 54.44,
        created_at: chrono::Utc::now().timestamp_millis(),
    }))
}

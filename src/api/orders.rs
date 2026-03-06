use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub id: String,
    pub status: String,
    pub table_id: Option<String>,
    pub items: Vec<OrderItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub tip: f64,
    pub total: f64,
    pub payment_method: Option<String>,
    pub source: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderItem {
    pub id: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderList {
    pub orders: Vec<Order>,
}

// 订单状态列表
pub fn get_status_list() -> Vec<(&'static str, &'static str)> {
    vec![
        ("pending", "待确认"),
        ("paid", "已支付"),
        ("preparing", "制作中"),
        ("ready", "已完成"),
        ("delivered", "已送达"),
        ("cancelled", "已取消"),
    ]
}

pub async fn get_orders(
    user_id: Option<String>,
) -> Json<OrderList> {
    // 示例订单数据
    let orders = vec![];
    Json(OrderList { orders })
}

pub async fn get_order(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<Option<Order>> {
    let order = Order {
        id: id.clone(),
        status: "preparing".to_string(),
        table_id: Some("A1".to_string()),
        items: vec![
            OrderItem {
                id: "1".to_string(),
                product_name: "宫保鸡丁".to_string(),
                quantity: 2,
                unit_price: 18.99,
                notes: None,
            },
        ],
        subtotal: 37.98,
        tax: 3.37,
        tip: 0.0,
        total: 41.35,
        payment_method: Some("card".to_string()),
        source: "dine_in".to_string(),
        created_at: chrono::Utc::now().timestamp_millis(),
    };
    Json(Some(order))
}

pub async fn get_order_status(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    // 模拟订单状态
    Json(serde_json::json!({
        "order_id": id,
        "status": "preparing",
        "status_text": "制作中",
        "progress": 50,
        "steps": [
            {"key": "pending", "label": "已下单", "completed": true},
            {"key": "paid", "label": "已支付", "completed": true},
            {"key": "preparing", "label": "制作中", "completed": false, "current": true},
            {"key": "ready", "label": "已完成", "completed": false}
        ]
    }))
}

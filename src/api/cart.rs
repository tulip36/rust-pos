use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 内存中的购物车存储
static CARTS: Lazy<Mutex<HashMap<String, Vec<CartItem>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub total: f64,
}

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub session_id: String,
    pub item: CartItem,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartRequest {
    pub session_id: String,
    pub item_id: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub session_id: String,
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

// 添加商品到购物车
pub async fn add_to_cart(
    Json(payload): Json<AddToCartRequest>,
) -> Json<serde_json::Value> {
    let mut carts = CARTS.lock().unwrap();
    
    let items = carts.entry(payload.session_id.clone()).or_insert_with(Vec::new);
    
    // 检查是否已存在
    if let Some(existing) = items.iter_mut().find(|i| i.id == payload.item.id) {
        existing.quantity += payload.item.quantity;
    } else {
        items.push(payload.item);
    }
    
    // 计算购物车总额
    let subtotal: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let tax = subtotal * 0.08875;
    let total = subtotal + tax;
    
    Json(serde_json::json!({
        "success": true,
        "cart": {
            "items": items,
            "subtotal": subtotal,
            "tax": tax,
            "total": total,
        }
    }))
}

// 更新购物车商品数量
pub async fn update_cart(
    Json(payload): Json<UpdateCartRequest>,
) -> Json<serde_json::Value> {
    let mut carts = CARTS.lock().unwrap();
    
    if let Some(items) = carts.get_mut(&payload.session_id) {
        if payload.quantity <= 0 {
            items.retain(|i| i.id != payload.item_id);
        } else if let Some(item) = items.iter_mut().find(|i| i.id == payload.item_id) {
            item.quantity = payload.quantity;
        }
    }
    
    let items = carts.get(&payload.session_id).cloned().unwrap_or_default();
    let subtotal: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let tax = subtotal * 0.08875;
    let total = subtotal + tax;
    
    Json(serde_json::json!({
        "success": true,
        "cart": {
            "items": items,
            "subtotal": subtotal,
            "tax": tax,
            "total": total,
        }
    }))
}

// 获取购物车
pub async fn get_cart(
    session_id: String,
) -> Json<serde_json::Value> {
    let carts = CARTS.lock().unwrap();
    let items = carts.get(&session_id).cloned().unwrap_or_default();
    
    let subtotal: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let tax = subtotal * 0.08875;
    let total = subtotal + tax;
    
    Json(serde_json::json!({
        "cart": {
            "items": items,
            "subtotal": subtotal,
            "tax": tax,
            "total": total,
        }
    }))
}

// 清空购物车
pub async fn clear_cart(
    session_id: String,
) -> Json<serde_json::Value> {
    let mut carts = CARTS.lock().unwrap();
    carts.remove(&session_id);
    
    Json(serde_json::json!({
        "success": true,
    }))
}

// 创建订单
pub async fn create_order(
    Json(payload): Json<CreateOrderRequest>,
) -> Json<OrderResponse> {
    let mut carts = CARTS.lock().unwrap();
    let items = carts.remove(&payload.session_id).unwrap_or_default();
    
    if items.is_empty() {
        return Json(OrderResponse {
            success: false,
            order_id: String::new(),
            order: Order {
                id: String::new(),
                status: "empty".to_string(),
                subtotal: 0.0,
                tax: 0.0,
                tip: 0.0,
                total: 0.0,
            },
        });
    }
    
    let order_id = uuid::Uuid::new_v4().to_string();
    let subtotal: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
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

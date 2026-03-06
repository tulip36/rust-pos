use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category_id: Option<String>,
    pub image_url: Option<String>,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub products: Vec<Product>,
}

// 获取完整菜单
pub async fn get_menu() -> Json<Vec<MenuCategory>> {
    // 简化版：返回示例数据
    // 实际需要从数据库查询
    let menu = vec![
        MenuCategory {
            id: "1".to_string(),
            name: "凉菜".to_string(),
            description: Some("开胃小菜".to_string()),
            products: vec![
                Product {
                    id: "1".to_string(),
                    name: "凉拌黄瓜".to_string(),
                    description: Some("清脆爽口".to_string()),
                    price: 8.99,
                    category_id: Some("1".to_string()),
                    image_url: None,
                    is_available: true,
                },
                Product {
                    id: "2".to_string(),
                    name: "卤牛肉".to_string(),
                    description: Some("秘制卤味".to_string()),
                    price: 28.99,
                    category_id: Some("1".to_string()),
                    image_url: None,
                    is_available: true,
                },
            ],
        },
        MenuCategory {
            id: "2".to_string(),
            name: "热菜".to_string(),
            description: Some("主菜".to_string()),
            products: vec![
                Product {
                    id: "3".to_string(),
                    name: "宫保鸡丁".to_string(),
                    description: Some("经典川菜".to_string()),
                    price: 18.99,
                    category_id: Some("2".to_string()),
                    image_url: None,
                    is_available: true,
                },
                Product {
                    id: "4".to_string(),
                    name: "麻婆豆腐".to_string(),
                    description: Some("麻辣鲜香".to_string()),
                    price: 15.99,
                    category_id: Some("2".to_string()),
                    image_url: None,
                    is_available: true,
                },
            ],
        },
    ];
    
    Json(menu)
}

#[derive(Debug, Deserialize)]
pub struct CreateMenuItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category_id: Option<String>,
}

pub async fn create_menu_item(
    Json(payload): Json<CreateMenuItemRequest>,
) -> Json<serde_json::Value> {
    let id = uuid::Uuid::new_v4().to_string();
    Json(serde_json::json!({
        "success": true,
        "id": id,
        "name": payload.name,
    }))
}

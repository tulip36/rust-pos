use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub products: Vec<Product>,
}

// 获取完整菜单
pub async fn get_menu() -> Json<Vec<MenuCategory>> {
    let menu = vec![
        MenuCategory {
            id: "1".to_string(),
            name: "凉菜".to_string(),
            description: Some("开胃小菜".to_string()),
            products: vec![
                Product {
                    id: "101".to_string(),
                    name: "凉拌黄瓜".to_string(),
                    description: Some("清脆爽口".to_string()),
                    price: 8.99,
                    image_url: None,
                },
                Product {
                    id: "102".to_string(),
                    name: "凉拌木耳".to_string(),
                    description: Some("营养健康".to_string()),
                    price: 10.99,
                    image_url: None,
                },
                Product {
                    id: "103".to_string(),
                    name: "卤牛肉".to_string(),
                    description: Some("秘制卤味".to_string()),
                    price: 28.99,
                    image_url: None,
                },
            ],
        },
        MenuCategory {
            id: "2".to_string(),
            name: "热菜".to_string(),
            description: Some("主菜".to_string()),
            products: vec![
                Product {
                    id: "201".to_string(),
                    name: "宫保鸡丁".to_string(),
                    description: Some("经典川菜".to_string()),
                    price: 18.99,
                    image_url: None,
                },
                Product {
                    id: "202".to_string(),
                    name: "麻婆豆腐".to_string(),
                    description: Some("麻辣鲜香".to_string()),
                    price: 15.99,
                    image_url: None,
                },
                Product {
                    id: "203".to_string(),
                    name: "红烧肉".to_string(),
                    description: Some("肥而不腻".to_string()),
                    price: 25.99,
                    image_url: None,
                },
                Product {
                    id: "204".to_string(),
                    name: "鱼香肉丝".to_string(),
                    description: Some("酸甜可口".to_string()),
                    price: 19.99,
                    image_url: None,
                },
            ],
        },
        MenuCategory {
            id: "3".to_string(),
            name: "主食".to_string(),
            description: Some("米饭面条".to_string()),
            products: vec![
                Product {
                    id: "301".to_string(),
                    name: "白米饭".to_string(),
                    description: Some("东北大米".to_string()),
                    price: 3.99,
                    image_url: None,
                },
                Product {
                    id: "302".to_string(),
                    name: "扬州炒饭".to_string(),
                    description: Some("粒粒分明".to_string()),
                    price: 12.99,
                    image_url: None,
                },
                Product {
                    id: "303".to_string(),
                    name: "拉面".to_string(),
                    description: Some("手工拉面".to_string()),
                    price: 15.99,
                    image_url: None,
                },
            ],
        },
        MenuCategory {
            id: "4".to_string(),
            name: "汤品".to_string(),
            description: Some("鲜美汤类".to_string()),
            products: vec![
                Product {
                    id: "401".to_string(),
                    name: "鸡蛋汤".to_string(),
                    description: Some("家常味道".to_string()),
                    price: 6.99,
                    image_url: None,
                },
                Product {
                    id: "402".to_string(),
                    name: "酸辣汤".to_string(),
                    description: Some("开胃佳品".to_string()),
                    price: 8.99,
                    image_url: None,
                },
            ],
        },
        MenuCategory {
            id: "5".to_string(),
            name: "饮料".to_string(),
            description: Some("饮品".to_string()),
            products: vec![
                Product {
                    id: "501".to_string(),
                    name: "可乐".to_string(),
                    description: Some("冰镇可口".to_string()),
                    price: 3.99,
                    image_url: None,
                },
                Product {
                    id: "502".to_string(),
                    name: "雪碧".to_string(),
                    description: Some("清爽解渴".to_string()),
                    price: 3.99,
                    image_url: None,
                },
                Product {
                    id: "503".to_string(),
                    name: "橙汁".to_string(),
                    description: Some("鲜榨果汁".to_string()),
                    price: 5.99,
                    image_url: None,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category_id: Option<String>,
    pub is_available: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category_id: Option<String>,
}

pub async fn get_products() -> Json<Vec<Product>> {
    Json(vec![])
}

pub async fn create_product(
    Json(payload): Json<CreateProductRequest>,
) -> Json<serde_json::Value> {
    let id = uuid::Uuid::new_v4().to_string();
    Json(serde_json::json!({
        "success": true,
        "id": id,
    }))
}

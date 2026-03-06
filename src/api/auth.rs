use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    Json(payload): Json<RegisterRequest>,
) -> Json<serde_json::Value> {
    let user_id = uuid::Uuid::new_v4().to_string();
    Json(serde_json::json!({
        "success": true,
        "user": {
            "id": user_id,
            "email": payload.email,
        }
    }))
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let user_id = uuid::Uuid::new_v4().to_string();
    let token = uuid::Uuid::new_v4().to_string();
    Json(serde_json::json!({
        "success": true,
        "token": token,
        "user": {
            "id": user_id,
            "email": payload.email,
        }
    }))
}

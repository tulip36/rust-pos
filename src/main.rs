use std::net::SocketAddr;
use axum::{
    Router,
    routing::get,
    response::Json,
};
use tower_http::cors::{CorsLayer, Any};

#[derive(serde::Serialize)]
struct Status {
    status: String,
    version: String,
}

async fn status() -> Json<Status> {
    Json(Status {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 路由
    let app = Router::new()
        .route("/api/status", get(status))
        .route("/api/menu", get(api::menu::get_menu))
        .route("/api/menu", post(api::menu::create_menu_item))
        .route("/api/cart", post(api::cart::create_order))
        .route("/api/orders", get(api::orders::get_orders))
        .route("/api/orders/:id", get(api::orders::get_order))
        .route("/api/payment", post(api::payment::process_payment))
        .route("/api/auth/register", post(api::auth::register))
        .route("/api/auth/login", post(api::auth::login))
        .route("/api/kitchen/orders", get(api::kitchen::get_kitchen_orders))
        .route("/api/kitchen/orders", patch(api::kitchen::update_kitchen_order))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod api {
    pub mod menu;
    pub mod cart;
    pub mod orders;
    pub mod payment;
    pub mod auth;
    pub mod kitchen;
}

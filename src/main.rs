#![allow(clippy::result_large_err)]


use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::get,
};
use once_cell::sync::Lazy;
use svix_ksuid::{KsuidLike, KsuidMs};

pub static INSTANCE_ID: Lazy<String> = Lazy::new(|| KsuidMs::new(None, None).to_string());

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8050").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({})))
}

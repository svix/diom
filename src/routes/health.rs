use axum::routing::get;
use axum::{Router, extract::Json, http::StatusCode};

#[tracing::instrument]
async fn ping() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({})))
}

pub fn router() -> Router {
    Router::new()
        .route("/health/ping", get(ping).head(ping))
}

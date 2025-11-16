#![allow(clippy::result_large_err)]


use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    routing::get,
};
use once_cell::sync::Lazy;
use svix_ksuid::{KsuidLike, KsuidMs};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;

pub static INSTANCE_ID: Lazy<String> = Lazy::new(|| KsuidMs::new(None, None).to_string());

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting server with instance_id: {}", *INSTANCE_ID);

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/kv", routes::kv::router())
        .nest("/ratelimit", routes::ratelimit::router())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8050").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({})))
}

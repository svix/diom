use axum::Router;

pub mod health;
pub mod kv;
pub mod ratelimit;

pub fn router() -> Router {
    Router::new().merge(health::router()).merge(kv::router()).merge(ratelimit::router())
}

mod admin;
mod auth_token;
mod bootstrap;
mod cache;
mod cluster;
mod cluster_admin;
mod common;
mod health;
mod idempotency;
mod jwt_auth;
mod kv;
mod msgpack;
mod msgs;
mod rate_limit;
mod schema_snapshot;

#[ctor::ctor]
fn test_setup() {
    diom_backend::setup_tracing_for_tests();
}

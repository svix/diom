mod admin;
mod auth_token;
mod bootstrap;
mod cache;
mod idempotency;
mod jwt_auth;
mod kv;
mod msgpack;
mod msgs;
mod rate_limit;

#[ctor::ctor]
fn test_setup() {
    diom::setup_tracing_for_tests();
}

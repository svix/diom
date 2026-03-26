mod admin;
mod auth_token;
mod bootstrap;
mod cache;
mod idempotency;
mod kv;
mod msgpack;
mod msgs;
mod rate_limit;

#[ctor::ctor]
fn test_setup() {
    coyote::setup_tracing_for_tests();
}

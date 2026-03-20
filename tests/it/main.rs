mod admin;
mod bootstrap;
mod cache;
mod idempotency;
mod kv;
mod msgpack;
mod msgs;
mod rate_limit;
mod stream;

#[ctor::ctor]
fn test_setup() {
    diom::setup_tracing_for_tests();
}

mod admin;
mod auth_token;
mod bootstrap;
mod cache;
mod idempotency;
mod kv;
mod msgpack;
mod msgs;
mod rate_limit;
mod transformations;

#[ctor::ctor]
fn test_setup() {
    // If this process was re-exec'd as a transform worker subprocess, run the
    // worker event loop and exit — never reaching the test harness.
    if std::env::args().nth(1).as_deref() == Some("transform-worker") {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(diom_transformations::run_as_worker())
            .expect("worker exited with error");
        std::process::exit(0);
    }
    diom::setup_tracing_for_tests();
}

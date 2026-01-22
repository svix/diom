use std::fs;

use anyhow::Context as _;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut openapi = coyote::openapi::initialize_openapi();

    let router = coyote::v1::router();
    _ = aide::axum::ApiRouter::new()
        .nest("/api/v1", router)
        .finish_api_with(&mut openapi, coyote::openapi::add_security_scheme);

    coyote::openapi::postprocess_spec(&mut openapi);

    let openapi_json = serde_json::to_string_pretty(&openapi)?;
    fs::write("openapi.json", &openapi_json).context("writing openapi.json")?;

    Ok(())
}

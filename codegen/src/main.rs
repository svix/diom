use std::{collections::BTreeSet, fs};

use anyhow::Context as _;
use openapi_codegen::{IncludeMode, api::Api, schemars::schema::Schema};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

const OUTPUTS: &[(&str, &str)] = &[
    (
        "codegen/templates/rust/api_summary.rs.jinja",
        "clients/rust/src/api",
    ),
    (
        "codegen/templates/rust/api_resource.rs.jinja",
        "clients/rust/src/api",
    ),
    (
        "codegen/templates/rust/component_type_summary.rs.jinja",
        "clients/rust/src/models",
    ),
    (
        "codegen/templates/rust/component_type.rs.jinja",
        "clients/rust/src/models",
    ),
];

fn main() -> anyhow::Result<()> {
    anyhow::ensure!(
        cfg!(feature = "generate"),
        "must enable --feature=generate to generate code"
    );

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut openapi = coyote::openapi::initialize_openapi();

    let router = coyote::v1::router();
    _ = aide::axum::ApiRouter::new()
        .nest("/api/v1", router)
        .finish_api_with(&mut openapi, coyote::openapi::add_security_scheme);

    coyote::openapi::postprocess_spec(&mut openapi);

    let openapi_json = serde_json::to_string_pretty(&openapi)?;
    fs::write("openapi.json", &openapi_json).context("writing openapi.json")?;

    // FIXME: No longer needed once aide is upgraded in openapi-codegen
    let openapi: openapi_codegen::aide::openapi::OpenApi =
        serde_json::from_str(&openapi_json).context("deserializing OpenAPI")?;

    let webhooks = get_webhooks(&openapi);
    let paths = openapi.paths.context("spec must not be empty")?;
    let components = &mut openapi.components.unwrap_or_default();

    let api = Api::new(
        paths,
        components,
        &webhooks,
        IncludeMode::PublicAndInternal,
        &BTreeSet::from_iter([
            "v1.idempotency.start".to_owned(),
            "v1.idempotency.complete".to_owned(),
            "v1.idempotency.abandon".to_owned(),
        ]),
        &BTreeSet::new(),
    )?;

    for &(template, output_dir) in OUTPUTS {
        openapi_codegen::generate(&api, template.to_owned(), output_dir.into(), true)?;
    }

    std::process::Command::new("cargo")
        .args(["fmt", "--package=coyote-client"])
        .status()?;

    Ok(())
}

fn get_webhooks(spec: &openapi_codegen::aide::openapi::OpenApi) -> Vec<String> {
    let mut referenced_components = std::collections::BTreeSet::<String>::new();

    for (_, webhook) in &spec.webhooks {
        let Some(item) = webhook.as_item() else {
            continue;
        };

        for (_, op) in item.iter() {
            if let Some(body) = &op.request_body
                && let Some(item) = body.as_item()
                && let Some(json_content) = item.content.get("application/json")
                && let Some(schema) = &json_content.schema
                && let Schema::Object(obj) = &schema.json_schema
                && let Some(reference) = &obj.reference
                && let Some(component_name) = reference.split('/').next_back()
            {
                referenced_components.insert(component_name.to_owned());
            }
        }
    }

    referenced_components.into_iter().collect::<Vec<String>>()
}

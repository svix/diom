use std::{collections::BTreeSet, fs, path::Path, process::ExitCode};

use anyhow::Context as _;
use openapi_codegen::{IncludeMode, api::Api, schemars::schema::Schema};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod go;
mod java;
mod javascript;
mod python;
mod rust;
#[macro_use]
mod utils;

fn main() -> anyhow::Result<ExitCode> {
    anyhow::ensure!(
        cfg!(feature = "generate"),
        "must enable --feature=generate to generate code"
    );

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    std::env::set_current_dir(repo_root())?;

    let mut openapi = diom::openapi::initialize_openapi();

    let router = diom::v1::router();
    _ = aide::axum::ApiRouter::new()
        .nest("/api/v1", router)
        .finish_api_with(&mut openapi, diom::openapi::add_security_scheme);

    diom::openapi::postprocess_spec(&mut openapi);

    let openapi_json = serde_json::to_string_pretty(&openapi)? + "\n";

    // Only write openapi.json (and thus update its modified-at timestamp) if there are any changes.
    // Avoids unnecessary rebuilds of the diom crate which uses `include_str!("openapi.json")`.
    let old_openapi_json = fs::read_to_string("openapi.json").unwrap_or_else(|_| String::new());
    if openapi_json != old_openapi_json {
        fs::write("openapi.json", &openapi_json).context("writing openapi.json")?;
    }

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

    let results = async_io::block_on(future_zip!(
        go::generate(&api),
        java::generate(&api),
        javascript::generate(&api),
        python::generate(&api),
        rust::generate(&api),
    ));
    let mut exit_code = ExitCode::SUCCESS;
    for result in results {
        if let Err(e) = result {
            eprintln!("{e}\n");
            exit_code = ExitCode::FAILURE;
        }
    }

    Ok(exit_code)
}

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn get_webhooks(spec: &openapi_codegen::aide::openapi::OpenApi) -> Vec<String> {
    let mut referenced_components = BTreeSet::new();

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

    referenced_components.into_iter().collect()
}

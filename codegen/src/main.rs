use std::{collections::BTreeSet, fs, io, path::Path, process::ExitCode};

use anyhow::{Context as _, anyhow};
use openapi_codegen::{IncludeMode, api::Api, schemars::schema::Schema};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod formatting;
mod outputs;
mod utils;

use self::outputs::OUTPUTS;

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

    let mut openapi = coyote::openapi::initialize_openapi();

    let router = coyote::v1::router();
    _ = aide::axum::ApiRouter::new()
        .nest("/api/v1", router)
        .finish_api_with(&mut openapi, coyote::openapi::add_security_scheme);

    coyote::openapi::postprocess_spec(&mut openapi);

    let openapi_json = serde_json::to_string_pretty(&openapi)? + "\n";

    // Only write openapi.json (and thus update its modified-at timestamp) if there are any changes.
    // Avoids unnecessary rebuilds of the coyote crate which uses `include_str!("openapi.json")`.
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

    for output_dir in OUTPUTS {
        if output_dir.managed {
            let res = fs::remove_dir_all(output_dir.path);
            if let Err(e) = res
                && e.kind() != io::ErrorKind::NotFound
            {
                let context = format!("clearing managed directory `{}`", output_dir.path);
                return Err(anyhow!(e).context(context));
            }
        }

        for &template in output_dir.templates {
            let tpl_name = format!("codegen/templates/{template}");
            openapi_codegen::generate(&api, tpl_name, output_dir.path.into(), true)?;
        }
    }

    let exit_code = formatting::run();
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

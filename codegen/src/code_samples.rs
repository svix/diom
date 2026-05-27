use std::{collections::BTreeSet, path::Path};

use anyhow::Context as _;
use fs_err as fs;
use openapi_codegen::{CodegenLanguage, CodesampleTemplates, aide::openapi, generate_codesamples};

pub(crate) fn add_to_spec(openapi: &mut openapi::OpenApi) -> anyhow::Result<()> {
    let templates = load_templates()?;
    let code_samples = async_io::block_on(generate_codesamples(
        &*openapi,
        templates,
        BTreeSet::new(),
        core::convert::identity,
    ))
    .context("generating code samples")?;

    for operation in openapi
        .paths
        .as_mut()
        .iter_mut()
        .flat_map(|p| p.paths.values_mut())
        .filter_map(|r| r.as_item_mut())
        .flat_map(path_item_operations_mut)
    {
        let operation_id = operation
            .operation_id
            .as_ref()
            .context("all operations must have an ID")?;

        let Some(samples) = code_samples.get(operation_id) else {
            continue;
        };
        let indexmap::map::Entry::Vacant(v) =
            operation.extensions.entry("x-codeSamples".to_owned())
        else {
            tracing::warn!(
                operation_id,
                "original spec already contains code samples for this operation"
            );
            continue;
        };

        v.insert(serde_json::to_value(samples).unwrap());
    }

    Ok(())
}

fn load_templates() -> anyhow::Result<CodesampleTemplates> {
    const TEMPLATES: &[(&str, &str, CodegenLanguage)] = &[
        ("cli", "CLI", CodegenLanguage::Shell),
        ("go", "Go", CodegenLanguage::Go),
        ("java", "Java", CodegenLanguage::Java),
        ("javascript", "JavaScript", CodegenLanguage::TypeScript),
        ("python", "Python", CodegenLanguage::Python),
        ("rust", "Rust", CodegenLanguage::Rust),
    ];

    let mut result = CodesampleTemplates::default();
    for &(dir, label, lang) in TEMPLATES {
        let filename = format!("api_call_sample.{}.jinja", lang.ext());
        let source = fs::read_to_string(
            Path::new("codegen")
                .join("templates")
                .join(dir)
                .join(filename),
        )?;
        result.add_template(lang, label, source);
    }

    Ok(result)
}

fn path_item_operations_mut(
    inner: &mut openapi::PathItem,
) -> impl Iterator<Item = &mut openapi::Operation> {
    [
        &mut inner.get,
        &mut inner.put,
        &mut inner.post,
        &mut inner.delete,
        &mut inner.options,
        &mut inner.head,
        &mut inner.patch,
        &mut inner.trace,
    ]
    .into_iter()
    .flatten()
}

use aide::openapi::{self, OpenApi, Parameter, ReferenceOr};
use schemars::JsonSchema;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize_openapi() -> OpenApi {
    aide::generate::on_error(|error| {
        tracing::error!("Aide generation error: {error}");
    });
    // Extract schemas to `#/components/schemas/` instead of using inline schemas.
    aide::generate::extract_schemas(true);
    // Have aide attempt to infer the `Content-Type` of responses based on the
    // handlers' return types.
    aide::generate::infer_responses(true);
    aide::generate::inferred_empty_response_status(204);

    aide::generate::in_context(|ctx| {
        ctx.schema = schemars::generate::SchemaSettings::openapi3().into()
    });

    let _tag_groups = serde_json::json!([
        {
            "name": "General",
            "tags": ["Application", "Event Type"]
        },
        {
            "name": "Application specific",
            "tags": ["Authentication", "Endpoint", "Message", "Message Attempt", "Integration"]
        },
        {
            "name": "Utility",
            "tags": ["Health"]
        },
        {
            "name": "Webhooks",
            "tags": ["Webhooks"]
        }
    ]);

    const DESCRIPTION: &str = include_str!("../description.md");

    OpenApi {
        info: openapi::Info {
            title: "Svix API".to_owned(),
            version: VERSION.to_owned(),
            extensions: indexmap::indexmap! {
                "x-logo".to_string() => serde_json::json!({
                    "url": "https://www.svix.com/static/img/brand-padded.svg",
                    "altText": "Svix Logo",
                }),
            },
            description: Some(DESCRIPTION.to_string()),
            ..Default::default()
        },
        tags: vec![
            openapi::Tag {
                name: "Application".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Message".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Message Attempt".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Endpoint".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Integration".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Event Type".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Authentication".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Health".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Webhooks".to_owned(),
                ..openapi::Tag::default()
            },
        ],
        // FIXME: add me back
        // extensions: indexmap::indexmap! {
        //     "x-tagGroups".to_owned() => tag_groups,
        // },
        ..Default::default()
    }
}

/// Adds the `Idempotency-Key` header parameter to all `POST` operations in the schema.
fn add_idempotency_to_post(openapi: &mut OpenApi) {
    // The header's value can be any valid string
    let string_schema = aide::generate::in_context(|ctx| String::json_schema(&mut ctx.schema));

    let s = openapi::SchemaObject {
        json_schema: string_schema,
        external_docs: None,
        example: None,
    };

    let idempotency_key_data = openapi::ParameterData {
        name: "idempotency-key".to_string(),
        description: Some("The request's idempotency key".to_string()),
        required: false,
        deprecated: None,
        format: openapi::ParameterSchemaOrContent::Schema(s),
        example: None,
        examples: indexmap::indexmap! {},
        explode: None,
        extensions: indexmap::indexmap! {},
    };

    if let Some(paths) = &mut openapi.paths {
        for (_, op) in &mut paths.paths {
            match op {
                openapi::ReferenceOr::Reference { reference, .. } => {
                    // References to operations should never appear in our
                    // schema since all our operations are unique, and we
                    // don't reference any 3rd party/external operations.
                    tracing::warn!(
                        "Unexpected operation reference encountered in OpenAPI schema: {reference}"
                    );
                }
                openapi::ReferenceOr::Item(op) => {
                    if let Some(post) = &mut op.post {
                        post.parameters.push(ReferenceOr::Item(Parameter::Header {
                            parameter_data: idempotency_key_data.clone(),
                            style: openapi::HeaderStyle::Simple,
                        }));
                    }
                }
            }
        }
    }
}

/// Remove schemas from `components.schemas` of the spec which are under normal
/// circumstances not referenced. At the moment these are struct schemas used
/// by query parameters and path placeholders.
fn remove_unneeded_schemas(openapi: &mut OpenApi) {
    let Some(components) = &mut openapi.components else {
        return;
    };

    components.schemas.retain(|name, _| {
        !(name.ends_with("Path") || name.ends_with("QueryParams") || name.starts_with("Pagination"))
    });
}

fn flatten_weird_nested_ref(openapi: &mut OpenApi) {
    let Some(components) = &mut openapi.components else {
        return;
    };

    for schema in components.schemas.values_mut() {
        let Some(schema) = schema.json_schema.as_object_mut() else {
            continue;
        };
        let Some(props) = schema.get_mut("properties") else {
            continue;
        };
        let Some(props) = props.as_object_mut() else {
            continue;
        };

        for prop in props.values_mut() {
            if let Some(schema) = prop.as_object_mut()
                && !schema.contains_key("$ref")
                && let Some(all_of) = schema.get_mut("allOf")
                && let Some(all_of) = all_of.as_array_mut()
                && all_of.len() == 1
                && let Some(sub) = all_of[0].as_object_mut()
                && sub.len() == 1
                && let Some((k, v)) = sub.remove_entry("$ref")
            {
                schema.insert(k, v);
                schema.remove("allOf");
            }
        }
    }
}

pub fn add_security_scheme(
    api: aide::transform::TransformOpenApi<'_>,
) -> aide::transform::TransformOpenApi<'_> {
    api.security_scheme(
        "HTTPBearer",
        aide::openapi::SecurityScheme::Http {
            scheme: "bearer".to_string(),
            bearer_format: None,
            description: Some("HTTP Bearer token passed in the `Authorization` header".into()),
            extensions: Default::default(),
        },
    )
}

/// Applies a list of hacks to the finished OpenAPI spec to make it usable with
/// our tooling.
pub fn postprocess_spec(openapi: &mut OpenApi) {
    let hacks = [
        add_idempotency_to_post,
        flatten_weird_nested_ref,
        remove_unneeded_schemas,
    ];

    for hack in hacks {
        hack(openapi);
    }
}

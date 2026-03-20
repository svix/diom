use std::mem;

use aide::openapi::{self, OpenApi};
use serde_json::json;

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
            title: "Diom API".to_owned(),
            version: VERSION.to_owned(),
            // FIXME: diom branding
            extensions: indexmap::indexmap! {
                "x-logo".to_string() => serde_json::json!({
                    "altText": "Svix Logo",
                    "url": "https://www.svix.com/static/img/brand-padded.svg",
                }),
            },
            description: Some(DESCRIPTION.to_string()),
            ..Default::default()
        },
        tags: vec![
            openapi::Tag {
                name: "Cache".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Idempotency".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Key Value Store".to_owned(),
                ..openapi::Tag::default()
            },
            // FIXME: Add back when we have routes
            // openapi::Tag {
            //     name: "Queue".to_owned(),
            //     ..openapi::Tag::default()
            // },
            openapi::Tag {
                name: "Rate Limiter".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Stream".to_owned(),
                ..openapi::Tag::default()
            },
            openapi::Tag {
                name: "Health".to_owned(),
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

/// Sorts components.schemas by name.
fn sort_schemas_by_name(openapi: &mut OpenApi) {
    if let Some(components) = &mut openapi.components {
        components.schemas.sort_unstable_keys();
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

/// Turn anyOf { { whatever }, { nullable: true, enum: [null] } } into { whatever, nullable: true }
fn fix_stupid_nullable_repr(openapi: &mut OpenApi) {
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
                && let Some(any_of) = schema.get_mut("anyOf")
                && let Some(any_of) = any_of.as_array_mut()
                && any_of.len() == 2
                && any_of[1] == json!({ "nullable": true, "enum": [null] })
                && let Some(main_schema) = any_of[0].as_object_mut()
            {
                main_schema.insert("nullable".to_owned(), true.into());
                *schema = mem::take(main_schema);
            }
        }
    }
}

pub fn add_security_scheme(
    api: aide::transform::TransformOpenApi<'_>,
) -> aide::transform::TransformOpenApi<'_> {
    api.security_scheme(
        "HTTPBearer",
        openapi::SecurityScheme::Http {
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
        sort_schemas_by_name,
        flatten_weird_nested_ref,
        fix_stupid_nullable_repr,
        remove_unneeded_schemas,
    ];

    for hack in hacks {
        hack(openapi);
    }
}

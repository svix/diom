use aide::openapi::{self, OpenApi, Parameter, ReferenceOr};
use indexmap::IndexMap;
use schemars::JsonSchema;
use std::mem;

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

#[tracing::instrument(skip_all)]
pub fn add_one_of_discriminator(openapi: &mut OpenApi) {
    let mut new_schemas = Vec::new();

    let Some(components) = &mut openapi.components else {
        return;
    };
    'schemas: for (schema_name, schema_object) in &mut components.schemas {
        let Some(schema) = schema_object.json_schema.as_object_mut() else {
            continue;
        };

        // Extract base properties and required before mutable borrow
        let base_props = schema
            .get("properties")
            .and_then(|v| v.as_object())
            .cloned();
        let base_required = schema.get("required").and_then(|v| v.as_array()).cloned();

        let Some(one_of) = schema.get_mut("oneOf").and_then(|o| o.as_array_mut()) else {
            continue;
        };

        let mut discriminator = None;
        let mut mapping = IndexMap::new();

        for variant in one_of {
            let Some(v_obj) = variant.as_object_mut() else {
                continue;
            };

            let variant_discr = v_obj
                .get_mut("properties")
                .and_then(|p| p.as_object_mut())
                .and_then(|p| {
                    p.iter().find_map(|(prop_name, prop)| {
                        let schema_object = prop.as_object()?;
                        if let Some(const_val) = schema_object.get("const") {
                            return Some((prop_name, Some(const_val)));
                        };

                        let enum_val = schema_object.get("enum")?;
                        let enum_val = enum_val.as_array()?;
                        if enum_val.len() != 1 {
                            return None;
                        };

                        Some((prop_name, Some(&enum_val[0])))
                    })
                });

            let Some((discr_name, variant_name)) = variant_discr else {
                tracing::warn!("discriminator not found");
                continue 'schemas;
            };

            if let Some(d) = &discriminator {
                if discr_name != d {
                    tracing::warn!("ambiguity error");
                    continue 'schemas;
                }
            } else {
                discriminator = Some(discr_name.clone());
            }

            let Some(variant_name) = variant_name.and_then(|v| v.as_str()) else {
                tracing::warn!("non-string discriminator value");
                continue 'schemas;
            };

            let variant_schema_name = format!("{schema_name}_{variant_name}");
            let reference = format!("#/components/schemas/{variant_schema_name}");

            // Add mapping so redoc renders the right discriminator value
            mapping.insert(variant_name.to_owned(), reference.clone());

            // Copy all the non-variant-specific fields and `required` entries
            // onto the variant.
            //
            // It seems like ReDoc ignores regular `properties` when `mapping`
            // exists, instead expecting every mapped `$ref` to exhaustively
            // define all the fields itself.

            // Merge base properties into variant properties
            if let Some(base_props) = &base_props
                && let Some(properties) =
                    v_obj.get_mut("properties").and_then(|p| p.as_object_mut())
            {
                properties.extend(base_props.iter().map(|(k, v)| (k.clone(), v.clone())));
            }

            // Merge base required fields into variant required fields
            if let Some(base_required) = &base_required {
                let variant_required = v_obj
                    .entry("required".to_owned())
                    .or_insert_with(|| serde_json::Value::Array(Vec::new()))
                    .as_array_mut();

                if let Some(variant_required) = variant_required {
                    let new_fields: Vec<_> = base_required
                        .iter()
                        .filter(|field| !variant_required.contains(field))
                        .cloned()
                        .collect();
                    variant_required.extend(new_fields);
                }
            }

            // Move every oneOf member to its own named component,
            // so `discriminator.mapping` (set above) resolved correctly.
            let variant_schema = mem::replace(variant, serde_json::json!({ "$ref": reference }));
            new_schemas.push((
                variant_schema_name,
                openapi::SchemaObject {
                    json_schema: serde_json::from_value(variant_schema).unwrap(),
                    external_docs: None,
                    example: None,
                },
            ));
        }

        let Some(discriminator) = discriminator else {
            tracing::warn!("discriminator not found");
            continue;
        };
        schema.insert(
            "discriminator".to_owned(),
            serde_json::json!({ "propertyName": discriminator, "mapping": mapping }),
        );
    }

    components.schemas.extend(new_schemas);
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
        add_one_of_discriminator,
        remove_unneeded_schemas,
    ];

    for hack in hacks {
        hack(openapi);
    }
}

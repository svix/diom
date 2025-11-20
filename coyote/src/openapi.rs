use aide::openapi::{self, OpenApi, Parameter, ReferenceOr};
use schemars::{visit::Visitor, JsonSchema};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize_openapi() -> OpenApi {
    aide::gen::on_error(|error| {
        tracing::error!("Aide generation error: {error}");
    });
    // Extract schemas to `#/components/schemas/` instead of using inline schemas.
    aide::gen::extract_schemas(true);
    // Have aide attempt to infer the `Content-Type` of responses based on the
    // handlers' return types.
    aide::gen::infer_responses(true);
    aide::gen::inferred_empty_response_status(204);

    aide::gen::in_context(|ctx| ctx.schema = schemars::gen::SchemaSettings::openapi3().into());

    let tag_groups = serde_json::json![[
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
    ]];

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
        extensions: indexmap::indexmap! {
            "x-tagGroups".to_owned() => tag_groups,
        },
        ..Default::default()
    }
}

/// Replaces OpenAPI 3.1 style `"foo": true` schemas with OpenAPI 3.0 style
/// `"foo": {"type": "object"}` schemas.
fn replace_true_schemas(openapi: &mut OpenApi) {
    use schemars::schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec};

    // Checks if it's a plain boolean schema, and if yes replaces it with a
    // `{"type": "object"}` schema. If it's an object then it will descend into
    // its properties and `additionalProperties` too.
    fn visit_schema(schema: &mut Schema) {
        match schema {
            Schema::Bool(true) => {
                *schema = Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
                    ..Default::default()
                })
            }
            Schema::Bool(false) => {
                tracing::warn!("unexpected `false` schema encountered");
            }
            Schema::Object(obj) => {
                // When examples are added to a `"foo": bool` schema it gets
                // expanded into an object, i.e. `"foo": {"example": ...}`, but
                // no "type" field is set on it. Although the OpenAPI spec does
                // not specifically say that the type field is mandatory, in
                // practice a lack of the "type" field should only ever occur
                // when a `true` schema gets replaced because it is somehow
                // modified (e.g. example added), or because it's a "$ref"
                // object.
                // If it's not a reference, then we must add the "type" field
                // back with the value "object" so code generators work correctly.
                if obj.instance_type.is_none() && obj.reference.is_none() {
                    obj.instance_type = Some(SingleOrVec::Single(Box::new(InstanceType::Object)));
                }

                obj.object.as_mut().map(visit_object_validation);
            }
        }
    }

    fn visit_object_validation(obj: &mut Box<ObjectValidation>) {
        if let Some(additional_props) = &mut obj.additional_properties {
            visit_schema(additional_props)
        }
        for (_, schema) in &mut obj.properties {
            visit_schema(schema)
        }
    }

    if let Some(components) = &mut openapi.components {
        for (_, schema) in &mut components.schemas {
            visit_schema(&mut schema.json_schema)
        }
    }
}

/// Adds the `Idempotency-Key` header parameter to all `POST` operations in the schema.
fn add_idempotency_to_post(openapi: &mut OpenApi) {
    // The header's value can be any valid string
    let string_schema = aide::gen::in_context(|ctx| String::json_schema(&mut ctx.schema));

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
    if let Some(components) = &mut openapi.components {
        components.schemas.retain(|name, _| {
            !(name.ends_with("Path")
                || name.ends_with("QueryParams")
                || name.starts_with("Pagination"))
        });
    }
}

/// Replaces the `examples` property of a schema with a singular `example`
/// property.
/// OpenAPI <=3.0 used `example` as an extension, >=3.1 standardized `examples`.
fn replace_multiple_examples(openapi: &mut OpenApi) {
    let mut visitor = schemars::visit::SetSingleExample {
        retain_examples: false,
    };

    if let Some(components) = &mut openapi.components {
        for (_, schema_object) in &mut components.schemas {
            visitor.visit_schema(&mut schema_object.json_schema);
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
        remove_unneeded_schemas,
        replace_true_schemas,
        replace_multiple_examples,
    ];

    for hack in hacks {
        hack(openapi);
    }
}

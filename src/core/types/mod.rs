// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{ops::Deref, sync::LazyLock};

use diom_error::validation_error;
use regex::Regex;
#[allow(unused_imports)]
use schemars::JsonSchema;
use schemars::{Schema, json_schema};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

const ALL_ERROR: &str = "__all__";

#[allow(unused_macros)]
macro_rules! enum_wrapper {
    ($name_id:ty) => {
        impl Serialize for $name_id {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i16((*self).into())
            }
        }

        impl<'de> Deserialize<'de> for $name_id {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;
                i16::deserialize(deserializer)?.try_into().map_err(|_| {
                    Error::custom(format!("Failed deserializing {}", stringify!($name_id)))
                })
            }
        }
    };
}

fn validate_limited_str(s: &str) -> Result<(), ValidationErrors> {
    const MAX_LENGTH: usize = 256;
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_.:]+$").unwrap());
    let mut errors = ValidationErrors::new();
    if s.is_empty() {
        errors.add(
            ALL_ERROR,
            validation_error(
                Some("length"),
                Some("String must be at least one character"),
            ),
        );
    } else if s.len() > MAX_LENGTH {
        errors.add(
            ALL_ERROR,
            validation_error(Some("length"), Some("String too long")),
        );
    } else if !RE.is_match(s) {
        errors.add(
            ALL_ERROR,
            validation_error(
                Some("illegal_string_pattern"),
                Some("String must match the following pattern: [a-zA-Z0-9\\-_.:]."),
            ),
        );
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub trait BaseUid: Deref<Target = String> {
    const ID_PREFIX: &'static str;

    fn validate_(&self) -> Result<(), ValidationErrors> {
        let mut errors = match validate_limited_str(self) {
            Ok(_) => ValidationErrors::new(),
            Err(x) => x,
        };
        if self.starts_with(Self::ID_PREFIX) {
            errors.add(
                ALL_ERROR,
                validation_error(
                    Some("invalid_uid_prefix"),
                    Some("Uids are not allowed to have the same prefix as the ID. Prefix with _?"),
                ),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

macro_rules! string_wrapper {
    ($name_id:ident) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $name_id(pub String);

        string_wrapper_impl!($name_id);
    };
    ($name_id:ident, $string_schema:expr) => {
        string_wrapper!($name_id);

        common_jsonschema_impl!($name_id, $string_schema);
    };
}

macro_rules! string_wrapper_impl {
    ($name_id:ident) => {
        impl Deref for $name_id {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name_id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<String> for $name_id {
            fn from(s: String) -> Self {
                $name_id(s)
            }
        }
    };
}

/// A container type for storing schema information commonly used by string
/// wrapper types.
#[derive(Default)]
pub struct StringSchema {
    pub string_validation: Option<Schema>,
    pub example: Option<String>,
}

impl StringSchema {
    pub fn schema_for_ids(prefix: &'static str) -> Self {
        Self {
            string_validation: None,
            example: Some(format!("{prefix}1srOrx2ZWZBpBUvZwXKQmoEYga2")),
        }
    }

    pub fn schema_for_uids(prefix: &'static str) -> Self {
        Self {
            string_validation: Some(json_schema!({
                "minLength": 1,
                "maxLength": 256,
                "pattern": r"^[a-zA-Z0-9\-_.]+$",
            })),
            example: Some(format!("unique-{prefix}identifier").replace('_', "-")),
        }
    }
}

/// Macro to generate a [`JsonSchema`] impl for string wrapper types.
/// * `name_id` is the name of the identifier for which the impl is generated.
/// * `string_schema` is a [`StringSchema`] to enrich the generated schema with
///   more information.
macro_rules! common_jsonschema_impl {
    ($name_id:ident, $string_schema:expr) => {
        impl ::schemars::JsonSchema for $name_id {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                stringify!($name_id).into()
            }

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                let mut schema = String::json_schema(generator);

                if let Some(obj) = schema.as_object_mut() {
                    // This is just to help with type hints when the macro is expanded.
                    let options: $crate::core::types::StringSchema = $string_schema;

                    if let Some(mut v) = options.string_validation {
                        obj.extend(::std::mem::take(v.ensure_object()));
                    }

                    if let Some(example) = options.example {
                        obj.insert("example".to_owned(), serde_json::Value::String(example));
                    }
                }

                schema
            }

            fn inline_schema() -> bool {
                true
            }
        }
    };
}

/// A macro to which you pass the list of variants of an enum using `repr(N)`
/// and it returns a `Vec<(N, String)>`, where each element is `(value, "VariantStringified")`
#[allow(unused_macros)]
macro_rules! repr_enum {
    ($($variant:ident),+) => {
        vec![
            $(($variant.into(), stringify!($variant).to_string())),+
        ]
    }
}

/// Generates a `JsonSchema` implementation for an enum using `repr(N)`. The
/// enum must also derive `IntoPrimitive`.
///
/// Arguments are:
/// 1. Name of the enum type, `Foo`
/// 2. The repr type used, e.g. in case of `repr(i16)` it must be `i16`
/// 3. The string description to be used in the docs.
///
/// Remaining arguments must be the variants in order. For example:
///
/// ```ignore
/// #[derive(IntoPrimitive)]
/// #[repr(u8)]
/// enum MyEnum {
///     Foo = 0,
///     Bar = 1,
///     Qux = 5,
/// }
///
/// jsonschema_for_repr_enum! {
///     MyEnum,
///     u8,
///     "My nice little enum",
///     Foo, Bar, Qux
/// }
/// ```
#[allow(unused_macros)]
macro_rules! jsonschema_for_repr_enum {
    ($tyname:ty, $repr_ty:ty, $descr:expr, $($variant:ident),+) => {
        impl JsonSchema for $tyname {
            fn schema_name() -> Cow<'static, str> {
                stringify!($tyname).to_string()
            }

            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                use schemars::schema::{InstanceType, Metadata, Schema, SchemaObject, SingleOrVec};
                use $tyname::*;

                // A list of variant values and their corresponding name.
                let variants: Vec<($repr_ty, String)> = repr_enum!($($variant),+);
                // The list of possible enum primitive values.
                let values = variants.iter().map(|(value, _)| serde_json::json!(value)).collect();
                // The list of nice variant names the above values correspond to.
                let variant_names = variants.iter().map(|(_, name)| serde_json::Value::String(name.clone())).collect();

                Schema::Object(SchemaObject{
                    metadata: Some(Box::new(Metadata {
                        title: Some(Self::schema_name()),
                        description: Some($descr.to_string()),
                        ..Default::default()
                    })),
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Integer))),
                    enum_values: Some(values),
                    extensions: FromIterator::from_iter([
                        ("x-enum-varnames".to_string(), serde_json::Value::Array(variant_names)),
                    ]),
                    ..Default::default()
                })
            }
        }
    }
}

string_wrapper!(
    EntityKey,
    StringSchema {
        string_validation: Some(json_schema!({
            "maxLength": 256,
            "pattern": r"^[a-zA-Z0-9\-_.:]+$",
        })),
        example: Some("some_key".to_string()),
    }
);

impl Validate for EntityKey {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_limited_str(&self.0)
    }
}

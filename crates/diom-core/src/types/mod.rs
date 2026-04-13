use std::{ops::Deref, sync::LazyLock};

use regex::Regex;
#[allow(unused_imports)]
use schemars::JsonSchema;
use schemars::{Schema, json_schema};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::validation::validation_error;

mod byte_string;
mod duration_ms;
mod metadata;

pub use self::{byte_string::ByteString, duration_ms::DurationMs, metadata::Metadata};

const ALL_ERROR: &str = "__all__";

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

        impl AsRef<str> for $name_id {
            fn as_ref(&self) -> &str {
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
                    let options: $crate::types::StringSchema = $string_schema;

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

string_wrapper!(
    EntityKey,
    StringSchema {
        string_validation: Some(json_schema!({
            "maxLength": 256,
            "pattern": r"^[a-zA-Z0-9\-/_.=+:]+$",
        })),
        example: Some("some_key".to_string()),
    }
);

impl Validate for EntityKey {
    fn validate(&self) -> Result<(), ValidationErrors> {
        const MAX_LENGTH: usize = 256;
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-/_.=+:]+$").unwrap());
        let mut errors = ValidationErrors::new();
        if self.0.is_empty() {
            errors.add(
                ALL_ERROR,
                validation_error(
                    Some("length"),
                    Some("String must be at least one character"),
                ),
            );
        } else if self.0.len() > MAX_LENGTH {
            errors.add(
                ALL_ERROR,
                validation_error(Some("length"), Some("String too long")),
            );
        } else if !RE.is_match(&self.0) {
            errors.add(
                ALL_ERROR,
                validation_error(
                    Some("invalid_entity_key"),
                    Some(r"Entity key must match the following pattern: ^[a-zA-Z0-9\-/_.=+:]+$."),
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

/// Consistency level for reads.
///
/// Strong consistency (also known as linearizability) guarantees that a read will see all previous
/// writes. Weak consistency allows stale reads, but can save one or more round trip to the leader.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum Consistency {
    Strong,
    Weak,
}

impl Consistency {
    pub fn linearizable(&self) -> bool {
        matches!(self, Self::Strong)
    }

    pub fn strong() -> Self {
        Self::Strong
    }

    pub fn weak() -> Self {
        Self::Weak
    }
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::EntityKey;

    fn allowed(s: &str) {
        assert!(
            EntityKey(s.to_string()).validate().is_ok(),
            "{s:?} should be allowed"
        );
    }

    fn rejected(s: &str) {
        assert!(
            EntityKey(s.to_string()).validate().is_err(),
            "{s:?} should be rejected"
        );
    }

    #[test]
    fn test_allowed_entity_keys() {
        allowed("foo");
        allowed("foo/bar");
        allowed("foo/bar/baz");
        allowed("foo:bar");
        allowed("foo:bar/baz");
        allowed("foo/bar:baz");
        allowed(":baz");
        allowed("foo:");
        allowed("foo:bar:baz");
        allowed("1");
        allowed("foo/+==");
    }

    #[test]
    fn test_rejected_entity_keys() {
        rejected("");
        rejected("foo bar");
        rejected("foo&bar");
        rejected("foo\\bar");
    }
}

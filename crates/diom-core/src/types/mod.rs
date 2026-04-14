use std::{ops::Deref, sync::LazyLock};

use regex::Regex;
#[allow(unused_imports)]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

use crate::validation::validation_error;

mod byte_string;
mod duration_ms;
mod metadata;
mod unix_timestamp_ms;

pub use self::{
    byte_string::ByteString, duration_ms::DurationMs, metadata::Metadata,
    unix_timestamp_ms::UnixTimestampMs,
};

pub const ALL_ERROR: &str = "__all__";

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

#[macro_export]
macro_rules! string_wrapper {
    ($name_id:ident { $($init:tt)* }) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $name_id(pub String);

        impl $crate::types::StringWrapper for $name_id {
            const INFO: $crate::types::StringSchema = $crate::types::StringSchema {
                $($init)*
            };
        }

        impl std::ops::Deref for $name_id {
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
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl From<String> for $name_id {
            fn from(s: String) -> Self {
                $name_id(s)
            }
        }

        impl $crate::__reexport::schemars::JsonSchema for $name_id {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($name_id).into()
            }

            fn json_schema(
                _g: &mut $crate::__reexport::schemars::SchemaGenerator,
            ) -> $crate::__reexport::schemars::Schema {
                let info = <Self as $crate::types::StringWrapper>::INFO;
                $crate::__reexport::schemars::json_schema!({
                    "type": "string",
                    "minLength": info.min_length,
                    "maxLength": info.max_length,
                    "pattern": info.pattern,
                    "example": info.example,
                })
            }

            fn inline_schema() -> bool {
                true
            }
        }

        impl $crate::__reexport::validator::Validate for $name_id {
            fn validate(&self) -> Result<(), $crate::__reexport::validator::ValidationErrors> {
                use std::sync::LazyLock;
                use $crate::__reexport::regex::Regex;

                static RE: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(<$name_id as $crate::types::StringWrapper>::INFO.pattern).unwrap()
                });

                let info = <Self as $crate::types::StringWrapper>::INFO;
                let mut errors = $crate::__reexport::validator::ValidationErrors::new();
                if self.0.len() < info.min_length {
                    errors.add(
                        $crate::types::ALL_ERROR,
                        $crate::validation::validation_error(Some("length"), Some("String too short")),
                    );
                } else if self.0.len() > info.max_length {
                    errors.add(
                        $crate::types::ALL_ERROR,
                        $crate::validation::validation_error(Some("length"), Some("String too long")),
                    );
                } else if !RE.is_match(&self.0) {
                    errors.add(
                        $crate::types::ALL_ERROR,
                        $crate::validation::validation_error(
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
    };
}

#[doc(hidden)]
pub struct StringSchema {
    pub min_length: usize,
    pub max_length: usize,
    pub pattern: &'static str,
    pub example: &'static str,
}

#[doc(hidden)]
pub trait StringWrapper {
    const INFO: StringSchema;
}

string_wrapper!(EntityKey {
    min_length: 1,
    max_length: 256,
    pattern: r"^[a-zA-Z0-9\-/_.=+:]+$",
    example: "some_key"
});

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

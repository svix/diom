#[allow(unused_imports)]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod byte_string;
mod duration_ms;
mod metadata;
mod unix_timestamp_ms;

pub use self::{
    byte_string::ByteString, duration_ms::DurationMs, metadata::Metadata,
    unix_timestamp_ms::UnixTimestampMs,
};

pub const ALL_ERROR: &str = "__all__";

/// Trait representing timestamps that can be represented as a number of milliseconds since the unix
/// epoch
pub trait AsMillisecond {
    /// Get this value as the number of milliseconds since the unix epoch
    fn as_millisecond(&self) -> u64;
}

impl AsMillisecond for jiff::Timestamp {
    /// Get this jiff::Timestamp as the number of milliseconds since the unix epoch
    fn as_millisecond(&self) -> u64 {
        jiff::Timestamp::as_millisecond(*self) as _
    }
}

impl AsMillisecond for UnixTimestampMs {
    /// Get this UnixTimestampMs as the number of milliseconds since the unix epoch
    fn as_millisecond(&self) -> u64 {
        UnixTimestampMs::as_millisecond(*self)
    }
}

#[macro_export]
macro_rules! string_wrapper {
    ($name_id:ident { $($init:tt)* }) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
        #[serde(transparent)]
        pub struct $name_id(pub String);

        impl $crate::PersistableValue for $name_id {}

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

        impl<'de> $crate::__reexport::serde::Deserialize<'de> for $name_id {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use std::sync::LazyLock;
                use $crate::__reexport::{
                    regex::Regex,
                    serde::de::Error as DeError,
                };

                static RE: LazyLock<Regex> = LazyLock::new(|| {
                    Regex::new(<$name_id as $crate::types::StringWrapper>::INFO.pattern).unwrap()
                });

                let value = String::deserialize(deserializer)?;

                let info = <Self as $crate::types::StringWrapper>::INFO;
                if value.len() < info.min_length {
                    return Err(DeError::custom("String too short"));
                }
                if value.len() > info.max_length {
                    return Err(DeError::custom("String too long"));
                }
                if !RE.is_match(&value) {
                    return Err(DeError::custom(::std::concat!(
                        stringify!($name_id),
                        r" must match the following pattern: ^[a-zA-Z0-9\-/_.=+:]+$.",
                    )));
                }

                Ok(Self(value))
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
    use serde::{Deserialize as _, de::value::StringDeserializer};

    use super::EntityKey;

    fn string_de(s: &str) -> StringDeserializer<serde::de::value::Error> {
        StringDeserializer::new(s.to_owned())
    }

    fn allowed(s: &str) {
        assert!(
            EntityKey::deserialize(string_de(s)).is_ok(),
            "{s:?} should be allowed"
        );
    }

    fn rejected(s: &str) {
        assert!(
            EntityKey::deserialize(string_de(s)).is_err(),
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

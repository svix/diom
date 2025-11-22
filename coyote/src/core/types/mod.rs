// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{ops::Deref, sync::LazyLock};

use chrono::{DateTime, Utc};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use svix_ksuid::*;
use validator::{Validate, ValidationErrors};

use crate::v1::utils::validation_error;

const ALL_ERROR: &str = "__all__";

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

pub trait BaseId: Deref<Target = String> {
    const PREFIX: &'static str;
    type Output;

    fn validate_(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if !&self.starts_with(Self::PREFIX) {
            errors.add(
                ALL_ERROR,
                validation_error(Some("id"), Some("Invalid id. Expected different prefix")),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn generate_(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> String {
        let ksuid = KsuidMs::new(dt, payload);
        format!("{}{}", Self::PREFIX, ksuid.to_string())
    }

    fn new(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self::Output;

    fn start_id(start: DateTime<Utc>) -> Self::Output {
        let buf = [0u8; KsuidMs::PAYLOAD_BYTES];
        Self::new(Some(start), Some(&buf[..]))
    }

    fn end_id(start: DateTime<Utc>) -> Self::Output {
        let buf = [0xFFu8; KsuidMs::PAYLOAD_BYTES];
        Self::new(Some(start), Some(&buf[..]))
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.ksuid().timestamp()
    }

    fn ksuid(&self) -> svix_ksuid::KsuidMs {
        let ksuid_str = self
            .strip_prefix(Self::PREFIX)
            .expect("ID has invalid prefix");
        <svix_ksuid::KsuidMs as svix_ksuid::KsuidLike>::from_base62(ksuid_str)
            .expect("ID was not encoded as valid ksuid")
    }
}

fn validate_limited_str(s: &str) -> Result<(), ValidationErrors> {
    const MAX_LENGTH: usize = 256;
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_.]+$").unwrap());
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
                Some("String must match the following pattern: [a-zA-Z0-9\\-_.]."),
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
    pub string_validation: Option<schemars::schema::StringValidation>,
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
            string_validation: Some(schemars::schema::StringValidation {
                min_length: Some(1),
                max_length: Some(256),
                pattern: Some(r"^[a-zA-Z0-9\-_.]+$".to_string()),
            }),
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
            fn schema_name() -> String {
                stringify!($name_id).to_string()
            }

            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                let mut schema = String::json_schema(gen);

                if let schemars::schema::Schema::Object(obj) = &mut schema {
                    // This is just to help with type hints when the macro is expanded.
                    let options: $crate::core::types::StringSchema = $string_schema;

                    obj.string = options.string_validation.map(Box::new);
                    if let Some(example) = options.example {
                        obj.extensions
                            .insert("example".to_string(), serde_json::Value::String(example));
                    }
                }

                schema
            }

            fn is_referenceable() -> bool {
                false
            }
        }
    };
}

/// A macro to which you pass the list of variants of an enum using `repr(N)`
/// and it returns a `Vec<(N, String)>`, where each element is `(value, "VariantStringified")`
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
macro_rules! jsonschema_for_repr_enum {
    ($tyname:ty, $repr_ty:ty, $descr:expr, $($variant:ident),+) => {
        impl JsonSchema for $tyname {
            fn schema_name() -> String {
                stringify!($tyname).to_string()
            }

            fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
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
    crate::core::types::StringSchema {
        string_validation: Some(schemars::schema::StringValidation {
            max_length: Some(256),
            min_length: None,
            pattern: Some(r"^[a-zA-Z0-9\-_.]+$".to_string()),
        }),
        example: Some("some_key".to_string()),
    }
);

impl Validate for EntityKey {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_limited_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use base64::{engine::general_purpose::STANDARD, Engine};
    use validator::Validate;

    use super::{
        validate_header_map, ApplicationId, ApplicationUid, EndpointHeaders, EndpointHeadersPatch,
        EndpointSecret, EventChannel, EventTypeName,
    };
    use crate::core::cryptography::AsymmetricKey;

    #[test]
    fn test_id_validation() {
        let app_id = ApplicationId("app_24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        app_id.validate().unwrap();

        let app_id = ApplicationId("badprefix_24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        assert!(app_id.validate().is_err());

        let app_uid = ApplicationUid("app_24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        assert!(app_uid.validate().is_err());

        let app_uid = ApplicationUid("24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        app_uid.validate().unwrap();

        // With a space
        let app_uid = ApplicationUid("24NVKcPqNLXKu3 ".to_owned());
        assert!(app_uid.validate().is_err());

        // Check all allowed
        let app_uid = ApplicationUid("azAZ09-_.".to_owned());
        app_uid.validate().unwrap();

        // Check length
        let long_str: String = "X".repeat(300);
        let app_id = ApplicationId(long_str.clone());
        assert!(app_id.validate().is_err());
        let app_uid = ApplicationUid(long_str);
        assert!(app_uid.validate().is_err());

        let empty_str: String = "".to_owned();
        let app_id = ApplicationId(empty_str.clone());
        assert!(app_id.validate().is_err());
        let app_uid = ApplicationUid(empty_str);
        assert!(app_uid.validate().is_err());
    }

    #[test]
    fn test_event_names_validation() {
        // With a space
        let evt_name = EventTypeName("event ".to_owned());
        assert!(evt_name.validate().is_err());

        // Check all allowed
        let evt_name = EventTypeName("azAZ09-_.".to_owned());
        evt_name.validate().unwrap();

        // Check length
        let long_str: String = "X".repeat(300);
        let evt_name = EventTypeName(long_str);
        assert!(evt_name.validate().is_err());

        let empty_str = "".to_owned();
        let evt_name = EventTypeName(empty_str);
        assert!(evt_name.validate().is_err());
    }

    #[test]
    fn test_event_channel_validation() {
        // With a space
        let evt_name = EventChannel("event ".to_owned());
        assert!(evt_name.validate().is_err());

        // Check all allowed
        let evt_name = EventChannel("azAZ09-_.".to_owned());
        evt_name.validate().unwrap();

        // Check length
        let long_str: String = "X".repeat(300);
        let evt_name = EventChannel(long_str);
        assert!(evt_name.validate().is_err());
    }

    #[test]
    fn test_endpoint_headers_validation() {
        let hdr_map = HashMap::from([
            ("valid".to_owned(), "true".to_owned()),
            ("also-valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        validate_header_map(&endpoint_headers.0).unwrap();

        let hdr_map = HashMap::from([
            ("invalid?".to_owned(), "true".to_owned()),
            ("valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());

        let hdr_map = HashMap::from([
            ("invalid\0".to_owned(), "true".to_owned()),
            ("valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());

        let hdr_map = HashMap::from([("User-Agent".to_string(), "true".to_owned())]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());

        let hdr_map = HashMap::from([("X-Amz-".to_string(), "true".to_owned())]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());
    }

    #[test]
    fn test_endpoint_headers_patch_validation() {
        let hdr_map = HashMap::from([
            ("valid".to_owned(), Some("true".to_owned())),
            ("also-valid".to_owned(), Some("true".to_owned())),
        ]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        endpoint_headers.validate().unwrap();

        let hdr_map = HashMap::from([
            ("invalid?".to_owned(), Some("true".to_owned())),
            ("valid".to_owned(), Some("true".to_owned())),
        ]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([
            ("invalid\0".to_owned(), Some("true".to_owned())),
            ("valid".to_owned(), Some("true".to_owned())),
        ]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([("User-Agent".to_string(), Some("true".to_owned()))]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([("X-Amz-".to_string(), Some("true".to_owned()))]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());
    }

    #[test]
    fn test_endpoint_secret_validation() {
        let secret = EndpointSecret::Symmetric(STANDARD.decode("bm90LXZhbGlkCg==").unwrap());
        assert!(secret.validate().is_err());

        let secret =
            EndpointSecret::Symmetric(STANDARD.decode("C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap());
        secret.validate().unwrap();

        let secret = EndpointSecret::Asymmetric(AsymmetricKey::from_base64("6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==").unwrap());
        secret.validate().unwrap();

        let secret = EndpointSecret::Asymmetric(AsymmetricKey::from_base64("6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlaaaaaaaaaaAJ6p9lMicMFs6Kvg==").unwrap());
        assert!(secret.validate().is_err());
    }

    #[derive(serde::Deserialize)]
    struct EndpointSecretTestStruct {
        key: EndpointSecret,
    }

    #[test]
    fn test_endpoint_secret_deserialization() {
        for key in [
            "w",
            "whsec_%",
            "whsec_wronglength",
            "whpk_1SiA4o9hyqTCpIqC5V9HUakiiaeACeqfZTInDBbOir4=", // Public key
            "whsk_6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kv", // Bad SK
            "hwsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD",
        ] {
            let js = serde_json::json!({ "key": key });
            assert!(serde_json::from_value::<EndpointSecretTestStruct>(js).is_err());
        }

        let js = serde_json::json!({ "key": "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD" });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        if let EndpointSecret::Symmetric(key) = ep.key {
            assert_eq!(
                STANDARD.decode("C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap(),
                key
            );
        } else {
            panic!("Shouldn't get here");
        }

        // Too long secret
        let js = serde_json::json!({ "key": "whsec_V09IYXZUaFJoSnFobnpJQkpPMXdpdGFNWnJsRzAxdXZCeTVndVpwRmxSSXFsc0oyYzBTRWRUekJhYnlaZ0JSRGNPQ3BGZG1xYjFVVmRGQ3UK" });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        assert!(ep.key.validate().is_err());

        // Valid long secret
        let long_sec = "TUdfVE5UMnZlci1TeWxOYXQtX1ZlTW1kLTRtMFdhYmEwanIxdHJvenRCbmlTQ2hFdzBnbHhFbWdFaTJLdzQwSA==";
        let js = serde_json::json!({ "key": format!("whsec_{long_sec}") });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        if let EndpointSecret::Symmetric(key) = ep.key {
            assert_eq!(STANDARD.decode(long_sec).unwrap(), key);
        } else {
            panic!("Shouldn't get here");
        }

        // Asymmetric key
        let asym_sec = "6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==";
        let js = serde_json::json!({ "key": format!("whsk_{asym_sec}") });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        if let EndpointSecret::Asymmetric(key) = ep.key {
            assert_eq!(STANDARD.decode(asym_sec).unwrap(), key.0.sk.as_slice());
        } else {
            panic!("Shouldn't get here");
        }
    }
}

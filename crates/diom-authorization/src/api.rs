use std::fmt;

use diom_core::PersistableValue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ResourcePattern;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(transparent)]
pub struct RoleId(pub String);

impl RoleId {
    pub fn admin() -> Self {
        Self("admin".to_owned())
    }

    /// Role used by requests to the internal API server.
    ///
    /// Might be split into multiple roles down the line.
    pub fn operator() -> Self {
        Self("operator".to_owned())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(transparent)]
pub struct AccessPolicyId(pub String);

impl AccessPolicyId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AccessPolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, JsonSchema, PersistableValue)]
pub struct AccessRule {
    pub effect: AccessRuleEffect,
    pub resource: ResourcePattern,
    pub actions: Vec<String>,
}

impl AccessRule {
    pub fn uses_reserved_namespace(&self) -> bool {
        self.resource.namespace.is_reserved()
    }
}

// Unfortunately it is currently quite easy to create an AccessRule in code that
// when serialized would fail to deserialize again.
// This custom serialize implementation prevents that by reporting a
// serialization error if the access rule would not be deserializable.
//
// At the time of writing, there is no code that creates such an access rule
// (at least no `api::AccessRule`, the special operator rules only ever exist
// as the `AccessRule` type from the crate root), but it's hard to prevent
// such code from being introduced statically.
impl Serialize for AccessRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct as _;

        if self.uses_reserved_namespace() {
            return Err(serde::ser::Error::custom(
                "access rule with reserved namespace must not be serialized",
            ));
        }

        let Self {
            effect,
            resource,
            actions,
        } = self;

        let mut s = serde::Serializer::serialize_struct(serializer, "AccessRule", 3)?;
        s.serialize_field("effect", effect)?;
        s.serialize_field("resource", resource)?;
        s.serialize_field("actions", actions)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for AccessRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AccessRuleRepr {
            effect: AccessRuleEffect,
            resource: ResourcePattern,
            actions: Vec<String>,
        }

        let AccessRuleRepr {
            effect,
            resource,
            actions,
        } = AccessRuleRepr::deserialize(deserializer)?;

        let result = Self {
            effect,
            resource,
            actions,
        };

        if result.uses_reserved_namespace() {
            return Err(serde::de::Error::custom(format!(
                "namespace {} is reserved",
                result.resource.namespace
            )));
        }

        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleEffect {
    Allow,
    Deny,
}

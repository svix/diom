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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleEffect {
    Allow,
    Deny,
}

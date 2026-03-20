use std::{collections::HashMap, fmt};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct RoleId(String);

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

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct AccessPolicyId(String);

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

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Role {
    id: RoleId,
    description: String,
    #[serde(default)]
    rules: Vec<AccessRule>,
    #[serde(default)]
    policies: Vec<AccessPolicyId>,
    #[serde(default)]
    context: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AccessPolicy {
    id: AccessPolicyId,
    description: String,
    rules: Vec<AccessRule>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct AccessRule {
    effect: AccessRuleEffect,
    resource: String,
    actions: Vec<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleEffect {
    Allow,
    Deny,
}

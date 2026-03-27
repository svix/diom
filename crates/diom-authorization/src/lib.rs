use std::{collections::HashMap, fmt, sync::Arc};

use diom_id::{AuthTokenId, Module};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod pattern;
mod verification;

pub use self::{
    pattern::{KeyPattern, NamespacePattern, ResourcePattern},
    verification::{Forbidden, verify_operation},
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
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

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct Role {
    pub id: RoleId,
    pub description: String,
    #[serde(default)]
    pub rules: Vec<AccessRule>,
    #[serde(default)]
    pub policies: Vec<AccessPolicyId>,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct AccessPolicy {
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct AccessRule {
    pub effect: AccessRuleEffect,
    pub resource: ResourcePattern,
    pub actions: Vec<String>,
}

impl AccessRule {
    pub fn matches(&self, operation: &RequestedOperation<'_>) -> bool {
        self.resource.matches(operation) && self.actions.iter().any(|a| a == operation.action)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleEffect {
    Allow,
    Deny,
}

#[derive(Debug)]
pub struct RequestedOperation<'a> {
    pub module: Module,
    pub namespace: Option<&'a str>,
    pub key: Option<&'a str>,
    pub action: &'static str,
}

impl RequestedOperation<'_> {
    pub fn resource_str(&self) -> String {
        let module = self.module;
        let namespace = self.namespace.unwrap_or("");
        let key = self.key.unwrap_or("**");
        format!("{module}:{namespace}:{key}")
    }
}

/// The `Permissions` for a request
#[derive(Clone)]
pub struct Permissions {
    /// The role of the requester
    pub role: RoleId,
    /// The auth token id, if we used auth token
    pub auth_token_id: Option<AuthTokenId>,
    /// The access rules of the requester's role
    pub access_rules: Arc<[AccessRule]>,
}

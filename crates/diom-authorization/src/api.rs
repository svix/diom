use std::{
    fmt,
    sync::{Arc, OnceLock},
};

use diom_core::PersistableValue;
use diom_id::Module;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Context, KeyPattern, ModulePattern, NamespacePattern, RequestedOperation, ResourcePattern,
};

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
    pub fn admin_rules() -> Arc<[Self]> {
        static RULES: OnceLock<Arc<[AccessRule]>> = OnceLock::new();
        RULES
            .get_or_init(|| {
                [
                    ModulePattern::Any,
                    ModulePattern::Exactly(Module::AdminAccessPolicy),
                    ModulePattern::Exactly(Module::AdminAuthToken),
                    ModulePattern::Exactly(Module::AdminCluster),
                    ModulePattern::Exactly(Module::AdminNamespace),
                    ModulePattern::Exactly(Module::AdminRole),
                ]
                .map(|module| AccessRule {
                    effect: AccessRuleEffect::Allow,
                    resource: ResourcePattern {
                        module,
                        namespace: NamespacePattern::Any,
                        key: KeyPattern::any(),
                    },
                    actions: vec!["*".to_string()],
                })
                .into()
            })
            .clone()
    }

    pub fn operator_rules() -> Arc<[Self]> {
        static RULES: OnceLock<Arc<[AccessRule]>> = OnceLock::new();
        RULES
            .get_or_init(|| {
                [AccessRule {
                    effect: AccessRuleEffect::Allow,
                    resource: ResourcePattern {
                        module: ModulePattern::Any,
                        namespace: NamespacePattern::Named("_internal".to_owned()),
                        key: KeyPattern::any(),
                    },
                    actions: vec!["*".to_owned()],
                }]
                .into()
            })
            .clone()
    }

    pub fn uses_reserved_namespace(&self) -> bool {
        self.resource.namespace.is_reserved()
    }

    pub fn matches(&self, operation: &RequestedOperation<'_>, context: Context<'_>) -> bool {
        self.resource.matches(operation, context)
            && self
                .actions
                .iter()
                .any(|a| a == "*" || a == operation.action)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
#[serde(rename_all = "snake_case")]
pub enum AccessRuleEffect {
    Allow,
    Deny,
}

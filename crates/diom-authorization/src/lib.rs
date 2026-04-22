use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use diom_id::{AuthTokenId, Module};

pub mod api;
mod context;
mod pattern;
mod verification;

use self::api::{ModulePattern, ResourcePattern, RoleId};

pub use self::{
    context::Context,
    pattern::{KeyPattern, KeyPatternSegment, NamespacePattern},
    verification::{Forbidden, verify_operation},
};

#[derive(Debug, Default)]
pub struct AccessRuleList {
    allow: Vec<AccessRule>,
    deny: Vec<AccessRule>,
}

impl AccessRuleList {
    pub fn empty() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn admin() -> Arc<Self> {
        static RULES: OnceLock<Arc<AccessRuleList>> = OnceLock::new();
        RULES
            .get_or_init(|| {
                Arc::new(Self {
                    allow: [
                        ModulePattern::Any,
                        ModulePattern::Exactly(Module::AdminAccessPolicy),
                        ModulePattern::Exactly(Module::AdminAuthToken),
                        ModulePattern::Exactly(Module::AdminCluster),
                        ModulePattern::Exactly(Module::AdminNamespace),
                        ModulePattern::Exactly(Module::AdminRole),
                    ]
                    .map(|module| AccessRule {
                        resource: ResourcePattern {
                            module,
                            namespace: NamespacePattern::Any,
                            key: KeyPattern::any(),
                        },
                        actions: vec!["*".to_string()],
                    })
                    .into(),
                    deny: Vec::new(),
                })
            })
            .clone()
    }

    fn operator() -> Arc<Self> {
        static RULES: OnceLock<Arc<AccessRuleList>> = OnceLock::new();
        RULES
            .get_or_init(|| {
                Arc::new(Self {
                    allow: [AccessRule {
                        resource: ResourcePattern {
                            module: ModulePattern::Any,
                            namespace: NamespacePattern::Named("_internal".to_owned()),
                            key: KeyPattern::any(),
                        },
                        actions: vec!["*".to_owned()],
                    }]
                    .into(),
                    deny: Vec::new(),
                })
            })
            .clone()
    }
}

impl From<Vec<api::AccessRule>> for AccessRuleList {
    fn from(rules: Vec<api::AccessRule>) -> Self {
        let mut result = AccessRuleList::default();
        result.extend(rules);
        result
    }
}

impl Extend<api::AccessRule> for AccessRuleList {
    fn extend<T: IntoIterator<Item = api::AccessRule>>(&mut self, rules: T) {
        for rule in rules {
            let list = match rule.effect {
                api::AccessRuleEffect::Allow => &mut self.allow,
                api::AccessRuleEffect::Deny => &mut self.deny,
            };

            list.push(AccessRule {
                resource: rule.resource,
                actions: rule.actions,
            });
        }
    }
}

#[derive(Debug)]
pub struct AccessRule {
    pub resource: ResourcePattern,
    pub actions: Vec<String>,
}

impl AccessRule {
    pub fn matches(&self, operation: &RequestedOperation<'_>, context: Context<'_>) -> bool {
        self.resource.matches(operation, context)
            && self
                .actions
                .iter()
                .any(|a| a == "*" || a == operation.action)
    }
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
        let key = self.key.unwrap_or("*");
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
    pub access_rules: Arc<AccessRuleList>,
    /// Arbitrary key-value context forwarded from JWT claims (empty for non-JWT auth)
    pub context: HashMap<String, String>,
}

impl Permissions {
    /// Returns the builtin `admin` permissions.
    ///
    /// This constructor must only be used for requests that authenticate with
    /// the global admin token.
    pub fn admin() -> Self {
        Self {
            role: RoleId::admin(),
            auth_token_id: None,
            access_rules: AccessRuleList::admin(),
            context: HashMap::new(),
        }
    }

    /// Returns the builtin `operator` permissions.
    ///
    /// This constructor must only be used for requests made through the
    /// internal self-call server.
    pub fn operator() -> Self {
        Self {
            role: RoleId::operator(),
            auth_token_id: None,
            access_rules: AccessRuleList::operator(),
            context: HashMap::new(),
        }
    }
}

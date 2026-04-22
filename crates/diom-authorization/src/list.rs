use diom_id::Module;
use std::sync::{Arc, OnceLock};

use crate::{
    RequestedOperation,
    api::{self, ModulePattern, ResourcePattern},
    context::Context,
    pattern::{KeyPattern, NamespacePattern},
};

#[derive(Debug, Default)]
pub struct AccessRuleList {
    pub(crate) allow: Vec<AccessRule>,
    pub(crate) deny: Vec<AccessRule>,
}

impl AccessRuleList {
    pub fn empty() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub(crate) fn admin() -> Arc<Self> {
        pub(crate) static RULES: OnceLock<Arc<AccessRuleList>> = OnceLock::new();
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

    pub(crate) fn operator() -> Arc<Self> {
        pub(crate) static RULES: OnceLock<Arc<AccessRuleList>> = OnceLock::new();
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
pub(crate) struct AccessRule {
    pub resource: ResourcePattern,
    pub actions: Vec<String>,
}

impl AccessRule {
    pub(crate) fn matches(&self, operation: &RequestedOperation<'_>, context: Context<'_>) -> bool {
        self.resource.matches(operation, context)
            && self
                .actions
                .iter()
                .any(|a| a == "*" || a == operation.action)
    }
}

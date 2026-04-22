use diom_id::Module;
use std::sync::{Arc, OnceLock};

use crate::{
    RequestedOperation, api,
    context::Context,
    pattern::{KeyPattern, NamespacePattern},
};

pub struct Forbidden;

#[derive(Debug, Default)]
pub struct AccessRuleList {
    allow: AccessRuleListInner,
    deny: AccessRuleListInner,
}

impl AccessRuleList {
    pub fn empty() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn allow(allow: AccessRuleListInner) -> Arc<Self> {
        Arc::new(Self {
            allow,
            deny: AccessRuleListInner::default(),
        })
    }

    pub(crate) fn admin() -> Arc<Self> {
        pub(crate) static RULES: OnceLock<Arc<AccessRuleList>> = OnceLock::new();
        RULES
            .get_or_init(|| {
                Self::allow(AccessRuleListInner {
                    any_module_rules: vec![AccessRule::any()],

                    // Non-admin modules are covered by any_module_rules.
                    //
                    // Could use ..Default::default() for these,
                    // but this way we're forced to keep the list up to date.
                    cache_rules: vec![],
                    idempotency_rules: vec![],
                    kv_rules: vec![],
                    rate_limit_rules: vec![],
                    msgs_rules: vec![],
                    auth_token_rules: vec![],

                    // Admin modules are not covered by any_module_rules.
                    admin_cluster_rules: vec![AccessRule::any()],
                    admin_namespace_rules: vec![AccessRule::any()],
                    admin_auth_token_rules: vec![AccessRule::any()],
                    admin_role_rules: vec![AccessRule::any()],
                    admin_access_policy_rules: vec![AccessRule::any()],
                })
            })
            .clone()
    }

    pub(crate) fn operator() -> Arc<Self> {
        pub(crate) static RULES: OnceLock<Arc<AccessRuleList>> = OnceLock::new();
        RULES
            .get_or_init(|| {
                Self::allow(AccessRuleListInner {
                    any_module_rules: vec![AccessRule::internal()],
                    ..Default::default()
                })
            })
            .clone()
    }

    pub fn verify_operation(
        &self,
        operation: &RequestedOperation<'_>,
        context: Context<'_>,
    ) -> Result<(), Forbidden> {
        // deny rules take precedence, if we found a matching one
        // we can stop going through the rest and reject.
        if self.deny.matches(operation, context) {
            return Err(Forbidden);
        }

        // found an allow rule and allow deny rules have been checked.
        // request is okay.
        if self.allow.matches(operation, context) {
            return Ok(());
        }

        // no deny or allow rules found => implicit deny
        Err(Forbidden)
    }

    /// Discard any excess allocated capacity for more rules, to the extent possible.
    pub fn shrink_to_fit(&mut self) {
        self.allow.shrink_to_fit();
        self.deny.shrink_to_fit();
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

            list.push(rule.resource, rule.actions);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct AccessRuleListInner {
    any_module_rules: Vec<AccessRule>,

    cache_rules: Vec<AccessRule>,
    idempotency_rules: Vec<AccessRule>,
    kv_rules: Vec<AccessRule>,
    rate_limit_rules: Vec<AccessRule>,
    msgs_rules: Vec<AccessRule>,
    auth_token_rules: Vec<AccessRule>,
    admin_cluster_rules: Vec<AccessRule>,
    admin_namespace_rules: Vec<AccessRule>,
    admin_auth_token_rules: Vec<AccessRule>,
    admin_role_rules: Vec<AccessRule>,
    admin_access_policy_rules: Vec<AccessRule>,
}

impl AccessRuleListInner {
    fn push(&mut self, resource: api::ResourcePattern, actions: Vec<String>) {
        let api::ResourcePattern {
            module,
            namespace,
            key,
        } = resource;

        let rules = match module {
            api::ModulePattern::Any => &mut self.any_module_rules,
            api::ModulePattern::Exactly(module) => match module {
                Module::Cache => &mut self.cache_rules,
                Module::Idempotency => &mut self.idempotency_rules,
                Module::Kv => &mut self.kv_rules,
                Module::RateLimit => &mut self.rate_limit_rules,
                Module::Msgs => &mut self.msgs_rules,
                Module::AuthToken => &mut self.auth_token_rules,
                Module::AdminCluster => &mut self.admin_cluster_rules,
                Module::AdminNamespace => &mut self.admin_namespace_rules,
                Module::AdminAuthToken => &mut self.admin_auth_token_rules,
                Module::AdminRole => &mut self.admin_role_rules,
                Module::AdminAccessPolicy => &mut self.admin_access_policy_rules,
            },
        };

        rules.push(AccessRule {
            namespace,
            key,
            actions,
        });
    }

    fn shrink_to_fit(&mut self) {
        let Self {
            any_module_rules,
            cache_rules,
            idempotency_rules,
            kv_rules,
            rate_limit_rules,
            msgs_rules,
            auth_token_rules,
            admin_cluster_rules,
            admin_namespace_rules,
            admin_auth_token_rules,
            admin_role_rules,
            admin_access_policy_rules,
        } = self;

        any_module_rules.shrink_to_fit();
        cache_rules.shrink_to_fit();
        idempotency_rules.shrink_to_fit();
        kv_rules.shrink_to_fit();
        rate_limit_rules.shrink_to_fit();
        msgs_rules.shrink_to_fit();
        auth_token_rules.shrink_to_fit();
        admin_cluster_rules.shrink_to_fit();
        admin_namespace_rules.shrink_to_fit();
        admin_auth_token_rules.shrink_to_fit();
        admin_role_rules.shrink_to_fit();
        admin_access_policy_rules.shrink_to_fit();
    }

    fn matches(&self, operation: &RequestedOperation<'_>, context: Context<'_>) -> bool {
        // NOTE: Any substantial changes to this function, i.e. not just adding another module,
        //       should be mirrored in `api::ModulePattern::match` (used in lower-level tests).
        if !operation.module.is_admin_module()
            && self
                .any_module_rules
                .iter()
                .any(|r| r.matches(operation, context))
        {
            return true;
        }

        let rules = match operation.module {
            Module::Cache => &self.cache_rules,
            Module::Idempotency => &self.idempotency_rules,
            Module::Kv => &self.kv_rules,
            Module::RateLimit => &self.rate_limit_rules,
            Module::Msgs => &self.msgs_rules,
            Module::AuthToken => &self.auth_token_rules,
            Module::AdminCluster => &self.admin_cluster_rules,
            Module::AdminNamespace => &self.admin_namespace_rules,
            Module::AdminAuthToken => &self.admin_auth_token_rules,
            Module::AdminRole => &self.admin_role_rules,
            Module::AdminAccessPolicy => &self.admin_access_policy_rules,
        };
        rules.iter().any(|r| r.matches(operation, context))
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
struct AccessRule {
    namespace: NamespacePattern,
    key: KeyPattern,
    actions: Vec<String>,
}

impl AccessRule {
    /// Access Rule allows all actions on all keys within all (non-reserved) namespaces.
    fn any() -> Self {
        Self {
            namespace: NamespacePattern::Any,
            key: KeyPattern::any(),
            actions: vec!["*".to_string()],
        }
    }

    /// Access Rule that allows all actions on all keys within the special `_internal` namespace.
    ///
    /// For use by the builtin operator role, and nobody else.
    fn internal() -> Self {
        Self {
            namespace: NamespacePattern::Named("_internal".to_owned()),
            key: KeyPattern::any(),
            actions: vec!["*".to_owned()],
        }
    }

    fn matches(&self, operation: &RequestedOperation<'_>, context: Context<'_>) -> bool {
        self.namespace.matches(operation.namespace)
            && self.key.matches(operation.key, context)
            && self
                .actions
                .iter()
                .any(|a| a == "*" || a == operation.action)
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessRule, AccessRuleList, AccessRuleListInner};
    use crate::{
        api,
        pattern::{KeyPattern, NamespacePattern},
    };

    #[test]
    fn test_construction_from_api_access_rules() {
        let access_rules = vec![
            api::AccessRule {
                effect: api::AccessRuleEffect::Deny,
                resource: "kv:*:foo/bar".parse().unwrap(),
                actions: vec!["*".to_owned()],
            },
            api::AccessRule {
                effect: api::AccessRuleEffect::Allow,
                resource: "cache:ns:*".parse().unwrap(),
                actions: vec!["get".to_owned(), "set".to_owned()],
            },
            api::AccessRule {
                effect: api::AccessRuleEffect::Allow,
                resource: "*:ns2:xyz".parse().unwrap(),
                actions: vec!["*".to_owned()],
            },
            api::AccessRule {
                effect: api::AccessRuleEffect::Deny,
                resource: "admin/cluster::*".parse().unwrap(),
                actions: vec!["force-snapshot".to_owned()],
            },
            api::AccessRule {
                effect: api::AccessRuleEffect::Allow,
                resource: "*:ns3:abc".parse().unwrap(),
                actions: vec!["set".to_owned()],
            },
        ];

        let AccessRuleList { allow, deny } = AccessRuleList::from(access_rules);

        {
            let AccessRuleListInner {
                any_module_rules,
                cache_rules,
                idempotency_rules,
                kv_rules,
                rate_limit_rules,
                msgs_rules,
                auth_token_rules,
                admin_cluster_rules,
                admin_namespace_rules,
                admin_auth_token_rules,
                admin_role_rules,
                admin_access_policy_rules,
            } = allow;

            let [any_module_rule_1, any_module_rule_2]: [_; 2] =
                any_module_rules.try_into().unwrap();
            let [cache_module_rule]: [_; 1] = cache_rules.try_into().unwrap();
            assert_eq!(idempotency_rules.len(), 0);
            assert_eq!(kv_rules.len(), 0);
            assert_eq!(rate_limit_rules.len(), 0);
            assert_eq!(msgs_rules.len(), 0);
            assert_eq!(auth_token_rules.len(), 0);
            assert_eq!(admin_cluster_rules.len(), 0);
            assert_eq!(admin_namespace_rules.len(), 0);
            assert_eq!(admin_auth_token_rules.len(), 0);
            assert_eq!(admin_role_rules.len(), 0);
            assert_eq!(admin_access_policy_rules.len(), 0);

            assert_eq!(
                any_module_rule_1,
                AccessRule {
                    namespace: NamespacePattern::Named("ns2".to_owned()),
                    key: "xyz".parse().unwrap(),
                    actions: vec!["*".to_owned()]
                }
            );
            assert_eq!(
                any_module_rule_2,
                AccessRule {
                    namespace: NamespacePattern::Named("ns3".to_owned()),
                    key: "abc".parse().unwrap(),
                    actions: vec!["set".to_owned()]
                }
            );

            assert_eq!(
                cache_module_rule,
                AccessRule {
                    namespace: NamespacePattern::Named("ns".to_owned()),
                    key: KeyPattern::any(),
                    actions: vec!["get".to_owned(), "set".to_owned()]
                }
            );
        }

        {
            let AccessRuleListInner {
                any_module_rules,
                cache_rules,
                idempotency_rules,
                kv_rules,
                rate_limit_rules,
                msgs_rules,
                auth_token_rules,
                admin_cluster_rules,
                admin_namespace_rules,
                admin_auth_token_rules,
                admin_role_rules,
                admin_access_policy_rules,
            } = deny;

            let [kv_rule]: [_; 1] = kv_rules.try_into().unwrap();
            let [admin_cluster_rule]: [_; 1] = admin_cluster_rules.try_into().unwrap();
            assert_eq!(any_module_rules.len(), 0);
            assert_eq!(cache_rules.len(), 0);
            assert_eq!(idempotency_rules.len(), 0);
            assert_eq!(rate_limit_rules.len(), 0);
            assert_eq!(msgs_rules.len(), 0);
            assert_eq!(auth_token_rules.len(), 0);
            assert_eq!(admin_namespace_rules.len(), 0);
            assert_eq!(admin_auth_token_rules.len(), 0);
            assert_eq!(admin_role_rules.len(), 0);
            assert_eq!(admin_access_policy_rules.len(), 0);

            assert_eq!(
                kv_rule,
                AccessRule {
                    namespace: NamespacePattern::Any,
                    key: "foo/bar".parse().unwrap(),
                    actions: vec!["*".to_owned()],
                }
            );
            assert_eq!(
                admin_cluster_rule,
                AccessRule {
                    namespace: NamespacePattern::Default,
                    key: KeyPattern::any(),
                    actions: vec!["force-snapshot".to_owned()],
                }
            );
        }
    }
}

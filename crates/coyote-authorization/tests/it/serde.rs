#![expect(clippy::disallowed_types)] // serde_json::Value use is correct for tests

use std::collections::HashMap;

use coyote_authorization::{
    AccessPolicy, AccessPolicyId, AccessRule, AccessRuleEffect, KeyPattern, NamespacePattern,
    ResourcePattern, Role, RoleId,
};
use coyote_id::Module;
use serde_json::json;

fn example_rules() -> Vec<AccessRule> {
    vec![
        AccessRule {
            effect: AccessRuleEffect::Allow,
            resource: ResourcePattern {
                module: Module::Cache,
                namespace: NamespacePattern::Default,
                key: KeyPattern::Prefix("foo/".to_owned()),
            },
            actions: vec!["get".to_owned(), "set".to_owned()],
        },
        AccessRule {
            effect: AccessRuleEffect::Deny,
            resource: ResourcePattern {
                module: Module::Cache,
                namespace: NamespacePattern::Default,
                key: KeyPattern::Exactly("foo/bar".to_owned()),
            },
            actions: vec!["get".to_owned()],
        },
        AccessRule {
            effect: AccessRuleEffect::Allow,
            resource: ResourcePattern {
                module: Module::Kv,
                namespace: NamespacePattern::Default,
                key: KeyPattern::Prefix("some-data/".to_owned()),
            },
            actions: vec!["get".to_owned(), "set".to_owned()],
        },
    ]
}

fn example_rules_serialized() -> serde_json::Value {
    json!([
        {
            "effect": "allow",
            "resource": "cache::foo/*",
            "actions": [
                "get",
                "set"
            ],
        },
        {
            "effect": "deny",
            "resource": "cache::foo/bar",
            "actions": [
                "get"
            ],
        },
        {
            "effect": "allow",
            "resource": "kv::some-data/*",
            "actions": [
                "get",
                "set"
            ],
        },
    ])
}

#[test]
fn role_serde() {
    let role = Role {
        id: RoleId("my-role".to_owned()),
        description: "Whatever.".to_owned(),
        rules: example_rules(),
        policies: vec![
            AccessPolicyId("some-policy".to_owned()),
            AccessPolicyId("another-policy".to_owned()),
        ],
        context: HashMap::from_iter([
            ("tier".to_owned(), "enterprise".to_owned()),
            ("category".to_owned(), "my-category".to_owned()),
        ]),
    };

    let serialized = json!({
        "id": "my-role",
        "description": "Whatever.",
        "rules": example_rules_serialized(),
        "policies": [
            "some-policy",
            "another-policy",
        ],
        "context": {
            "tier": "enterprise",
            "category": "my-category",
        },
    });

    assert_eq!(serde_json::to_value(&role).unwrap(), serialized);
    assert_eq!(role, serde_json::from_value(serialized).unwrap());
}

#[test]
fn policy_serde() {
    let policy = AccessPolicy {
        id: AccessPolicyId("my-policy".to_owned()),
        description: "My first policy.".to_owned(),
        rules: example_rules(),
    };

    let serialized = json!({
        "id": "my-policy",
        "description": "My first policy.",
        "rules": example_rules_serialized()
    });

    assert_eq!(serde_json::to_value(&policy).unwrap(), serialized);
    assert_eq!(policy, serde_json::from_value(serialized).unwrap());
}

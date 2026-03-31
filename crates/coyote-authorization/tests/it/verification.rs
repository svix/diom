use coyote_authorization::{AccessRule, RequestedOperation, verify_operation};
use coyote_id::Module;
use serde_json::json;

fn example_rules() -> Vec<AccessRule> {
    serde_json::from_value(json!([
        {
            "effect": "allow",
            "resource": "cache::foo/*",
            "actions": [
                "read",
                "list"
            ],
        },
        {
            "effect": "deny",
            "resource": "cache::foo/bar",
            "actions": [
                "read"
            ],
        },
        {
            "effect": "allow",
            "resource": "cache::foo/baz",
            "actions": ["*"]
        },
        {
            "effect": "allow",
            "resource": "kv:xyz:some-data/*",
            "actions": [
                "create"
            ],
        },
        {
            "effect": "allow",
            "resource": "kv:*:some-data/*",
            "actions": [
                "read",
                "list"
            ],
        },
    ]))
    .unwrap()
}

#[test]
fn test_verify_cache_example() {
    let rules = example_rules();

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo"),
        action: "read",
    };
    // no rule matches (need at least the extra /)
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/foo"),
        action: "read",
    };
    // simple match of first rule
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/foo"),
        action: "create",
    };
    // no matching rule for action
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/baz"),
        action: "create",
    };
    // wildcard action allowed for this key
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/"),
        action: "read",
    };
    // match of first rule with ** matching empty string
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/bar"),
        action: "read",
    };
    // explicit deny rule matches
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/bar"),
        action: "list",
    };
    // deny rule only affects Read, not List
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/barr"),
        action: "read",
    };
    // deny rule is exact-match, does not match here
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: Some("ns"),
        key: Some("foo/foo"),
        action: "read",
    };
    // both cache rules only apply to the default namespace
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Msgs,
        namespace: None,
        key: Some("foo/foo"),
        action: "read",
    };
    // no matching rule for module
    assert!(verify_operation(&op, &rules).is_err());
}

#[test]
fn test_verify_kv_example() {
    let rules = example_rules();

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("xyz"),
        key: Some("some-data/foo"),
        action: "create",
    };
    // matches the first rule
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("xyz"),
        key: Some("some-data/foo"),
        action: "read",
    };
    // matches the second rule
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: None,
        key: Some("some-data/foo"),
        action: "create",
    };
    // Create action is only available for xyz namespace
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("abc"),
        key: Some("some-data/foo"),
        action: "create",
    };
    // same issue
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: None,
        key: Some("some-data/foo"),
        action: "read",
    };
    // Read action does work for arbitrary namespaces, including the default one
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("abc"),
        key: Some("some-data/foo"),
        action: "list",
    };
    // Same for List action, and a different named namespace
    assert!(verify_operation(&op, &rules).is_ok());
}

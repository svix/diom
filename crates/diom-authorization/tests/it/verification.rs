use diom_authorization::{AccessRule, RequestedOperation, verify_operation};
use diom_id::Module;
use serde_json::json;

fn example_rules() -> Vec<AccessRule> {
    serde_json::from_value(json!([
        {
            "effect": "allow",
            "resource": "cache::foo/*",
            "actions": [
                "Read",
                "List"
            ],
        },
        {
            "effect": "deny",
            "resource": "cache::foo/bar",
            "actions": [
                "Read"
            ],
        },
        {
            "effect": "allow",
            "resource": "kv:xyz:some-data/*",
            "actions": [
                "Create"
            ],
        },
        {
            "effect": "allow",
            "resource": "kv:*:some-data/*",
            "actions": [
                "Read",
                "List"
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
        action: "Read",
    };
    // no rule matches (need at least the extra /)
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/foo"),
        action: "Read",
    };
    // simple match of first rule
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/foo"),
        action: "Create",
    };
    // no matching rule for action
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/"),
        action: "Read",
    };
    // match of first rule with ** matching empty string
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/bar"),
        action: "Read",
    };
    // explicit deny rule matches
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/bar"),
        action: "List",
    };
    // deny rule only affects Read, not List
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/barr"),
        action: "Read",
    };
    // deny rule is exact-match, does not match here
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: Some("ns"),
        key: Some("foo/foo"),
        action: "Read",
    };
    // both cache rules only apply to the default namespace
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Msgs,
        namespace: None,
        key: Some("foo/foo"),
        action: "Read",
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
        action: "Create",
    };
    // matches the first rule
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("xyz"),
        key: Some("some-data/foo"),
        action: "Read",
    };
    // matches the second rule
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: None,
        key: Some("some-data/foo"),
        action: "Create",
    };
    // Create action is only available for xyz namespace
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("abc"),
        key: Some("some-data/foo"),
        action: "Create",
    };
    // same issue
    assert!(verify_operation(&op, &rules).is_err());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: None,
        key: Some("some-data/foo"),
        action: "Read",
    };
    // Read action does work for arbitrary namespaces, including the default one
    assert!(verify_operation(&op, &rules).is_ok());

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("abc"),
        key: Some("some-data/foo"),
        action: "List",
    };
    // Same for List action, and a different named namespace
    assert!(verify_operation(&op, &rules).is_ok());
}

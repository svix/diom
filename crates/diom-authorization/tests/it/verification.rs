use diom_authorization::{AccessRuleList, Context, RequestedOperation, api::AccessRule};
use diom_id::Module;
use serde_json::json;

fn example_rules() -> AccessRuleList {
    let rules: Vec<AccessRule> = serde_json::from_value(json!([
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
            "resource": "cache::foo/baz",
            "actions": ["*"]
        },
        {
            "effect": "allow",
            "resource": "kv:xyz:some-data/*",
            "actions": [
                "delete"
            ],
        },
        {
            "effect": "allow",
            "resource": "kv:*:some-data/*",
            "actions": [
                "get",
                "set"
            ],
        },
    ]))
    .unwrap();

    rules.into()
}

#[test]
fn test_verify_cache_example() {
    let rules = example_rules();

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo"),
        action: "get",
    };
    // no rule matches (need at least the extra /)
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/foo"),
        action: "get",
    };
    // simple match of first rule
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/foo"),
        action: "delete",
    };
    // no matching rule for action
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/baz"),
        action: "delete",
    };
    // wildcard action allowed for this key
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/"),
        action: "get",
    };
    // match of first rule with ** matching empty string
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/bar"),
        action: "get",
    };
    // explicit deny rule matches
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/bar"),
        action: "set",
    };
    // deny rule only affects Read, not List
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: None,
        key: Some("foo/barr"),
        action: "get",
    };
    // deny rule is exact-match, does not match here
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Cache,
        namespace: Some("ns"),
        key: Some("foo/foo"),
        action: "get",
    };
    // both cache rules only apply to the default namespace
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );

    let op = RequestedOperation {
        module: Module::Msgs,
        namespace: None,
        key: Some("foo/foo"),
        action: "get",
    };
    // no matching rule for module
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );
}

#[test]
fn test_verify_kv_example() {
    let rules = example_rules();

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("xyz"),
        key: Some("some-data/foo"),
        action: "delete",
    };
    // matches the first rule
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("xyz"),
        key: Some("some-data/foo"),
        action: "get",
    };
    // matches the second rule
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: None,
        key: Some("some-data/foo"),
        action: "delete",
    };
    // Create action is only available for xyz namespace
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("abc"),
        key: Some("some-data/foo"),
        action: "delete",
    };
    // same issue
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_err()
    );

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: None,
        key: Some("some-data/foo"),
        action: "get",
    };
    // Read action does work for arbitrary namespaces, including the default one
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );

    let op = RequestedOperation {
        module: Module::Kv,
        namespace: Some("abc"),
        key: Some("some-data/foo"),
        action: "set",
    };
    // Same for List action, and a different named namespace
    assert!(
        rules
            .verify_operation(&op, Context::empty_for_tests())
            .is_ok()
    );
}

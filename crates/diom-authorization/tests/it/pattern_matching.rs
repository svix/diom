use diom_authorization::{RequestedOperation, ResourcePattern};
use diom_id::Module;

static EXAMPLE_OP_KV: RequestedOperation<'static> = RequestedOperation {
    module: Module::Kv,
    namespace: None,
    key: Some("foo/bar"),
    action: "Read",
};

#[test]
fn test_default_namespace_exact_key_match() {
    let pat = "kv::foo/bar".parse::<ResourcePattern>().unwrap();
    assert!(pat.matches(&EXAMPLE_OP_KV));
}

#[test]
fn test_any_namespace_exact_key_match() {
    let pat = "kv:*:foo/bar".parse::<ResourcePattern>().unwrap();
    assert!(pat.matches(&EXAMPLE_OP_KV));
}

#[test]
fn test_default_namespace_any_key_match() {
    let pat = "kv::**".parse::<ResourcePattern>().unwrap();
    assert!(pat.matches(&EXAMPLE_OP_KV));
}

#[test]
fn test_any_namespace_any_key_match() {
    let pat = "kv:*:**".parse::<ResourcePattern>().unwrap();
    assert!(pat.matches(&EXAMPLE_OP_KV));
}

#[test]
fn test_any_namespace_wrong_module() {
    for wrong_module in ["auth_token", "cache", "idempotency", "msgs"] {
        let pat_s = &format!("{wrong_module}::**");
        let pat = pat_s.parse::<ResourcePattern>().unwrap();
        assert!(!pat.matches(&EXAMPLE_OP_KV), "{pat_s}");
    }
}

#[test]
fn test_any_namespace_wrong_exact_key_pattern() {
    for wrong_key in ["foo/baz", "foox/bar", "foo/ba", "foo/bars", "foobar"] {
        let pat_s = format!("kv:*:{wrong_key}");
        let pat = pat_s.parse::<ResourcePattern>().unwrap();
        assert!(!pat.matches(&EXAMPLE_OP_KV), "{pat_s}");
    }
}

static EXAMPLE_OP_AUTH_TOKEN: RequestedOperation<'static> = RequestedOperation {
    module: Module::AuthToken,
    namespace: Some("my-ns"),
    key: None,
    action: "Create",
};

#[test]
fn test_explicit_namespace_any_key_match() {
    let pat = "auth_token:my-ns:**".parse::<ResourcePattern>().unwrap();
    assert!(pat.matches(&EXAMPLE_OP_AUTH_TOKEN));
}

#[test]
fn test_any_namespace_any_key_match_2() {
    let pat = "auth_token:*:**".parse::<ResourcePattern>().unwrap();
    assert!(pat.matches(&EXAMPLE_OP_AUTH_TOKEN));
}

#[test]
fn test_wrong_namespace() {
    for wrong_ns in ["myns", "my-", "-ns", "m", "my-ns-"] {
        let pat_s = format!("auth_token:{wrong_ns}:**");
        let pat = pat_s.parse::<ResourcePattern>().unwrap();
        assert!(!pat.matches(&EXAMPLE_OP_AUTH_TOKEN), "{pat_s}");
    }
}

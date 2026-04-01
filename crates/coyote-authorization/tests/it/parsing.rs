use coyote_authorization::ResourcePattern;

#[test]
fn test_parse_invalid_ns_wildcard() {
    let err = "kv:x*:*".parse::<ResourcePattern>().unwrap_err();
    assert_eq!(
        err,
        "invalid namespace pattern: wildcard only allowed independently"
    );
}

#[test]
fn test_parse_invalid_key_wildcard() {
    let err = "kv::x*".parse::<ResourcePattern>().unwrap_err();
    assert_eq!(
        err,
        "asterisk may only be used as a standalone slash-separated segment"
    );
}

#[test]
fn test_parse_invalid_key_placeholder() {
    for invalid_pat in ["kv::$", "kv::${/", "kv::foo/$x", "kv::foo/{y}"] {
        let err = invalid_pat.parse::<ResourcePattern>().unwrap_err();
        assert_eq!(err, "invalid key pattern segment", "{invalid_pat}");
    }
}

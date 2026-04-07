use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use serde_json::json;
use test_utils::{StatusCode, TestClient, TestResult, server::TestServerBuilder};

use coyote_backend::cfg::{JwtConfig, JwtKey};

const JWT_SECRET: &str = "test-jwt-secret-do-not-use-in-production";

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn hs256_cfg() -> JwtConfig {
    JwtConfig {
        key: JwtKey::Hs256 {
            secret: JWT_SECRET.to_string(),
        },
        audience: None,
        issuer: None,
    }
}

fn make_jwt(claims: impl Serialize) -> String {
    encode(
        &Header::default(), // HS256
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .unwrap()
}

/// Start a server with JWT enabled and a role that can read/write the default KV namespace.
async fn setup() -> (test_utils::server::TestContext, String) {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| cfg.jwt = Some(hs256_cfg()))
        .build()
        .await;

    ctx.client
        .post("v1.admin.auth-role.upsert")
        .json(json!({
            "id": "jwt-kv-role",
            "description": "KV read/write role used in JWT tests",
            "rules": [
                { "effect": "allow", "resource": "kv:*:*", "actions": ["*"] }
            ],
        }))
        .await
        .unwrap()
        .ensure(StatusCode::OK)
        .unwrap();

    let base_uri = ctx.client.base_uri.clone();
    (ctx, base_uri)
}

#[tokio::test]
async fn test_jwt_valid_token() -> TestResult {
    let (_ctx, base_uri) = setup().await;

    let token = make_jwt(json!({
        "role": "jwt-kv-role",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.set")
        .json(json!({ "key": "jwt-test", "value": "hello".as_bytes(), "behavior": "upsert" }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "jwt-test" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert!(!resp["value"].is_null());
    Ok(())
}

#[tokio::test]
async fn test_jwt_context_forwarded() -> TestResult {
    // Context claims are accepted without error; the role is what gates access.
    let (_ctx, base_uri) = setup().await;

    let token = make_jwt(json!({
        "role": "jwt-kv-role",
        "context": { "team": "platform", "env": "test" },
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.set")
        .json(json!({ "key": "jwt-ctx-test", "value": "hi".as_bytes(), "behavior": "upsert" }))
        .await?
        .ensure(StatusCode::OK)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_expired_token() -> TestResult {
    let (_ctx, base_uri) = setup().await;

    // exp 100s in the past, well beyond the 60s leeway
    let token = make_jwt(json!({
        "role": "jwt-kv-role",
        "exp": now_secs() - 100,
    }));
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_nbf_in_future() -> TestResult {
    let (_ctx, base_uri) = setup().await;

    // nbf 1 hour in the future, well beyond the 60s leeway
    let token = make_jwt(json!({
        "role": "jwt-kv-role",
        "exp": now_secs() + 7200,
        "nbf": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_missing_exp() -> TestResult {
    let (_ctx, base_uri) = setup().await;

    // No `exp` claim — must be rejected because exp is required
    let token = make_jwt(json!({ "role": "jwt-kv-role" }));
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_invalid_signature() -> TestResult {
    let (_ctx, base_uri) = setup().await;

    let token = encode(
        &Header::default(),
        &json!({
            "role": "jwt-kv-role",
            "exp": now_secs() + 3600,
        }),
        &EncodingKey::from_secret(b"wrong-secret"),
    )
    .unwrap();
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_not_used_when_unconfigured() -> TestResult {
    // Without JWT config, a JWT-shaped token should be treated as a regular
    // auth token and fail with 401 (not a valid auth token).
    let ctx = TestServerBuilder::with_default_config().build().await;

    let token = make_jwt(json!({
        "role": "jwt-kv-role",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_audience_valid() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: Some(vec!["https://api.example.com".to_string()]),
                issuer: None,
            })
        })
        .build()
        .await;

    ctx.client
        .post("v1.admin.auth-role.upsert")
        .json(json!({
            "id": "jwt-aud-role",
            "description": "Audience test role",
            "rules": [{ "effect": "allow", "resource": "kv:*:*", "actions": ["*"] }],
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let token = make_jwt(json!({
        "role": "jwt-aud-role",
        "aud": "https://api.example.com",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.set")
        .json(json!({ "key": "aud-test", "value": "ok".as_bytes(), "behavior": "upsert" }))
        .await?
        .ensure(StatusCode::OK)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_audience_mismatch() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: Some(vec!["https://api.example.com".to_string()]),
                issuer: None,
            })
        })
        .build()
        .await;

    let token = make_jwt(json!({
        "role": "any-role",
        "aud": "https://other.example.com",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_audience_not_required_when_unconfigured() -> TestResult {
    // When no audience is configured, tokens with or without an `aud` claim
    // should both be accepted (aud validation is disabled).
    let (_ctx, base_uri) = setup().await;

    // Token with an aud claim — should still work since we don't configure an audience.
    let token = make_jwt(json!({
        "role": "jwt-kv-role",
        "aud": "https://anything.example.com",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(base_uri, &token);

    jwt_client
        .post("v1.kv.set")
        .json(json!({ "key": "aud-unconfigured", "value": "ok".as_bytes(), "behavior": "upsert" }))
        .await?
        .ensure(StatusCode::OK)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_audience_missing_when_required() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: Some(vec!["https://api.example.com".to_string()]),
                issuer: None,
            })
        })
        .build()
        .await;

    // Token has no `aud` claim but the config requires one.
    let token = make_jwt(json!({
        "role": "any-role",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_multiple_audiences() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: Some(vec![
                    "https://api.example.com".to_string(),
                    "https://api2.example.com".to_string(),
                ]),
                issuer: None,
            })
        })
        .build()
        .await;

    ctx.client
        .post("v1.admin.auth-role.upsert")
        .json(json!({
            "id": "jwt-multi-aud-role",
            "description": "Multi-audience test role",
            "rules": [{ "effect": "allow", "resource": "kv:*:*", "actions": ["*"] }],
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let base_uri = ctx.client.base_uri.clone();

    // Either configured audience value should be accepted.
    for aud in ["https://api.example.com", "https://api2.example.com"] {
        let token = make_jwt(json!({
            "role": "jwt-multi-aud-role",
            "aud": aud,
            "exp": now_secs() + 3600,
        }));
        TestClient::new(base_uri.clone(), &token)
            .post("v1.kv.set")
            .json(
                json!({ "key": "multi-aud-test", "value": "ok".as_bytes(), "behavior": "upsert" }),
            )
            .await?
            .ensure(StatusCode::OK)?;
    }

    // An audience not in the configured list must be rejected.
    let token = make_jwt(json!({
        "role": "jwt-multi-aud-role",
        "aud": "https://unlisted.example.com",
        "exp": now_secs() + 3600,
    }));
    TestClient::new(base_uri, &token)
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_issuer_valid() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: None,
                issuer: Some(vec!["https://auth.example.com".to_string()]),
            })
        })
        .build()
        .await;

    ctx.client
        .post("v1.admin.auth-role.upsert")
        .json(json!({
            "id": "jwt-iss-role",
            "description": "Issuer test role",
            "rules": [{ "effect": "allow", "resource": "kv:*:*", "actions": ["*"] }],
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let token = make_jwt(json!({
        "role": "jwt-iss-role",
        "iss": "https://auth.example.com",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.set")
        .json(json!({ "key": "iss-test", "value": "ok".as_bytes(), "behavior": "upsert" }))
        .await?
        .ensure(StatusCode::OK)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_issuer_mismatch() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: None,
                issuer: Some(vec!["https://auth.example.com".to_string()]),
            })
        })
        .build()
        .await;

    let token = make_jwt(json!({
        "role": "any-role",
        "iss": "https://evil.example.com",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

#[tokio::test]
async fn test_jwt_issuer_missing_when_required() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.jwt = Some(JwtConfig {
                key: JwtKey::Hs256 {
                    secret: JWT_SECRET.to_string(),
                },
                audience: None,
                issuer: Some(vec!["https://auth.example.com".to_string()]),
            })
        })
        .build()
        .await;

    // Token has no `iss` claim but the config requires one.
    let token = make_jwt(json!({
        "role": "any-role",
        "exp": now_secs() + 3600,
    }));
    let jwt_client = TestClient::new(ctx.client.base_uri.clone(), &token);

    jwt_client
        .post("v1.kv.get")
        .json(json!({ "key": "any" }))
        .await?
        .ensure(StatusCode::UNAUTHORIZED)?;

    Ok(())
}

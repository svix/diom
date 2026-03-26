use std::time::Duration;

use axum::{
    RequestExt,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use coyote_authorization::{
    AccessRule, AccessRuleEffect, KeyPattern, NamespacePattern, Permissions, ResourcePattern,
    RoleId,
};
use coyote_id::Module;
use tracing::Span;

use crate::{
    AppState,
    core::INTERNAL_NAMESPACE,
    v1::endpoints::auth_token::{AuthTokenVerifyIn, AuthTokenVerifyOut},
};
use coyote_admin_auth::State as AdminAuthState;
use coyote_error::{Error, ResultExt};

const AUTH_TOKEN_CACHE_TTL: Duration = Duration::from_secs(1);

pub(crate) use coyote_core::fifo_cache::FifoCache;

fn constant_time_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    if a.len() != b.len() {
        return false;
    }

    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

pub async fn authorization(
    state: State<AppState>,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    let perms = match request.extensions().get::<Permissions>() {
        Some(perms) => perms.to_owned(),
        None => {
            let perms = authorization_inner(state, &mut request).await?;
            request.extensions_mut().insert(perms.clone());
            perms
        }
    };

    // FIXME - get this explicitly from `AxumOtelSpanCreator`, instead of relying on
    // this middleware being invoked with that span active
    let http_request_span = Span::current();
    http_request_span.record("role", perms.role.as_str());
    if let Some(token_id) = &perms.auth_token_id {
        http_request_span.record("token_id", tracing::field::display(token_id.public()));
    }

    // This is run outside of the `tracing::instrument` function, so that the
    // route handler execution time isn't included in the span
    let mut response = next.run(request).await;
    response.extensions_mut().insert(perms);
    Ok(response)
}

/// Adds [`PermissionsAndAuditData`] to the Request, making it available in handlers via the `Extension`
/// extractor.
///
/// <https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers>
#[tracing::instrument(name = "authorization", skip_all, level = "trace")]
async fn authorization_inner(
    State(state): State<AppState>,
    request: &mut Request,
) -> axum::response::Result<Permissions> {
    let TypedHeader(Authorization(bearer)) = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| Error::authentication("auth_required", "`Authorization` header required"))?;

    let token = bearer.token();

    if let Some(admin_token) = &state.cfg.admin_token
        && constant_time_eq(admin_token, token)
    {
        // FIXME: we should block creating admin tokens on disk (so dynamic such tokens).
        // FIXME: ResourcePattern should support `*` for the module instead of having to hardcode.
        let perms = Permissions {
            role: RoleId::admin(),
            auth_token_id: None,
            access_rules: [
                Module::Cache,
                Module::Idempotency,
                Module::Kv,
                Module::RateLimit,
                Module::Msgs,
                Module::AuthToken,
            ]
            .into_iter()
            .map(|module| AccessRule {
                effect: AccessRuleEffect::Allow,
                resource: ResourcePattern {
                    module,
                    namespace: NamespacePattern::Any,
                    key: KeyPattern::Any,
                },
                actions: vec!["*".to_string()],
            })
            .collect::<Vec<_>>()
            .into(),
        };
        return Ok(perms);
    }

    let cached = state
        .auth_token_cache
        .read()
        .get(token, AUTH_TOKEN_CACHE_TTL)
        .cloned();

    let perms = if let Some(cached) = cached {
        cached
    } else {
        let out: AuthTokenVerifyOut = state
            .internal_call(
                "v1.auth-token.verify",
                &AuthTokenVerifyIn {
                    token: token.to_owned(),
                    namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                },
            )
            .await
            .or_internal_error()?;

        if let Some(mut out_token) = out.token {
            let role_id = RoleId::from_string(out_token.metadata.remove("role").unwrap());
            let admin_auth =
                AdminAuthState::init(state.do_not_use_dbs.clone()).or_internal_error()?;
            // FIXME: this is bad, we probably want to store the normalized role structure in a
            // forever or very long cache (never leaves RAM, can be invalidated by raft).
            let access_rules: Vec<_> = if let Some(role) = admin_auth
                .controller
                .get_role(&role_id)
                .await
                .or_internal_error()?
            {
                let mut rules = role.rules;
                for policy_id in &role.policies {
                    if let Some(policy) = admin_auth
                        .controller
                        .get_policy(policy_id)
                        .await
                        .or_internal_error()?
                    {
                        rules.extend(policy.rules);
                    }
                }
                rules
            } else {
                vec![]
            };
            let perms = Permissions {
                role: role_id,
                auth_token_id: Some(out_token.id.into_inner()),
                access_rules: access_rules.into(),
            };
            state
                .auth_token_cache
                .write()
                .put(token.to_string(), perms.clone());
            perms
        } else {
            return Err(Error::authentication("invalid_token", "Invalid token.").into());
        }
    };

    Ok(perms)
}

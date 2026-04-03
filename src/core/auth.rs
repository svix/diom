use std::{collections::HashMap, sync::Arc, time::Duration};

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
use diom_authorization::{AccessRule, Permissions, RoleId};
use diom_error::{OptionExt, Result};
use tracing::Span;

use crate::{
    AppState,
    core::{INTERNAL_NAMESPACE, jwt::JwtVerifier},
    v1::endpoints::auth_token::{AuthTokenVerifyIn, AuthTokenVerifyOut},
};
use diom_admin_auth::State as AdminAuthState;
use diom_error::{Error, ResultExt};

pub(crate) use diom_core::fifo_cache::FifoCache;

const AUTH_TOKEN_CACHE_TTL: Duration = Duration::from_secs(1);
const RULES_CACHE_TTL: Duration = Duration::from_secs(5);

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
        // FIXME: ensure that other auth tokens can't use the builtin admin (or operator) role
        return Ok(Permissions::admin());
    }

    if let Some(cached) = state
        .auth_token_cache
        .read()
        .get(token, AUTH_TOKEN_CACHE_TTL)
        .cloned()
    {
        return Ok(cached);
    }

    // Attempt JWT verification when a JWT verifier is present and the token looks like a JWT.
    if let Some(jwt_verifier) = &state.jwt_verifier
        && JwtVerifier::looks_like_jwt(token)
    {
        let claims = jwt_verifier
            .verify(token)
            .map_err(axum::response::IntoResponse::into_response)?;
        let role = RoleId(claims.role);
        let access_rules = resolve_access_rules(&state, &role).await?;
        let perms = Permissions {
            role,
            auth_token_id: None,
            access_rules,
            context: claims.context,
        };
        state
            .auth_token_cache
            .write()
            .put(token.to_string(), perms.clone());
        return Ok(perms);
    }

    let verify_out: AuthTokenVerifyOut = state
        .internal_call(
            "v1.auth-token.verify",
            &AuthTokenVerifyIn {
                token: token.to_owned(),
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
            },
        )
        .await
        .or_internal_error()?;

    let Some(mut auth_token) = verify_out.token else {
        return Err(Error::authentication("invalid_token", "Invalid token.").into());
    };

    let role = RoleId(
        auth_token
            .metadata
            .remove("role")
            .ok_or_internal_error("couldn't find role in auth token metadata")?,
    );

    let access_rules = resolve_access_rules(&state, &role).await?;
    let perms = Permissions {
        role,
        auth_token_id: Some(auth_token.id.into_inner()),
        access_rules,
        context: HashMap::new(),
    };

    state
        .auth_token_cache
        .write()
        .put(token.to_string(), perms.clone());

    Ok(perms)
}

async fn resolve_access_rules(state: &AppState, role_id: &RoleId) -> Result<Arc<[AccessRule]>> {
    if let Some(cached) = state
        .rules_cache
        .read()
        .get(role_id.as_str(), RULES_CACHE_TTL)
        .cloned()
    {
        return Ok(cached);
    }

    let admin_auth = AdminAuthState::init(state.do_not_use_dbs.clone()).or_internal_error()?;

    let Some(role) = admin_auth
        .controller
        .get_role(role_id)
        .await
        .or_internal_error()?
    else {
        return Ok([].into());
    };

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

    let rules: Arc<[AccessRule]> = rules.into();
    state
        .rules_cache
        .write()
        .put(role_id.to_string(), rules.clone());
    Ok(rules)
}

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
use diom_authorization::RoleId;
use diom_id::AuthTokenId;
use tracing::Span;

use crate::AppState;
use diom_error::Error;

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

/// The `Permissions` for a request
#[derive(Clone)]
pub struct Permissions {
    // pub scopes: ScopePermissions,
    /// The role of the requester
    pub role: RoleId,
    /// The auth token id, if we used auth token
    pub auth_token_id: Option<AuthTokenId>,
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

    let perms = if let Some(admin_token) = state.cfg.admin_token.as_ref()
        && constant_time_eq(admin_token, token)
    {
        Permissions {
            role: RoleId::admin(),
            auth_token_id: None,
        }
    } else {
        return Err(Error::authentication("invalid_token", "Invalid token.").into());
    };

    Ok(perms)
}

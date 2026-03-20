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
use tracing::Span;

use crate::AppState;
use coyote_error::Error;

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
    /// FIXME: probably want to use the right type.
    pub role: String,
    /// The auth token id, if we used auth token
    /// FIXME: probably want to use the right type.
    pub auth_token_id: Option<String>,
}

pub async fn authorization(
    state: State<AppState>,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    // FIXME - get this explicitly from `AxumOtelSpanCreator`, instead of relying on
    // this middleware being invoked with that span active
    let span = Span::current();
    let perms = authorization_inner(state, &mut request, span).await?;
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
    http_request_span: Span,
) -> axum::response::Result<Permissions> {
    let TypedHeader(Authorization(bearer)) = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| Error::authentication("auth_required", "`Authorization` header required"))?;

    let token = bearer.token();

    let perms = if let Some(admin_token) = state.cfg.admin_token.as_ref() {
        if !constant_time_eq(admin_token, token) {
            return Err(Error::authentication("invalid_token", "Invalid token.").into());
        }

        Permissions {
            role: "admin".to_string(),
            auth_token_id: Some("admin".to_string()),
        }
    } else {
        // FIXME: support non-admin tokens too.
        return Err(Error::authentication("invalid_token", "Invalid token.").into());
    };

    http_request_span.record("role", perms.role.as_str());
    if let Some(role) = perms.auth_token_id.as_ref() {
        http_request_span.record("role", role);
    }

    Ok(perms)
}

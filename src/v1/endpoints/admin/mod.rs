pub mod auth;
pub mod metrics;

use aide::axum::ApiRouter;

use crate::AppState;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(auth::router())
        .merge(metrics::router())
}

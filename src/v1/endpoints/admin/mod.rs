pub mod auth;
pub mod cluster;

use aide::axum::ApiRouter;

use crate::AppState;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(cluster::router())
        .merge(auth::router())
}

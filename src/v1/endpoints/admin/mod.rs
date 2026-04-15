pub mod auth;

use aide::axum::ApiRouter;

use crate::AppState;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().merge(auth::router())
}

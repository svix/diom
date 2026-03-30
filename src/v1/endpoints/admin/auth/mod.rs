pub mod role;
pub mod token;

use aide::axum::ApiRouter;

use crate::AppState;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(token::router())
        .merge(role::router())
}

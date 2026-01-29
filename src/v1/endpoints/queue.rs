use aide::axum::ApiRouter;

use crate::{AppState, v1::utils::openapi_tag};

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Queue");

    ApiRouter::new()
}

use std::time::Duration;

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::State;
use coyote_core::types::DurationMs;
use coyote_derive::aide_annotate;
use coyote_proto::MsgPackOrJson;
use coyote_transformations::run_script;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, error::Result, v1::utils::openapi_tag};

fn default_max_duration_ms() -> DurationMs {
    DurationMs::from(500u64)
}

#[derive(Deserialize, Validate, JsonSchema)]
pub struct TransformIn {
    /// JSON-encoded payload passed to the script as `input`.
    pub input: String,
    /// JavaScript source. Must define a `handler(input)` function.
    pub script: String,
    /// How long to let the script run before being killed.
    #[serde(default = "default_max_duration_ms")]
    pub max_duration_ms: DurationMs,
}

#[derive(Serialize, JsonSchema)]
pub struct TransformOut {
    /// JSON-encoded value returned by the script's `handler` function.
    pub output: String,
}

/// Execute a JavaScript transformation script against a payload and return the result.
#[aide_annotate(op_id = "v1.transformations.execute")]
async fn execute(
    State(_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<TransformIn>,
) -> Result<MsgPackOrJson<TransformOut>> {
    let output = run_script(&data.script, &data.input, Duration::from_millis(data.max_duration_ms.as_millis()))
        .await
        .map_err(coyote_error::Error::from)?;
    Ok(MsgPackOrJson(TransformOut { output }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Transformations");
    ApiRouter::new().api_route_with(execute_path, post_with(execute, execute_operation), &tag)
}

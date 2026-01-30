use std::error::Error as StdError;

use aide::OperationIo;
use axum::extract::{
    FromRequest, Request,
    rejection::{BytesRejection, FailedToBufferBody},
};
use coyote_error::{Error, HttpError, Result, ValidationErrorItem, validation_errors};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default, OperationIo)]
#[aide(input_with = "axum::extract::Json<T>", json_schema)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self> {
        let b = bytes::Bytes::from_request(req, state).await.map_err(|e| {
            tracing::error!("Error reading body as bytes: {}", e);

            match e {
                BytesRejection::FailedToBufferBody(FailedToBufferBody::LengthLimitError(_)) => {
                    HttpError::too_large(None, None)
                }

                _ => HttpError::internal_server_error(
                    None,
                    Some("Failed to read request body".to_owned()),
                ),
            }
        })?;
        let mut de = serde_json::Deserializer::from_slice(&b);

        let value: T = serde_path_to_error::deserialize(&mut de).map_err(|e| {
            let mut path = e
                .path()
                .to_string()
                .split('.')
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
            let inner = e.inner();

            let mut loc = vec!["body".to_owned()];
            loc.append(&mut path);
            HttpError::unprocessable_entity(vec![ValidationErrorItem {
                loc,
                msg: inner
                    .source()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| e.to_string()),
                ty: "value_error.jsondecode".to_owned(),
            }])
        })?;

        value.validate().map_err(|e| {
            HttpError::unprocessable_entity(validation_errors(vec!["body".to_owned()], e))
        })?;
        Ok(ValidatedJson(value))
    }
}

use std::error::Error as StdError;

use aide::OperationIo;
use axum::{
    RequestExt as _,
    extract::{
        FromRequest, Request,
        rejection::{BytesRejection, FailedToBufferBody},
    },
};
use bytes::Bytes;
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

    async fn from_request(req: Request, _: &S) -> Result<Self> {
        // Extracted into separate fns to avoid separate monomorphization for each value of T.
        fn map_bytes_error(e: BytesRejection) -> HttpError {
            match e {
                BytesRejection::FailedToBufferBody(FailedToBufferBody::LengthLimitError(_)) => {
                    HttpError::too_large(None, None)
                }

                _ => {
                    tracing::error!("Error reading body as bytes: {e}");
                    HttpError::internal_server_error(
                        None,
                        Some("Failed to read request body".to_owned()),
                    )
                }
            }
        }
        fn make_serde_error(e: serde_path_to_error::Error<serde_json::Error>) -> HttpError {
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
        }
        fn make_validation_error(e: validator::ValidationErrors) -> HttpError {
            HttpError::unprocessable_entity(validation_errors(vec!["body".to_owned()], e))
        }

        let b: Bytes = req.extract().await.map_err(map_bytes_error)?;
        let mut de = serde_json::Deserializer::from_slice(&b);
        let value: T = serde_path_to_error::deserialize(&mut de).map_err(make_serde_error)?;
        value.validate().map_err(make_validation_error)?;
        Ok(ValidatedJson(value))
    }
}

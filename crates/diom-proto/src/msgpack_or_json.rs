use aide::OperationIo;
use axum::{
    RequestExt as _,
    extract::{
        FromRequest, Request,
        rejection::{BytesRejection, FailedToBufferBody},
    },
};
use bytes::Bytes;
use diom_error::{Error, HttpError, Result, ValidationErrorItem, validation_errors};
use http::{HeaderMap, header};
use serde::de::DeserializeOwned;
use validator::Validate;

/// MsgPack-or-JSON extractor.
///
/// Validates incoming bodies using the [`Validate`] trait.
#[derive(Debug, Clone, Copy, Default, OperationIo)]
#[aide(input_with = "axum::extract::Json<T>", json_schema)] // FIXME: Also document MsgPack
pub struct MsgPackOrJson<T>(pub T);

impl<T, S> FromRequest<S> for MsgPackOrJson<T>
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
        fn make_serde_error(e: serde_path_to_error::Error<impl std::error::Error>) -> HttpError {
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

        let content_type = classify_content_type(req.headers())?;
        let b: Bytes = req.extract().await.map_err(map_bytes_error)?;
        let value: T = match content_type {
            SupportedContentType::MsgPack => {
                let mut de = rmp_serde::Deserializer::from_read_ref(&b);
                serde_path_to_error::deserialize(&mut de).map_err(make_serde_error)?
            }
            SupportedContentType::Json => {
                let mut de = serde_json::Deserializer::from_slice(&b);
                serde_path_to_error::deserialize(&mut de).map_err(make_serde_error)?
            }
        };
        value.validate().map_err(make_validation_error)?;
        Ok(MsgPackOrJson(value))
    }
}

enum SupportedContentType {
    MsgPack,
    Json,
}

fn classify_content_type(headers: &HeaderMap) -> Result<SupportedContentType, HttpError> {
    fn content_type_error(code: &str) -> HttpError {
        HttpError::bad_request(
            Some(code.to_owned()),
            Some(
                "Expected request with `content-type: application/msgpack` \
                 or `content-type: application/json`"
                    .to_owned(),
            ),
        )
    }

    let content_type = headers
        .get(header::CONTENT_TYPE)
        .ok_or_else(|| content_type_error("missing_content_type"))?;
    let content_type: mime::Mime = content_type
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| content_type_error("invalid_content_type"))?;
    if content_type.type_() != "application" {
        return Err(content_type_error("invalid_content_type"));
    }
    match content_type.subtype().as_str() {
        "msgpack" => Ok(SupportedContentType::MsgPack),
        "json" => Ok(SupportedContentType::Json),
        _ => Err(content_type_error("invalid_content_type")),
    }
}

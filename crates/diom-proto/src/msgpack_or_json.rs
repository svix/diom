#![expect(clippy::disallowed_types)] // FIXME: currently triggers due to input_with / output_with

use std::fmt;

use aide::OperationIo;
use axum::{
    RequestExt as _,
    extract::{
        FromRequest, Request,
        rejection::{BytesRejection, FailedToBufferBody},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use bytes::{BufMut as _, Bytes, BytesMut};
use http::{HeaderMap, HeaderValue, StatusCode, header};
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use crate::{StandardErrorBody, ValidationErrorBody, ValidationErrorItem, validation_errors};

tokio::task_local! {
    static RESPONSE_CONTENT_TYPE: SupportedContentType;
}

/// Middleware that captures the `accept` header on incoming requests and makes it available for
/// [`MsgPackOrJson`]'s `IntoResponse` implementation.
pub fn capture_accept_hdr(request: Request, next: Next) -> impl Future<Output = Response> {
    let headers = request.headers();
    let accept = headers.get(header::ACCEPT);

    let accept_msgpack = accept.is_some_and(|hdr_val| {
        hdr_val
            .as_bytes()
            .split(|&b| b == b',')
            // FIXME: Does not support q-values
            .any(|accept| accept == mime::APPLICATION_MSGPACK.as_ref().as_bytes())
    });

    // FIXME: What to do if accept is set but contains neither msgpack or JSON? Return an error?
    let res_content_type = if accept_msgpack
        || accept.is_none()
            && headers
                .get(header::CONTENT_TYPE)
                .is_some_and(|hdr_val| hdr_val == mime::APPLICATION_MSGPACK.as_ref().as_bytes())
    {
        // If explicitly accepted, or no accept header is specified
        // but request content-type is msgpack, use msgpack
        SupportedContentType::MsgPack
    } else {
        // Otherwise, use JSON
        SupportedContentType::Json
    };

    RESPONSE_CONTENT_TYPE.scope(res_content_type, next.run(request))
}

/// MsgPack-or-JSON extractor.
///
/// Validates incoming bodies using the [`Validate`] trait.
#[derive(Debug, Clone, Copy, Default, OperationIo)]
#[aide(
    // FIXME: Also document MsgPack
    input_with = "axum::Json<T>",
    output_with = "axum::Json<T>",
    json_schema
)]
pub struct MsgPackOrJson<T>(pub T);

impl<T, S> FromRequest<S> for MsgPackOrJson<T>
where
    // FIXME(@svix-jplatte): extra bound commented out to avoid merge issues
    T: DeserializeOwned + Validate, // + RequestInput,
    S: Send + Sync,
{
    type Rejection = MsgPackOrJsonRejection;

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        // Extracted into separate fns to avoid separate monomorphization for each value of T.
        fn map_bytes_error(e: BytesRejection) -> MsgPackOrJsonRejection {
            match e {
                BytesRejection::FailedToBufferBody(FailedToBufferBody::LengthLimitError(_)) => {
                    MsgPackOrJsonRejection::PayloadTooLarge
                }

                _ => {
                    tracing::error!("Error reading body as bytes: {e}");
                    MsgPackOrJsonRejection::InternalServerError {
                        msg: "Failed to read request body".to_owned(),
                    }
                }
            }
        }

        fn make_serde_error(
            e: serde_path_to_error::Error<impl std::error::Error>,
        ) -> MsgPackOrJsonRejection {
            let mut path = e
                .path()
                .to_string()
                .split('.')
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
            let inner = e.inner();

            let mut loc = vec!["body".to_owned()];
            loc.append(&mut path);
            MsgPackOrJsonRejection::Validation {
                errors: vec![ValidationErrorItem {
                    loc,
                    msg: inner
                        .source()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| e.to_string()),
                    ty: "value_error.jsondecode".to_owned(),
                }],
            }
        }

        fn make_validation_error(e: validator::ValidationErrors) -> MsgPackOrJsonRejection {
            MsgPackOrJsonRejection::Validation {
                errors: validation_errors(vec!["body".to_owned()], e),
            }
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

impl<T> IntoResponse for MsgPackOrJson<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // Extracted into separate fn so it's only compiled once for all T.
        fn make_response(
            buf: BytesMut,
            content_type: &'static mime::Mime,
            ser_result: Result<(), Box<dyn std::error::Error + Send + Sync>>,
        ) -> Response {
            match ser_result {
                Ok(()) => (
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(content_type.as_ref()),
                    )],
                    buf.freeze(),
                )
                    .into_response(),
                Err(err) => {
                    tracing::error!(err, "response serialization failed");
                    internal_server_error("response serialization error".to_owned())
                }
            }
        }

        fn box_error<E>(e: E) -> Box<dyn std::error::Error + Send + Sync>
        where
            E: std::error::Error + Send + Sync + 'static,
        {
            Box::new(e)
        }

        let res_content_type = RESPONSE_CONTENT_TYPE.try_get().unwrap_or_else(|_| {
            tracing::error!(
                "MsgPackOrJson used as response without capture_accept_hdr, falling back to JSON"
            );
            SupportedContentType::Json
        });

        let mut buf = BytesMut::with_capacity(128).writer();
        let (content_type, res) = match res_content_type {
            SupportedContentType::MsgPack => {
                let mut serializer = rmp_serde::Serializer::new(&mut buf)
                    .with_struct_map()
                    .with_bytes(rmp_serde::config::BytesMode::ForceAll);
                let serialize_result = self.0.serialize(&mut serializer).map_err(box_error);

                (&mime::APPLICATION_MSGPACK, serialize_result)
            }
            SupportedContentType::Json => {
                let serialize_result = serde_json::to_writer(&mut buf, &self.0).map_err(box_error);

                (&mime::APPLICATION_JSON, serialize_result)
            }
        };
        make_response(buf.into_inner(), content_type, res)
    }
}

#[derive(Clone, Copy)]
enum SupportedContentType {
    MsgPack,
    Json,
}

fn classify_content_type(
    headers: &HeaderMap,
) -> Result<SupportedContentType, MsgPackOrJsonRejection> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .ok_or_else(|| MsgPackOrJsonRejection::content_type("missing_content_type"))?;

    let content_type: mime::Mime = content_type
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| MsgPackOrJsonRejection::content_type("invalid_content_type"))?;

    if content_type.type_() != "application" {
        return Err(MsgPackOrJsonRejection::content_type("invalid_content_type"));
    }

    match content_type.subtype().as_str() {
        "msgpack" => Ok(SupportedContentType::MsgPack),
        "json" => Ok(SupportedContentType::Json),
        _ => Err(MsgPackOrJsonRejection::content_type("invalid_content_type")),
    }
}

pub enum MsgPackOrJsonRejection {
    PayloadTooLarge,
    InternalServerError { msg: String },
    ContentType { code: &'static str },
    Validation { errors: Vec<ValidationErrorItem> },
}

impl MsgPackOrJsonRejection {
    fn content_type(code: &'static str) -> Self {
        Self::ContentType { code }
    }
}

impl IntoResponse for MsgPackOrJsonRejection {
    fn into_response(self) -> Response {
        match self {
            Self::PayloadTooLarge => standard_error_response(
                StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
                "Request payload is too large.",
            ),
            Self::InternalServerError { msg } => internal_server_error(msg),
            Self::ContentType { code } => standard_error_response(
                StatusCode::BAD_REQUEST,
                code,
                "Expected request with `content-type: application/msgpack` \
                 or `content-type: application/json`"
                    .to_owned(),
            ),
            Self::Validation { errors } => error_response(
                StatusCode::UNPROCESSABLE_ENTITY,
                ValidationErrorBody::new(errors),
            ),
        }
    }
}

fn internal_server_error(msg: String) -> Response {
    standard_error_response(StatusCode::INTERNAL_SERVER_ERROR, "server_error", msg)
}

fn standard_error_response(
    status_code: StatusCode,
    code: &'static str,
    detail: impl fmt::Display,
) -> Response {
    error_response(status_code, StandardErrorBody::new(code, detail))
}

fn error_response<T: Serialize>(status_code: StatusCode, body: T) -> Response {
    (status_code, MsgPackOrJson(body)).into_response()
}

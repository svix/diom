#![expect(clippy::disallowed_types)] // FIXME: currently triggers due to input_with / output_with

use aide::OperationIo;
use axum::{
    RequestExt as _,
    extract::{FromRequest, Request},
};
use bytes::Buf as _;
use diom_authorization::{Permissions, verify_operation};
use diom_core::{
    types::Yoke,
    validation::{ValidationErrorItem, validation_errors},
};
use http_body_util::BodyExt as _;
use serde::Deserialize;
use validator::Validate;
use yoke::Yokeable;

use crate::{
    RequestInput,
    msgpack_or_json::{MsgPackOrJsonRejection, SupportedContentType, classify_content_type},
};

/// MsgPack-or-JSON extractor.
///
/// Validates incoming bodies using the [`Validate`] trait.
#[derive(OperationIo)]
#[aide(
    // FIXME: Also document MsgPack
    input_with = "axum::Json<T>",
    output_with = "axum::Json<T>",
    json_schema
)]
pub struct MsgPackOrJson2<T: for<'a> Yokeable<'a>>(pub Yoke<T>);

impl<T: for<'a> Yokeable<'a>> MsgPackOrJson2<T> {
    pub fn get(&self) -> &<T as Yokeable<'_>>::Output {
        self.0.get()
    }
}

impl<T, S> FromRequest<S> for MsgPackOrJson2<T>
where
    T: for<'a> Yokeable<'a, Output: Deserialize<'a> + Validate + RequestInput>,
    S: Send + Sync,
{
    type Rejection = MsgPackOrJsonRejection;

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        // Extracted into separate fns to avoid separate monomorphization for each value of T.
        fn map_body_error(err: axum::Error) -> MsgPackOrJsonRejection {
            // Copied from axum: https://docs.rs/axum-core/0.5.5/src/axum_core/extract/rejection.rs.html#17
            let box_error = match err.into_inner().downcast::<axum::Error>() {
                Ok(err) => err.into_inner(),
                Err(err) => err,
            };
            let box_error = match box_error.downcast::<axum::Error>() {
                Ok(err) => err.into_inner(),
                Err(err) => err,
            };
            match box_error.downcast::<http_body_util::LengthLimitError>() {
                Ok(_) => MsgPackOrJsonRejection::PayloadTooLarge,
                Err(err) => {
                    tracing::error!("Error reading body as bytes: {err}");
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

        let permissions = req.extensions().get::<Permissions>().cloned();

        let content_type = classify_content_type(req.headers())?;
        let mut body_buf = req
            .into_limited_body()
            .collect()
            .await
            .map_err(map_body_error)?
            .aggregate();
        let mut body_bytes = vec![0; body_buf.remaining()];
        body_buf.copy_to_slice(&mut body_bytes);

        let yoke: Yoke<T, Vec<u8>> = match content_type {
            SupportedContentType::MsgPack => Yoke::try_attach_to_cart(body_bytes, |b| {
                let mut de = rmp_serde::Deserializer::from_read_ref(b);
                serde_path_to_error::deserialize(&mut de).map_err(make_serde_error)
            })?,
            SupportedContentType::Json => Yoke::try_attach_to_cart(body_bytes, |b| {
                let mut de = serde_json::Deserializer::from_slice(b);
                serde_path_to_error::deserialize(&mut de).map_err(make_serde_error)
            })?,
        };
        let value = yoke.get();

        value.validate().map_err(make_validation_error)?;

        if let Some(op) = value.operation()
            && let Some(perms) = permissions
            && verify_operation(&op, &perms.access_rules).is_err()
        {
            return Err(MsgPackOrJsonRejection::Forbidden {
                resource: op.resource_str(),
                action: op.action,
            });
        }

        Ok(MsgPackOrJson2(yoke))
    }
}

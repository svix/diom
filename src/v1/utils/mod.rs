// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    borrow::Cow,
    error::Error as StdError,
    ops::Deref,
    time::{SystemTime, UNIX_EPOCH},
};

use aide::{
    OperationInput, OperationIo, OperationOutput,
    openapi::StatusCode as OpenApiStatusCode,
    transform::{TransformOperation, TransformPathItem},
};
use axum::{
    extract::{
        FromRequest, FromRequestParts, Query, Request,
        rejection::{BytesRejection, FailedToBufferBody},
    },
    response::IntoResponse,
};
use http::request::Parts;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use validator::{Validate, ValidationError};

use crate::error::{Error, HttpError, Result, ValidationErrorItem};

pub mod proto;

// Helper method to simplify the somewhat egregious API for creating a ValidationError
pub fn validation_error(code: Option<&'static str>, msg: Option<&'static str>) -> ValidationError {
    ValidationError {
        code: std::borrow::Cow::from(code.unwrap_or("validation")),
        message: msg.map(std::borrow::Cow::from),
        params: std::collections::HashMap::new(),
    }
}

/// Recursively searches a [`validator::ValidationErrors`] tree into a linear list of errors to be
/// sent to the user
pub fn validation_errors(
    acc_path: Vec<String>,
    err: validator::ValidationErrors,
) -> Vec<ValidationErrorItem> {
    err.into_errors()
        .into_iter()
        .flat_map(|(k, v)| {
            // Add the next field to the location
            let mut loc = acc_path.clone();
            loc.push(k.into());

            match v {
                // If it's a [`validator::ValidationErrorsKind::Field`], then it will be a vector of
                // errors to map to [`ValidationErrorItem`]s and insert to [`out`] before the next
                // iteration
                validator::ValidationErrorsKind::Field(vec) => vec
                    .into_iter()
                    .map(|err| ValidationErrorItem {
                        loc: loc.clone(),
                        msg: err
                            .message
                            .unwrap_or(Cow::Borrowed("Validation error"))
                            .to_string(),
                        ty: "value_error".to_owned(),
                    })
                    .collect(),
                // If it is a [`validator::ValidationErrorsKind::Struct`], then it will be another
                // [`validator::ValidationErrors`] to search
                validator::ValidationErrorsKind::Struct(errors) => validation_errors(loc, *errors),

                // If it is a [`validator::ValidationErrorsKind::List`], then it will be an
                // [`std::collections::BTreeMap`] of [`validator::ValidationErrors`] to search
                validator::ValidationErrorsKind::List(map) => map
                    .into_iter()
                    .flat_map(|(k, v)| {
                        // Add the list index to the location
                        let mut loc = loc.clone();
                        loc.push(format!("[{k}]"));

                        validation_errors(loc, *v)
                    })
                    .collect(),
            }
        })
        .collect()
}

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

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|err| HttpError::bad_request(None, Some(err.to_string())))?;
        value.validate().map_err(|e| {
            HttpError::unprocessable_entity(validation_errors(vec!["query".to_owned()], e))
        })?;
        Ok(ValidatedQuery(value))
    }
}

impl<T> Deref for ValidatedQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: JsonSchema> OperationInput for ValidatedQuery<T> {
    fn operation_input(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) {
        axum::extract::Query::<T>::operation_input(ctx, operation)
    }
}

pub async fn api_not_implemented() -> Result<()> {
    Err(HttpError::not_implemented(None, None).into())
}

pub fn validate_no_control_characters(str: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"[\x00-\x08]").unwrap();
    if re.is_match(str) {
        return Err(validation_error(
            Some("illegal_character"),
            Some("Control characters 0x00-0x08 not allowed."),
        ));
    }
    Ok(())
}

pub fn openapi_tag<T: AsRef<str>>(
    tag: T,
) -> impl Fn(TransformPathItem<'_>) -> TransformPathItem<'_> {
    move |op| op.tag(tag.as_ref())
}

pub fn openapi_desc<T: AsRef<str>>(
    desc: T,
) -> impl Fn(TransformOperation<'_>) -> TransformOperation<'_> {
    move |op| op.description(desc.as_ref())
}

pub fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// JsonStatus is a wrapper over `axum::extract::Json` as a handler output.
///
/// Setting the `STATUS` const parameter automatically sets the response
/// status code, as well as inserting it into the aide documentation.
pub struct JsonStatus<const STATUS: u16, T: JsonSchema + Serialize>(pub T);

impl<const STATUS: u16, T: JsonSchema + Serialize> IntoResponse for JsonStatus<STATUS, T> {
    fn into_response(self) -> axum::response::Response {
        (
            http::StatusCode::from_u16(STATUS).unwrap(),
            axum::extract::Json(self.0),
        )
            .into_response()
    }
}

impl<const STATUS: u16, T: JsonSchema + Serialize> OperationOutput for JsonStatus<STATUS, T> {
    type Inner = T;

    fn operation_response(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        axum::extract::Json::<T>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<OpenApiStatusCode>, aide::openapi::Response)> {
        if let Some(resp) = Self::operation_response(ctx, operation) {
            vec![(Some(OpenApiStatusCode::Code(STATUS)), resp)]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::{validate_no_control_characters, validation_errors};
    use crate::error::ValidationErrorItem;

    #[derive(Debug, Validate)]
    struct ValidationErrorTestStruct {
        #[validate(range(min = 10, message = "Below 10"))]
        a: u32,

        #[validate(nested)]
        b: ValidationErrorTestStructInner,

        #[validate(nested)]
        c: Vec<ValidationErrorTestStructInner>,
    }

    #[derive(Debug, Validate)]
    struct ValidationErrorTestStructInner {
        #[validate(range(max = 10, message = "Above 10"))]
        inner: u8,
    }

    #[test]
    fn test_validation_errors_fn() {
        let valid = ValidationErrorTestStruct {
            a: 11,
            b: ValidationErrorTestStructInner { inner: 1 },
            c: vec![
                ValidationErrorTestStructInner { inner: 2 },
                ValidationErrorTestStructInner { inner: 3 },
            ],
        };
        let invalid = ValidationErrorTestStruct {
            a: 9,
            b: ValidationErrorTestStructInner { inner: 11 },
            c: vec![
                ValidationErrorTestStructInner { inner: 12 },
                ValidationErrorTestStructInner { inner: 13 },
            ],
        };

        assert_eq!(valid.validate(), Ok(()));

        let errs = invalid.validate().unwrap_err();
        let errs = validation_errors(vec![], errs);

        assert_eq!(errs.len(), 4);

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["a".to_owned()],
            msg: "Below 10".to_owned(),
            ty: "value_error".to_owned(),
        }));

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["b".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["c".to_owned(), "[0]".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));
        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["c".to_owned(), "[1]".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));
    }

    #[test]
    fn test_validate_no_control_characters() {
        let a = "A good string";
        let b = "A\u{0000} bad string";

        assert!(validate_no_control_characters(a).is_ok());
        assert!(validate_no_control_characters(b).is_err());
    }
}

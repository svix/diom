use std::{borrow::Cow, fmt};

use schemars::JsonSchema;
use serde::Serialize;
use validator::ValidationError;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorBody {
    pub(crate) detail: Vec<ValidationErrorItem>,
}

impl ValidationErrorBody {
    pub fn new(detail: Vec<ValidationErrorItem>) -> Self {
        Self { detail }
    }
}

impl fmt::Display for ValidationErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "validation error {}",
            serde_json::to_string(&self.detail)
                .unwrap_or_else(|e| format!("\"unserializable error for {e}\""))
        )
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
/// Validation errors have their own schema to provide context for invalid requests eg. mismatched
/// types and out of bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY
/// error.
pub struct ValidationErrorItem {
    /// The location as a [`Vec`] of [`String`]s -- often in the form `["body", "field_name"]`,
    /// `["query", "field_name"]`, etc. They may, however, be arbitrarily deep.
    pub loc: Vec<String>,

    /// The message accompanying the validation error item.
    pub msg: String,

    /// The type of error, often "type_error" or "value_error", but sometimes with more context like
    /// as "value_error.number.not_ge"
    #[serde(rename = "type")]
    pub ty: String,
}

/// Helper function to simplify the somewhat egregious API for creating a ValidationError
pub fn validation_error(code: Option<&'static str>, msg: Option<&'static str>) -> ValidationError {
    ValidationError {
        code: Cow::from(code.unwrap_or("validation")),
        message: msg.map(Cow::from),
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

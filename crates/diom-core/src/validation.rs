use std::fmt;

use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, JsonSchema)]
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

    /// The type of error.
    ///
    /// Often "type_error" or "value_error", but sometimes with more context like
    /// "value_error.number.not_ge".
    #[serde(rename = "type")]
    pub ty: String,
}

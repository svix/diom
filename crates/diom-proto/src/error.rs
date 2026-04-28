use std::{borrow::Cow, fmt};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    InvalidInput,
    OperationError,
    ServerError,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ErrorBody {
    #[serde(rename = "type")]
    pub type_: ErrorType,
    pub code: Cow<'static, str>,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl ErrorBody {
    fn new(type_: ErrorType, code: Cow<'static, str>, detail: String) -> Self {
        Self {
            type_,
            code,
            detail,
            location: None,
        }
    }

    pub fn invalid_input(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::new(ErrorType::InvalidInput, code.into(), detail.to_string())
    }

    pub fn operation_error(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::new(ErrorType::OperationError, code.into(), detail.to_string())
    }

    pub fn server_error(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::new(ErrorType::ServerError, code.into(), detail.to_string())
    }

    pub fn from_raft(type_: ErrorType, code: Cow<'static, str>, detail: String) -> Self {
        Self::new(type_, code, detail)
    }

    pub fn with_location(mut self, location: impl Into<Option<String>>) -> Self {
        self.location = location.into();
        self
    }
}

impl fmt::Display for ErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            // only printed as part of diom_error::Error,
            // whose error variant already covers the type
            type_: _,
            code,
            detail,
            location,
        } = self;
        write!(f, "code={code:?} detail={detail:?} location={location:?}")
    }
}

use std::{borrow::Cow, fmt};

use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct StandardErrorBody {
    pub code: Cow<'static, str>,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl StandardErrorBody {
    pub fn new(code: &'static str, detail: impl fmt::Display) -> Self {
        Self {
            code: code.into(),
            detail: detail.to_string(),
            location: None,
        }
    }

    pub fn from_raft(code: Cow<'static, str>, detail: String) -> Self {
        Self {
            code,
            detail,
            location: None,
        }
    }

    pub fn with_location(mut self, location: impl Into<Option<String>>) -> Self {
        self.location = location.into();
        self
    }
}

impl fmt::Display for StandardErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            code,
            detail,
            location,
        } = self;
        write!(f, "code={code:?} detail={detail:?} location={location:?}")
    }
}

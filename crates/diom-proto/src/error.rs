use std::fmt;

use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct StandardErrorBody {
    code: &'static str,
    detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
}

impl StandardErrorBody {
    pub fn new(code: &'static str, detail: impl fmt::Display) -> Self {
        Self {
            code,
            detail: detail.to_string(),
            location: None,
        }
    }

    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
}

impl StandardErrorBody {
    pub fn code(&self) -> &str {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
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

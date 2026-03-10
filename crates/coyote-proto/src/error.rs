use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StandardErrorBody {
    code: &'static str,
    detail: String,
}

impl StandardErrorBody {
    pub fn new(code: &'static str, detail: impl fmt::Display) -> Self {
        Self {
            code,
            detail: detail.to_string(),
        }
    }
}

impl fmt::Display for StandardErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { code, detail } = self;
        write!(f, "code={code:?} detail={detail:?}")
    }
}

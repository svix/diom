// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "post")]
    Post,
    #[serde(rename = "put")]
    Put,
    #[serde(rename = "patch")]
    Patch,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Post => "post",
            Self::Put => "put",
            Self::Patch => "patch",
        };
        f.write_str(value)
    }
}

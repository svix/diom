// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenOut {
    pub id: String,

    pub name: String,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub created: jiff::Timestamp,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub updated: jiff::Timestamp,

    #[serde(
        with = "crate::unix_timestamp_ms_serde::optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry: Option<jiff::Timestamp>,

    pub role: String,

    /// Whether this token is currently enabled.
    pub enabled: bool,

    /// Whether this token has expired.
    ///
    /// Expired tokens may be pruned in the background at any time.
    pub expired: bool,
}

impl AdminAuthTokenOut {
    pub fn new(
        id: String,
        name: String,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
        role: String,
        enabled: bool,
        expired: bool,
    ) -> Self {
        Self {
            id,
            name,
            created,
            updated,
            expiry: None,
            role,
            enabled,
            expired,
        }
    }

    pub fn with_expiry(mut self, value: impl Into<Option<jiff::Timestamp>>) -> Self {
        self.expiry = value.into();
        self
    }
}

// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CacheGetIn {
    /// Whether or not the read should be linearizable
    ///
    /// If this is `true`, the read is guaranteed to see all previous operations, but will
    /// have to make at least one additional round-trip to the leader. If this is false, stale
    /// reads will be performed against the replica which receives this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearizable: Option<bool>,
}

impl CacheGetIn {
    pub fn new() -> Self {
        Self { linearizable: None }
    }

    pub fn with_linearizable(mut self, value: impl Into<Option<bool>>) -> Self {
        self.linearizable = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CacheGetIn_ {
    pub key: String,

    /// Whether or not the read should be linearizable
    ///
    /// If this is `true`, the read is guaranteed to see all previous operations, but will
    /// have to make at least one additional round-trip to the leader. If this is false, stale
    /// reads will be performed against the replica which receives this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearizable: Option<bool>,
}

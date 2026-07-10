// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvixPollerDeleteIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    pub poller_id: String,
}

impl SvixPollerDeleteIn {
    pub fn new(topic: String, poller_id: String) -> Self {
        Self {
            namespace: None,
            topic,
            poller_id,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}

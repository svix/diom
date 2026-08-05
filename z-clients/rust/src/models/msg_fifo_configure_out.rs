// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgFifoConfigureOut {
    pub retry_schedule: Vec<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_topic: Option<String>,
}

impl MsgFifoConfigureOut {
    pub fn new(retry_schedule: Vec<u64>) -> Self {
        Self {
            retry_schedule,
            dlq_topic: None,
        }
    }

    pub fn with_dlq_topic(mut self, value: impl Into<Option<String>>) -> Self {
        self.dlq_topic = value.into();
        self
    }
}

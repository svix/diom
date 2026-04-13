// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[derive(Clone, Debug, Deserialize)]
pub struct MsgPublishIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub msgs: Vec<MsgIn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl MsgPublishIn {
    pub fn new(msgs: Vec<MsgIn>) -> Self {
        Self {
            namespace: None,
            msgs,
            idempotency_key: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_idempotency_key(mut self, value: impl Into<Option<String>>) -> Self {
        self.idempotency_key = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgPublishIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    pub msgs: Vec<MsgIn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClusterForceElectionOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_leader_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_leader_id: Option<String>,
}

impl ClusterForceElectionOut {
    pub fn new() -> Self {
        Self {
            previous_leader_id: None,
            new_leader_id: None,
        }
    }

    pub fn with_previous_leader_id(mut self, value: impl Into<Option<String>>) -> Self {
        self.previous_leader_id = value.into();
        self
    }

    pub fn with_new_leader_id(mut self, value: impl Into<Option<String>>) -> Self {
        self.new_leader_id = value.into();
        self
    }
}

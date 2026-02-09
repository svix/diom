use std::collections::HashMap;

use coyote_configgroup::entities::ConfigGroupName;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

// FIXME(@svix-gabriel) - I opted for type aliases here just for expediency.
// We absolutely can (and should) use more robust types.
pub type StreamName = ConfigGroupName;
pub type MsgId = u64;
pub type ConsumerGroup = String;
pub type MsgHeaders = HashMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct MsgIn {
    pub payload: Vec<u8>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct MsgOut {
    pub id: MsgId,
    pub payload: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

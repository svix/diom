use std::collections::HashMap;

use uuid::Uuid;

// FIXME(@svix-gabriel) - I opted for type aliases here just for expediency.
// We absolutely can (and should) use more robust types.
pub type StreamId = Uuid;
pub type MsgId = u64;
pub type ConsumerGroup = String;
pub type MsgHeaders = HashMap<String, String>;

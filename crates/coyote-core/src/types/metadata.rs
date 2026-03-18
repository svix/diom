use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, JsonSchema)]
#[schemars(transparent)]
pub struct Metadata(pub HashMap<String, String>);

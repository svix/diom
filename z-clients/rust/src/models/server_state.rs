// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ServerState {
    #[serde(rename = "leader")]
    Leader,
    #[serde(rename = "follower")]
    Follower,
    #[serde(rename = "learner")]
    Learner,
    #[serde(rename = "candidate")]
    Candidate,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "unknown")]
    Unknown,
}

impl fmt::Display for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Leader => "leader",
            Self::Follower => "follower",
            Self::Learner => "learner",
            Self::Candidate => "candidate",
            Self::Shutdown => "shutdown",
            Self::Unknown => "unknown",
        };
        f.write_str(value)
    }
}

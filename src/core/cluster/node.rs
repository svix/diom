use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Node {
    #[default]
    NoAddress,
    SingleHomed(SocketAddr),
}

impl Node {
    pub fn new(s: SocketAddr) -> Self {
        Self::SingleHomed(s)
    }

    pub fn addrs(&self) -> Vec<SocketAddr> {
        match self {
            Self::NoAddress => vec![],
            Self::SingleHomed(s) => vec![*s],
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct NodeId {
    #[serde(with = "uuid::serde::simple")]
    inner: Uuid,
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl NodeId {
    pub(super) fn generate() -> Self {
        Self {
            inner: Uuid::new_v4(),
        }
    }
}

#[cfg(test)]
impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        let inner = Uuid::from_u64_pair(value, value);
        Self { inner }
    }
}

use anyhow::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tap::Pipe;
use url::Url;
use uuid::Uuid;

use crate::cfg::PeerAddr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, JsonSchema)]
pub enum Node {
    #[default]
    NoAddress,
    SingleHomed(SocketAddr),
    HostnameAndPort {
        hostname: String,
        port: u16,
    },
}

impl From<PeerAddr> for Node {
    fn from(value: PeerAddr) -> Self {
        match value {
            PeerAddr::SocketAddr(socket_addr) => Self::SingleHomed(socket_addr),
            PeerAddr::HostnameAndPort { hostname, port } => {
                Self::HostnameAndPort { hostname, port }
            }
        }
    }
}

impl TryFrom<Node> for PeerAddr {
    type Error = anyhow::Error;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        match value {
            Node::NoAddress => anyhow::bail!("cannot construct peer address from unaddressed node"),
            Node::SingleHomed(sa) => PeerAddr::SocketAddr(sa),
            Node::HostnameAndPort { hostname, port } => {
                PeerAddr::HostnameAndPort { hostname, port }
            }
        }
        .pipe(Ok)
    }
}

impl Node {
    pub fn new(s: SocketAddr) -> Self {
        Self::SingleHomed(s)
    }

    pub fn names(&self) -> Vec<String> {
        match self {
            Self::NoAddress => vec![],
            Self::SingleHomed(addr) => vec![addr.to_string()],
            Self::HostnameAndPort { hostname, port } => vec![format!("{hostname}:{port}")],
        }
    }

    pub fn url_for(&self, path: &str) -> anyhow::Result<Url> {
        let base = match self {
            Self::NoAddress => anyhow::bail!("no address provided"),
            Self::SingleHomed(sa) => sa.to_string(),
            Self::HostnameAndPort { hostname, port } => format!("{hostname}:{port}"),
        };
        format!("http://{base}/{path}")
            .parse()
            .context("generating url for network RPC")
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

impl JsonSchema for NodeId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.simple().fmt(f)
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

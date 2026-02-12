use crate::{cfg::Configuration, core::cluster::state_machine::StoreHandle};

use super::{
    discovery::Discovery,
    operations::{InternalRequest, InternalResponse},
    raft::{Node, NodeId, Raft},
};
use diom_operations::{OperationRequest, OperationResponse};
use openraft::error::{ClientWriteError, RaftError};
use serde::{Deserialize, Serialize};

type WriteResult<T> = Result<T, RaftError<NodeId, ClientWriteError<NodeId, Node>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseParseError {
    InvalidVariant,
}

impl std::fmt::Display for ResponseParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ResponseParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Invalid response from consensus system"
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Request {
    ClusterInternal(InternalRequest),
    Kv(diom_kv::operations::KvOperation),
    RateLimiter(diom_rate_limiter::operations::RateLimiterOperation),
}

impl From<diom_kv::operations::KvOperation> for Request {
    fn from(value: diom_kv::operations::KvOperation) -> Self {
        Request::Kv(value)
    }
}

impl From<diom_rate_limiter::operations::RateLimiterOperation> for Request {
    fn from(value: diom_rate_limiter::operations::RateLimiterOperation) -> Self {
        Request::RateLimiter(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Blank,
    ClusterInternal(InternalResponse),
    Kv(diom_kv::operations::Response),
    RateLimiter(diom_rate_limiter::operations::Response),
}

impl TryFrom<Response> for diom_kv::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Kv(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for diom_rate_limiter::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::RateLimiter(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

#[derive(Clone)]
pub struct RaftState {
    pub raft: Raft,
    pub node_id: NodeId,
    pub state_machine: StoreHandle,
}

impl RaftState {
    /// Write a single operation into the Raft log and return its response.
    pub async fn client_write<O>(&self, op: O) -> WriteResult<O::Response>
    where
        O: OperationRequest + Into<O::RequestParent>,
        O::RequestParent: Into<Request>,
        <<O as OperationRequest>::Response as OperationResponse>::ResponseParent: TryFrom<Response>,
        <<<O as OperationRequest>::Response as OperationResponse>::ResponseParent as TryFrom<
            Response,
        >>::Error: std::fmt::Debug,
        <<<O as OperationRequest>::Response as OperationResponse>::ResponseParent as TryInto<
            <O as OperationRequest>::Response,
        >>::Error: std::fmt::Debug,
    {
        let request = op.into().into();
        let response = self.raft.client_write(request).await?;
        let module_response =
            <<O as OperationRequest>::Response as OperationResponse>::ResponseParent::try_from(
                response.data,
            )
            .expect("raft response should be convertible into module response type");
        let resp = module_response
            .try_into()
            .expect("module response should be convertible into target type");
        Ok(resp)
    }

    pub async fn run_discovery_if_necessary(&self, cfg: Configuration) -> anyhow::Result<()> {
        let has_cluster = self
            .raft
            .with_raft_state(|s| {
                s.committed.is_some() || s.membership_state.effective().nodes().count() > 0
            })
            .await?;
        if has_cluster {
            tracing::debug!("node already has cluster information; skipping discovery");
        } else {
            tracing::debug!("node has no cluster information; kicking off discovery");
            let disco = Discovery::new(cfg, self.raft.clone(), self.node_id)?;
            if let Err(err) = disco.discover_cluster().await {
                tracing::error!(
                    ?err,
                    "discovery failed; this node must be manually initialized"
                );
            }
            tracing::info!("discovery succeeded");
        }
        Ok(())
    }
}

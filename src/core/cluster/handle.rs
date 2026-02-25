use crate::{cfg::Configuration, core::cluster::state_machine::StoreHandle};

use super::{
    discovery::Discovery,
    network::NetworkFactory,
    operations::{InternalRequest, InternalResponse},
    raft::{NodeId, Raft},
};
use anyhow::{Context, Result};
use coyote_operations::{OperationRequest, OperationResponse};
use openraft::RaftNetworkFactory;
use serde::{Deserialize, Serialize};

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
    Kv(coyote_kv::operations::KvOperation),
    CreateKv(coyote_kv::operations::CreateKvOp),
    RateLimiter(coyote_rate_limiter::operations::RateLimiterOperation),
    Idempotency(coyote_idempotency::operations::IdempotencyOperation),
    Cache(coyote_cache::operations::CacheOperation),
    CreateCache(coyote_cache::operations::CreateCacheOp),
    CreateIdempotency(coyote_idempotency::operations::CreateIdempotencyOp),
    Stream(stream_deprecated::operations::StreamOperation),
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Request::ClusterInternal(_) => write!(f, "cluster_internal"),
            Request::Kv(_) => write!(f, "kv"),
            Request::CreateKv(_) => write!(f, "create_kv"),
            Request::RateLimiter(_) => write!(f, "ratelimiter"),
            Request::Idempotency(_) => write!(f, "idempotency"),
            Request::Cache(_) => write!(f, "cache"),
            Request::CreateCache(_) => write!(f, "create_cache"),
            Request::CreateIdempotency(_) => write!(f, "create_idempotency"),
            Request::Stream(_) => write!(f, "stream"),
        }
    }
}

impl From<coyote_kv::operations::KvOperation> for Request {
    fn from(value: coyote_kv::operations::KvOperation) -> Self {
        Request::Kv(value)
    }
}

impl From<coyote_rate_limiter::operations::RateLimiterOperation> for Request {
    fn from(value: coyote_rate_limiter::operations::RateLimiterOperation) -> Self {
        Request::RateLimiter(value)
    }
}

impl From<coyote_idempotency::operations::IdempotencyOperation> for Request {
    fn from(value: coyote_idempotency::operations::IdempotencyOperation) -> Self {
        Request::Idempotency(value)
    }
}

impl From<coyote_cache::operations::CacheOperation> for Request {
    fn from(value: coyote_cache::operations::CacheOperation) -> Self {
        Request::Cache(value)
    }
}

impl From<coyote_cache::operations::CreateCacheOp> for Request {
    fn from(value: coyote_cache::operations::CreateCacheOp) -> Self {
        Request::CreateCache(value)
    }
}

impl From<coyote_idempotency::operations::CreateIdempotencyOp> for Request {
    fn from(value: coyote_idempotency::operations::CreateIdempotencyOp) -> Self {
        Request::CreateIdempotency(value)
    }
}

impl From<coyote_kv::operations::CreateKvOp> for Request {
    fn from(value: coyote_kv::operations::CreateKvOp) -> Self {
        Request::CreateKv(value)
    }
}

impl From<stream_deprecated::operations::StreamOperation> for Request {
    fn from(value: stream_deprecated::operations::StreamOperation) -> Self {
        Request::Stream(value)
    }
}

impl From<super::operations::InternalRequest> for Request {
    fn from(value: super::operations::InternalRequest) -> Self {
        Self::ClusterInternal(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Blank,
    ClusterInternal(InternalResponse),
    CreateCache(coyote_cache::operations::CreateCacheOperationResponse),
    CreateIdempotency(coyote_idempotency::operations::CreateIdempotencyOperationResponse),
    CreateKv(coyote_kv::operations::CreateKvOperationResponse),
    Kv(coyote_kv::operations::Response),
    RateLimiter(coyote_rate_limiter::operations::Response),
    Idempotency(coyote_idempotency::operations::Response),
    Cache(coyote_cache::operations::Response),
    Stream(stream_deprecated::operations::Response),
}

impl TryFrom<Response> for coyote_kv::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Kv(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for coyote_rate_limiter::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::RateLimiter(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for coyote_idempotency::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Idempotency(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for coyote_cache::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Cache(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for coyote_cache::operations::CreateCacheOperationResponse {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::CreateCache(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for coyote_idempotency::operations::CreateIdempotencyOperationResponse {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::CreateIdempotency(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for coyote_kv::operations::CreateKvOperationResponse {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::CreateKv(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for stream_deprecated::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Stream(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for super::operations::InternalResponse {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::ClusterInternal(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

#[derive(Clone)]
pub struct RaftState {
    pub raft: Raft,
    pub node_id: NodeId,
    pub state_machine: StoreHandle,
    pub(super) network: NetworkFactory,
}

impl RaftState {
    /// Write a single operation into the Raft log and return its response.
    #[tracing::instrument(skip_all)]
    pub async fn client_write<O>(&self, op: O) -> anyhow::Result<O::Response>
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
        let response = match self.raft.client_write(request.clone()).await {
            Ok(resp) => {
                tracing::trace!(log_id=?resp.log_id(), "request applied to log");
                resp.data
            }
            Err(err) => {
                if let Some(forward_to_leader) = err.forward_to_leader() {
                    if let Some(leader_id) = forward_to_leader.leader_id
                        && let Some(leader_node) = &forward_to_leader.leader_node
                    {
                        tracing::debug!("received write to non-leader, forwarding");
                        let mut network_handle = self.network.clone();
                        let client = network_handle.new_client(leader_id, leader_node).await;
                        client
                            .forward_request::<openraft::AnyError>(
                                super::proto::ForwardedWriteRequest {
                                    source_node_id: self.node_id,
                                    request,
                                },
                            )
                            .await
                            .map(|r| r.response)
                            .map_err(|e| anyhow::anyhow!(e))?
                    } else {
                        tracing::error!(
                            "received write to non-leader, and I don't know who the leader is!"
                        );
                        anyhow::bail!("no leader");
                    }
                } else {
                    return Err(err.into());
                }
            }
        };
        let module_response =
            <<O as OperationRequest>::Response as OperationResponse>::ResponseParent::try_from(
                response,
            )
            .expect("raft response should be convertible into module response type");
        let resp = module_response
            .try_into()
            .expect("module response should be convertible into target type");
        Ok(resp)
    }

    pub async fn run_discovery_if_necessary(&self, cfg: Configuration) -> Result<()> {
        let has_cluster = self
            .raft
            .with_raft_state(|s| {
                s.committed.is_some() || s.membership_state.effective().nodes().count() > 0
            })
            .await
            .context("reading cluster state")?;
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

    pub async fn is_up(&self) -> bool {
        self.raft
            .with_raft_state(move |s| {
                s.server_state.is_leader()
                    || s.server_state.is_follower()
                    || s.server_state.is_candidate()
            })
            .await
            .unwrap_or(false)
    }
}

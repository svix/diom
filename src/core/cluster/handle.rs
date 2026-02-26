use std::fmt;

use crate::{cfg::Configuration, core::cluster::state_machine::StoreHandle};

use super::{
    NodeId,
    discovery::Discovery,
    network::NetworkFactory,
    operations::{InternalRequest, InternalResponse},
    raft::Raft,
};
use anyhow::Context;
use diom_operations::{OperationRequest, OperationResponse};
use openraft::RaftNetworkFactory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseParseError {
    InvalidVariant,
}

impl fmt::Display for ResponseParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    CreateKv(diom_kv::operations::CreateKvOp),
    RateLimiter(diom_rate_limiter::operations::RateLimiterOperation),
    Idempotency(diom_idempotency::operations::IdempotencyOperation),
    Cache(diom_cache::operations::CacheOperation),
    CreateCache(diom_cache::operations::CreateCacheOp),
    CreateIdempotency(diom_idempotency::operations::CreateIdempotencyOp),
    Stream(stream_deprecated::operations::StreamOperation),
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl From<diom_idempotency::operations::IdempotencyOperation> for Request {
    fn from(value: diom_idempotency::operations::IdempotencyOperation) -> Self {
        Request::Idempotency(value)
    }
}

impl From<diom_cache::operations::CacheOperation> for Request {
    fn from(value: diom_cache::operations::CacheOperation) -> Self {
        Request::Cache(value)
    }
}

impl From<diom_cache::operations::CreateCacheOp> for Request {
    fn from(value: diom_cache::operations::CreateCacheOp) -> Self {
        Request::CreateCache(value)
    }
}

impl From<diom_idempotency::operations::CreateIdempotencyOp> for Request {
    fn from(value: diom_idempotency::operations::CreateIdempotencyOp) -> Self {
        Request::CreateIdempotency(value)
    }
}

impl From<diom_kv::operations::CreateKvOp> for Request {
    fn from(value: diom_kv::operations::CreateKvOp) -> Self {
        Request::CreateKv(value)
    }
}

impl From<stream_deprecated::operations::StreamOperation> for Request {
    fn from(value: stream_deprecated::operations::StreamOperation) -> Self {
        Request::Stream(value)
    }
}

impl From<InternalRequest> for Request {
    fn from(value: InternalRequest) -> Self {
        Self::ClusterInternal(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Blank,
    ClusterInternal(InternalResponse),
    CreateCache(diom_cache::operations::CreateCacheOperationResponse),
    CreateIdempotency(diom_idempotency::operations::CreateIdempotencyOperationResponse),
    CreateKv(diom_kv::operations::CreateKvOperationResponse),
    Kv(diom_kv::operations::Response),
    RateLimiter(diom_rate_limiter::operations::Response),
    Idempotency(diom_idempotency::operations::Response),
    Cache(diom_cache::operations::Response),
    Stream(stream_deprecated::operations::Response),
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

impl TryFrom<Response> for diom_idempotency::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Idempotency(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for diom_cache::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Cache(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for diom_cache::operations::CreateCacheOperationResponse {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::CreateCache(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for diom_idempotency::operations::CreateIdempotencyOperationResponse {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::CreateIdempotency(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for diom_kv::operations::CreateKvOperationResponse {
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

impl TryFrom<Response> for InternalResponse {
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
        >>::Error: fmt::Debug,
        <<<O as OperationRequest>::Response as OperationResponse>::ResponseParent as TryInto<
            <O as OperationRequest>::Response,
        >>::Error: fmt::Debug,
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
            .map_err(|e| {
                anyhow::anyhow!(
                    "raft response should be convertible into module response type: {e:?}"
                )
            })?;
        let resp = module_response.try_into().map_err(|e| {
            anyhow::anyhow!("module response should be convertible into target type: {e:?}")
        })?;
        Ok(resp)
    }

    pub async fn run_discovery_if_necessary(&self, cfg: Configuration) -> anyhow::Result<()> {
        let network = NetworkFactory::new(&cfg)?;
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
            let disco = Discovery::new(cfg, self.raft.clone(), self.node_id, network)?;
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

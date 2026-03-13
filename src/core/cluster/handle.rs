use std::{fmt, time::Duration};

use crate::{cfg::Configuration, core::cluster::state_machine::StoreHandle};

use super::{
    NodeId, discovery::Discovery, network::NetworkFactory, operations::InternalOperation,
    raft::Raft,
};
use anyhow::Context;
use coyote_operations::{OperationRequest, OperationRequestMetadata, OperationResponse};
use openraft::RaftNetworkFactory;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::mpsc::Sender;

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
    ClusterInternal(InternalOperation),
    Kv(coyote_kv::operations::KvOperation),
    RateLimiter(coyote_rate_limit::operations::RateLimiterOperation),
    Idempotency(coyote_idempotency::operations::IdempotencyOperation),
    Cache(coyote_cache::operations::CacheOperation),
    Msgs(coyote_msgs::operations::MsgsOperation),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestWithContext {
    pub inner: Request,
    #[serde(
        rename = "t",
        with = "jiff::fmt::serde::timestamp::millisecond::required"
    )]
    pub timestamp: jiff::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<OperationRequestMetadata>,
}

impl fmt::Display for RequestWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            Request::ClusterInternal(_) => write!(f, "cluster_internal"),
            Request::Kv(_) => write!(f, "kv"),
            Request::RateLimiter(_) => write!(f, "ratelimiter"),
            Request::Idempotency(_) => write!(f, "idempotency"),
            Request::Cache(_) => write!(f, "cache"),
            Request::Msgs(_) => write!(f, "msgs"),
        }
    }
}

impl RequestWithContext {
    pub(crate) fn new(
        req: Request,
        timestamp: jiff::Timestamp,
        ctx: Option<OperationRequestMetadata>,
    ) -> Self {
        Self {
            inner: req,
            timestamp,
            context: ctx,
        }
    }

    pub(crate) fn hashed_key(&self) -> Option<String> {
        let digest = match &self.inner {
            Request::Kv(op) => Sha256::digest(op.key_name()?),
            Request::RateLimiter(op) => Sha256::digest(op.key_name()),
            Request::Idempotency(op) => Sha256::digest(op.key_name()?),
            Request::Cache(op) => Sha256::digest(op.key_name()?),
            Request::Msgs(op) => Sha256::digest(op.key_name()),
            Request::ClusterInternal(_) => return None,
        };
        Some(hex::encode(digest))
    }
}

impl From<coyote_kv::operations::KvOperation> for Request {
    fn from(value: coyote_kv::operations::KvOperation) -> Self {
        Request::Kv(value)
    }
}

impl From<coyote_rate_limit::operations::RateLimiterOperation> for Request {
    fn from(value: coyote_rate_limit::operations::RateLimiterOperation) -> Self {
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

impl From<coyote_msgs::operations::MsgsOperation> for Request {
    fn from(value: coyote_msgs::operations::MsgsOperation) -> Self {
        Request::Msgs(value)
    }
}

impl From<InternalOperation> for Request {
    fn from(value: InternalOperation) -> Self {
        Self::ClusterInternal(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Blank,
    ClusterInternal(super::operations::Response),
    Kv(coyote_kv::operations::Response),
    RateLimiter(coyote_rate_limit::operations::Response),
    Idempotency(coyote_idempotency::operations::Response),
    Cache(coyote_cache::operations::Response),
    Msgs(coyote_msgs::operations::Response),
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

impl TryFrom<Response> for coyote_rate_limit::operations::Response {
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

impl TryFrom<Response> for coyote_msgs::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Msgs(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

impl TryFrom<Response> for super::operations::Response {
    type Error = ResponseParseError;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::ClusterInternal(v) => Ok(v),
            _ => Err(ResponseParseError::InvalidVariant),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundCommand {
    Snapshot,
}

#[derive(Clone)]
pub struct RaftState {
    pub raft: Raft,
    pub node_id: NodeId,
    pub state_machine: StoreHandle,
    pub(super) network: NetworkFactory,
    pub background_channel: Sender<BackgroundCommand>,
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
        let inner: Request = op.into().into();
        let now = self.state_machine.now();
        let request =
            RequestWithContext::new(inner, now, Some(opentelemetry::Context::current().into()));
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

    pub(crate) async fn trigger_snapshot(&self) -> anyhow::Result<()> {
        self.background_channel
            .send(BackgroundCommand::Snapshot)
            .await
            .context("attempting to send background command to trigger snapshot")
    }

    /// Accomplish a linearizable wait for the caller
    ///
    /// On the leader, this is implemented by calling `openraft::Raft::ensure_linearizable`.
    pub async fn wait_linearizable(&self) -> anyhow::Result<()> {
        let leader_id = match self.raft.current_leader().await {
            Some(n) if n == self.node_id => {
                tracing::trace!("performing a linearizable read on the leader");
                self.raft.ensure_linearizable().await?;
                return Ok(());
            }
            Some(leader) => leader,
            None => anyhow::bail!("no cluster leader, cannot perform linearizable operations"),
        };
        let leader_node = self
            .raft
            .with_raft_state(move |s| s.membership_state.effective().get_node(&leader_id).cloned())
            .await?
            .ok_or_else(|| anyhow::anyhow!("unable to look up leader node IP"))?;
        tracing::trace!(?leader_id, "performing a linearizable read on a follower");
        let mut network_handle = self.network.clone();
        let client = network_handle.new_client(leader_id, &leader_node).await;
        let Some(last_committed_log_id) = client.get_last_committed_log_id().await? else {
            tracing::warn!(
                "attempted to do a linearizable read, but nothing has ever been written"
            );
            return Ok(());
        };

        const DEFAULT_WAIT_TIME: Duration = Duration::from_secs(1);

        tracing::trace!(?last_committed_log_id, "waiting for follower to apply logs");
        self.raft
            .wait(Some(DEFAULT_WAIT_TIME))
            .applied_index_at_least(
                Some(last_committed_log_id.index),
                "waiting for linearizability",
            )
            .await?;
        Ok(())
    }
}

impl coyote_operations::OperationWriterBase for RaftState {
    type Request = Request;
    type Response = Response;

    async fn do_write_request(
        &self,
        request: Self::Request,
    ) -> coyote_operations::BackgroundResult<Self::Response> {
        let now = self.state_machine.now();
        let request =
            RequestWithContext::new(request, now, Some(opentelemetry::Context::current().into()));
        match self.raft.client_write(request.clone()).await {
            Ok(resp) => {
                tracing::trace!(log_id=?resp.log_id(), "request applied to log");
                Ok(resp.data)
            }
            Err(err) => {
                if err.forward_to_leader().is_some() {
                    Err(coyote_operations::BackgroundError::NotLeader)
                } else {
                    tracing::warn!(?err, "unhandled error writing request to raft");
                    Err(coyote_operations::BackgroundError::Other(
                        coyote_error::Error::internal(err),
                    ))
                }
            }
        }
    }
}

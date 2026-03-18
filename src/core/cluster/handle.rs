use std::{fmt, time::Duration};

use crate::{
    cfg::Configuration,
    core::cluster::{ClusterId, state_machine::StoreHandle},
};

use super::{
    Node, NodeId, discovery::Discovery, network::NetworkFactory, operations::InternalOperation,
    raft::Raft,
};
use anyhow::Context;
use coyote_core::Monotime;
use coyote_error::ResultExt;
use coyote_operations::{OperationRequest, OperationRequestMetadata, OperationResponse};
use itertools::Itertools;
use maplit::btreeset;
use openraft::{
    RaftNetworkFactory,
    error::{ClientWriteError, RaftError},
};
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
    RateLimit(coyote_rate_limit::operations::RateLimitOperation),
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
            Request::RateLimit(_) => write!(f, "ratelimiter"),
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
            Request::Cache(op) => Sha256::digest(op.key_name()?),
            Request::ClusterInternal(_) => return None,
            Request::Idempotency(op) => Sha256::digest(op.key_name()?),
            Request::Kv(op) => Sha256::digest(op.key_name()?),
            Request::Msgs(op) => Sha256::digest(op.key_name()),
            Request::RateLimit(op) => Sha256::digest(op.key_name()),
        };
        Some(hex::encode(digest))
    }

    pub(crate) fn module(&self) -> &'static str {
        match self.inner {
            Request::Cache(_) => "cache",
            Request::ClusterInternal(_) => "cluster-internal",
            Request::Idempotency(_) => "idempotency",
            Request::Kv(_) => "kv",
            Request::Msgs(_) => "msgs",
            Request::RateLimit(_) => "rate-limit",
        }
    }
}

impl From<coyote_kv::operations::KvOperation> for Request {
    fn from(value: coyote_kv::operations::KvOperation) -> Self {
        Request::Kv(value)
    }
}

impl From<coyote_rate_limit::operations::RateLimitOperation> for Request {
    fn from(value: coyote_rate_limit::operations::RateLimitOperation) -> Self {
        Request::RateLimit(value)
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
    RateLimit(coyote_rate_limit::operations::Response),
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
            Response::RateLimit(v) => Ok(v),
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

#[derive(Debug, Clone)]
pub(crate) enum CoyoteErrorOrForwardToLeader {
    Error(coyote_operations::OperationError),
    ForwardToLeader {
        leader_id: NodeId,
        leader_node: Node,
    },
}

impl CoyoteErrorOrForwardToLeader {
    #[track_caller]
    fn internal(e: impl fmt::Display) -> Self {
        Self::Error(coyote_operations::OperationError::from(
            coyote_error::Error::internal(e),
        ))
    }
}

impl From<coyote_error::Error> for CoyoteErrorOrForwardToLeader {
    fn from(value: coyote_error::Error) -> Self {
        Self::Error(value.into())
    }
}

impl From<RaftError<NodeId, ClientWriteError<NodeId, Node>>> for CoyoteErrorOrForwardToLeader {
    fn from(value: RaftError<NodeId, ClientWriteError<NodeId, Node>>) -> Self {
        if let Some(leader) = value.forward_to_leader() {
            let Some(leader_id) = leader.leader_id else {
                return Self::internal("wanted to forward to leader, but do not know leader ID");
            };
            let Some(leader_node) = leader.leader_node.clone() else {
                return Self::internal(
                    "wanted to forward to leader, but do not know leader address",
                );
            };
            Self::ForwardToLeader {
                leader_id,
                leader_node,
            }
        } else {
            Self::internal(value)
        }
    }
}

#[derive(Clone)]
pub struct RaftState {
    pub cfg: Configuration,
    pub raft: Raft,
    pub node_id: NodeId,
    pub state_machine: StoreHandle,
    pub(super) network: NetworkFactory,
    pub background_channel: Sender<BackgroundCommand>,
    pub time: Monotime,
}

impl RaftState {
    /// Write a single operation into the Raft log and return its response.
    #[tracing::instrument(skip_all)]
    pub async fn client_write<O>(&self, op: O) -> anyhow::Result<O::Response>
    where
        O: OperationRequest<
                RequestParent: Into<Request>,
                Response: OperationResponse<
                    ResponseParent: TryFrom<Response, Error: fmt::Debug>
                                        + TryInto<O::Response, Error: fmt::Debug>,
                >,
            > + Into<O::RequestParent>,
    {
        let inner: Request = op.into().into();
        let now = self.time.now();
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
                        tracing::trace!("received write to non-leader, forwarding");
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
        let module_response = <O::Response as OperationResponse>::ResponseParent::try_from(
            response,
        )
        .map_err(|e| {
            anyhow::anyhow!("raft response should be convertible into module response type: {e:?}")
        })?;
        let resp = module_response.try_into().map_err(|e| {
            anyhow::anyhow!("module response should be convertible into target type: {e:?}")
        })?;
        Ok(resp)
    }

    pub async fn run_discovery_if_necessary(&self) -> anyhow::Result<()> {
        let network = NetworkFactory::new(&self.cfg)?;
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
            let disco = Discovery::new(self.cfg.clone(), self.raft.clone(), self.node_id, network)?;
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
        let Ok(state) = self
            .raft
            .with_raft_state(|s| s.server_state)
            .await
            .inspect_err(|err| tracing::warn!(?err, "error reading server state"))
        else {
            return false;
        };
        if state.is_leader() {
            true
        } else if state.is_learner() {
            false
        } else {
            let Some(leader) = self.raft.current_leader().await else {
                tracing::warn!(my_state=?state, "no current leader known");
                return false;
            };
            if !self
                .raft
                .with_raft_state(move |s| {
                    s.membership_state.effective().get_node(&leader).is_some()
                })
                .await
                .unwrap_or(false)
            {
                tracing::warn!(
                    ?leader,
                    "I know the leader's node ID, but not yet their address"
                );
                false
            } else {
                true
            }
        }
    }

    pub async fn state(&self) -> anyhow::Result<openraft::ServerState> {
        self.raft
            .with_raft_state(|s| s.server_state)
            .await
            .context("error reading server state")
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

    pub(crate) async fn get_peer_last_committed_log(
        &self,
        node_id: NodeId,
        address: &Node,
    ) -> anyhow::Result<Option<openraft::LogId<NodeId>>> {
        let mut network_handle = self.network.clone();
        let client = network_handle.new_client(node_id, address).await;
        client
            .get_last_committed_log_id()
            .await
            .context("attempting to read last committed id from peer")
    }

    #[tracing::instrument(skip(self))]
    pub(crate) async fn handle_go_away(
        &self,
        cluster_id: ClusterId,
        node_id: NodeId,
    ) -> Result<(), crate::Error> {
        if node_id != self.node_id {
            return Err(crate::Error::bad_request(
                "invalid node_id",
                "Received a GO-AWAY request for the wrong node",
            ));
        }
        let Some(our_cluster_id) = self.state_machine.cluster_id().await else {
            tracing::warn!("received go-away at non-clustered node");
            return Err(crate::Error::bad_request(
                "invalid cluster_id",
                "Received a GO-AWAY request but we are not in a cluster",
            ));
        };
        if cluster_id != our_cluster_id {
            tracing::warn!(
                ?cluster_id,
                ?our_cluster_id,
                "received go-away for the wrong cluster"
            );
            return Err(crate::Error::bad_request(
                "invalid node_id",
                "Received a GO-AWAY request for the wrong cluster",
            ));
        }
        self.state_machine
            .poison(cluster_id)
            .await
            .or_internal_error()?;
        tracing::error!("this node has been removed from a cluster and will now shut down");
        if self.cfg.cluster.shut_down_on_go_away {
            crate::start_shut_down();
        }
        Ok(())
    }

    async fn guard_leader(&self) -> Result<(), CoyoteErrorOrForwardToLeader> {
        let Some(leader_id) = self.raft.current_leader().await else {
            return Err(CoyoteErrorOrForwardToLeader::internal(
                "unable to determine leader",
            ));
        };
        if leader_id != self.node_id {
            let Some(leader_node) = self
                .raft
                .with_raft_state(move |s| {
                    s.membership_state.effective().get_node(&leader_id).cloned()
                })
                .await
                .or_internal_error()?
            else {
                tracing::error!(
                    ?leader_id,
                    "want to forward request to leader, but cannot find leader node"
                );
                return Err(CoyoteErrorOrForwardToLeader::from(
                    coyote_error::Error::internal("unable to determine leader"),
                ));
            };
            return Err(CoyoteErrorOrForwardToLeader::ForwardToLeader {
                leader_id,
                leader_node,
            });
        }
        Ok(())
    }

    async fn remove_node_inner(&self, node_id: NodeId) -> Result<(), CoyoteErrorOrForwardToLeader> {
        self.guard_leader().await?;
        let Some(cluster_id) = self.state_machine.cluster_id().await else {
            return Err(CoyoteErrorOrForwardToLeader::internal(
                "attempted to remove node from a null cluster",
            ));
        };
        let (is_voter, is_learner, node) = self
            .raft
            .with_raft_state(move |s| {
                let membership = s.membership_state.effective().membership();
                let is_voter = membership.voter_ids().contains(&node_id);
                let is_learner = membership.learner_ids().contains(&node_id);
                let node = membership.get_node(&node_id).cloned();
                (is_voter, is_learner, node)
            })
            .await
            .or_internal_error()?;
        if !(is_voter || is_learner) {
            return Err(coyote_error::Error::invalid_user_input(
                "node is neither a voter nor a learner",
            )
            .into());
        }
        let Some(node) = node else {
            return Err(CoyoteErrorOrForwardToLeader::internal(
                "wanted to remove a node, but could not find its address",
            ));
        };
        if is_voter {
            tracing::info!(%node_id, ?node, "downgrading node from voter to learner");
            let proposal = openraft::ChangeMembers::RemoveVoters(btreeset! { node_id });
            self.raft.change_membership(proposal, true).await?;
        }
        let client = self.network.client_for(node_id, &node);
        tracing::info!(%node_id, ?node, "removing learner from cluster");
        let proposal = openraft::ChangeMembers::RemoveNodes(btreeset! { node_id });
        self.raft.change_membership(proposal, false).await?;
        if let Err(err) = client
            .go_away(crate::core::cluster::proto::GoAwayRequest {
                cluster_id,
                node_id,
            })
            .await
        {
            tracing::error!(
                ?err,
                "Peer refused GO-AWAY request and may need to be manually shut down"
            );
        }
        Ok(())
    }

    /// Remove the given node_id.
    ///
    /// If it's a voter, first downgrade it to a learner, then remove it entirely.
    /// Returns the removed node.
    pub(crate) async fn remove_node(&self, node_id: NodeId) -> coyote_error::Result<()> {
        match self.remove_node_inner(node_id).await {
            Ok(_) => Ok(()),
            Err(CoyoteErrorOrForwardToLeader::Error(e)) => Err(e.into()),
            Err(CoyoteErrorOrForwardToLeader::ForwardToLeader {
                leader_id,
                leader_node,
            }) => {
                let mut network_handle = self.network.clone();
                let client = network_handle.new_client(leader_id, &leader_node).await;
                client
                    .remove_node(super::proto::RemoveNodeRequest { node_id })
                    .await
                    .or_internal_error()?;
                Ok(())
            }
        }
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

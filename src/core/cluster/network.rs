use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use crate::{
    cfg::{Configuration, PeerAddr},
    core::{
        cluster::state_machine::StoredSnapshot,
        metrics::{ClusterNetworkMetrics, ClusterRequestStatus},
    },
};

use super::{LogId, Node, NodeId, proto, raft::TypeConfig};
use anyhow::Context;
use diom_proto::prelude::*;
use http::{HeaderMap, HeaderValue, StatusCode, header};
use openraft::{
    RaftNetworkFactory,
    error::{NetworkError, RaftError, Unreachable},
    network::RPCOption,
};
use openraft_legacy::network_v1::{InstallSnapshotError, RaftNetwork};
use serde::{Serialize, de::DeserializeOwned};
use tap::Pipe;

type RPCError<E = openraft::errors::Infallible> = openraft::error::RPCError<TypeConfig, E>;
type RPCResult<T, E = openraft::errors::Infallible> = Result<T, RPCError<E>>;

#[derive(Debug, Clone)]
pub(super) struct BadStatusError(StatusCode);

impl From<StatusCode> for BadStatusError {
    fn from(value: StatusCode) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for BadStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadStatusError({})", self.0)
    }
}

impl std::error::Error for BadStatusError {}

pub(super) fn build_client(
    cfg: &Configuration,
    request_timeout: Option<Duration>,
    include_secret: bool,
) -> anyhow::Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/msgpack"),
    );
    if include_secret && let Some(secret) = &cfg.cluster.secret {
        let header_value = format!("Bearer {secret}");
        let header_value =
            HeaderValue::from_str(&header_value).context("invalid interserver secret")?;
        headers.insert(header::AUTHORIZATION, header_value);
    }
    tracing::debug!(
        connect_timeout = ?cfg.cluster.connection_timeout,
        ?request_timeout,
        "initializing interserver client",
    );
    let client = reqwest::Client::builder()
        .connect_timeout(cfg.cluster.connection_timeout.into())
        .pipe(|client| {
            if let Some(timeout) = request_timeout {
                client.timeout(timeout)
            } else {
                client
            }
        })
        .http2_prior_knowledge()
        .default_headers(headers)
        .build()
        .context("building raft network client")?;
    Ok(client)
}

#[derive(Clone)]
pub(super) struct NetworkFactory {
    client: reqwest::Client,
    cfg: Configuration,
    metrics: ClusterNetworkMetrics,
}

impl NetworkFactory {
    pub(super) fn new(cfg: &Configuration, metrics: ClusterNetworkMetrics) -> anyhow::Result<Self> {
        Ok(Self {
            client: build_client(cfg, None, true)?,
            cfg: cfg.clone(),
            metrics,
        })
    }
}

pub(super) struct NetworkClient {
    target: NodeId,
    node: Node,
    client: reqwest::Client,
    cfg: Configuration,
    default_timeout: Duration,
    metrics: ClusterNetworkMetrics,
}

impl NetworkClient {
    #[allow(clippy::result_large_err)]
    async fn send_request<Req, Resp, Err>(&self, path: &str, req: Req) -> RPCResult<Resp, Err>
    where
        Req: Serialize + Sized,
        Resp: DeserializeOwned + Sized,
        Err: std::error::Error + DeserializeOwned + Sized,
    {
        self.send_request_with_timeout(path, req, self.default_timeout)
            .await
    }

    #[allow(clippy::result_large_err)]
    async fn send_request_with_timeout<Req, Resp, Err>(
        &self,
        path: &str,
        req: Req,
        timeout: Duration,
    ) -> RPCResult<Resp, Err>
    where
        Req: Serialize + Sized,
        Resp: DeserializeOwned + Sized,
        Err: std::error::Error + DeserializeOwned + Sized,
    {
        let start = Instant::now();
        // TODO(jbrown|2026-02-20) handle multiple addresses
        let Ok(url) = self.node.url_for(path) else {
            tracing::warn!(node_id=?self.target, node=?self.node, "node has no valid addresses, cannot send rpc");
            self.metrics.record_unaddressed_request(self.target);
            return Err(RPCError::Unreachable(Unreachable::new(
                &crate::Error::internal("no has no known addresses"),
            )));
        };
        tracing::trace!(%url, target = ?self.target, "sending internal RPC");

        let response = self
            .client
            .post(url)
            .timeout(timeout)
            .msgpack(&req)
            .map_err(|err| {
                tracing::warn!(
                    ?err,
                    "serialization error on RPC! this should be impossible!"
                );
                self.metrics.record_request(
                    self.target,
                    ClusterRequestStatus::SerializationError,
                    start.elapsed(),
                );
                RPCError::Network(NetworkError::new(&err))
            })?
            .pipe(|this| -> Result<reqwest::RequestBuilder, RPCError<Err>> {
                if let Some(secret) = &self.cfg.cluster.secret {
                    let auth = format!("Bearer {secret}");
                    let auth = HeaderValue::from_str(&auth).map_err(|err| {
                        tracing::warn!("invalid interserver secret value");
                        RPCError::<Err>::Network(NetworkError::new(&err))
                    })?;
                    Ok(this.header(header::AUTHORIZATION, auth))
                } else {
                    Ok(this)
                }
            })?
            .send()
            .await
            .map_err(|err| {
                tracing::warn!(?err, "error sending message to peer");
                let status = if err.is_timeout() {
                    ClusterRequestStatus::TimeoutError
                } else if err.is_connect() {
                    ClusterRequestStatus::ConnectError
                } else {
                    ClusterRequestStatus::OtherError
                };
                self.metrics
                    .record_request(self.target, status, start.elapsed());
                if err.is_connect() {
                    RPCError::Unreachable(Unreachable::new(&err))
                } else {
                    RPCError::Network(NetworkError::new(&err))
                }
            })?;

        let status = response.status();

        let body = match status {
            s if s.is_success() => response.msgpack().await.map_err(|err| {
                tracing::warn!(?err, ?status, "error deserializing response body");
                self.metrics.record_request(
                    self.target,
                    ClusterRequestStatus::DeserializationError,
                    start.elapsed(),
                );
                RPCError::Network(NetworkError::new(&err))
            })?,
            StatusCode::INTERNAL_SERVER_ERROR => {
                let err_response = response.msgpack().await.map_err(|err| {
                    tracing::warn!(?err, ?status, "error deserializing response body");
                    self.metrics.record_request(
                        self.target,
                        ClusterRequestStatus::DeserializationError,
                        start.elapsed(),
                    );
                    RPCError::Network(NetworkError::new(&err))
                })?;
                self.metrics.record_request(
                    self.target,
                    ClusterRequestStatus::OtherError,
                    start.elapsed(),
                );
                let error = openraft::error::RemoteError::new(self.target, err_response);
                return Err(RPCError::RemoteError(error));
            }
            _ => {
                tracing::warn!(?status, "error from responding server");
                self.metrics.record_request(
                    self.target,
                    ClusterRequestStatus::OtherError,
                    start.elapsed(),
                );
                return Err(RPCError::Network(NetworkError::new(&BadStatusError::from(
                    status,
                ))));
            }
        };

        tracing::trace!(
            ?status,
            duration = ?start.elapsed(),
            "response from peer server");

        self.metrics
            .record_request(self.target, ClusterRequestStatus::Success, start.elapsed());

        Ok(body)
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn forward_request(
        &self,
        req: proto::ForwardedWriteRequest,
    ) -> RPCResult<proto::ForwardedWriteResponse> {
        self.send_request("/repl/raft/handle-forwarded-write", req)
            .await
    }

    pub(super) async fn add_learner(
        &self,
        req: proto::AddLearnerRequest,
    ) -> Result<proto::AddLearnerResponse, RPCError> {
        self.send_request("/repl/raft/admin/add-learner", req).await
    }

    pub(super) async fn upgrade_learner(
        &self,
        req: proto::UpgradeLearnerRequest,
    ) -> Result<proto::UpgradeLearnerResponse, RPCError> {
        self.send_request("/repl/raft/admin/upgrade-learner", req)
            .await
    }

    pub(super) async fn remove_node(
        &self,
        req: proto::RemoveNodeRequest,
    ) -> Result<proto::RemoveNodeResponse, RPCError> {
        self.send_request("/repl/raft/admin/remove-node", req).await
    }

    pub(super) async fn go_away(
        &self,
        req: proto::GoAwayRequest,
    ) -> Result<proto::GoAwayResponse, RPCError> {
        self.send_request("/repl/raft/go-away", req).await
    }

    pub(super) async fn get_last_committed_log_id(&self) -> Result<Option<LogId>, RPCError> {
        let proto::LastIdResponse {
            last_committed_log_id,
        } = self
            .send_request("/repl/raft/last-id", proto::LastIdRequest {})
            .await?;
        Ok(last_committed_log_id)
    }

    #[allow(unused)]
    pub(super) fn target(&self) -> NodeId {
        self.target
    }
}

impl RaftNetwork<TypeConfig> for NetworkClient {
    #[tracing::instrument(skip_all)]
    async fn append_entries(
        &mut self,
        rpc: openraft::raft::AppendEntriesRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<openraft::raft::AppendEntriesResponse<TypeConfig>, RPCError<RaftError<TypeConfig>>>
    {
        self.send_request_with_timeout("/repl/raft/append_entries", rpc, option.soft_ttl())
            .await
    }

    #[tracing::instrument(skip_all)]
    async fn vote(
        &mut self,
        rpc: openraft::raft::VoteRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<openraft::raft::VoteResponse<TypeConfig>, RPCError<RaftError<TypeConfig>>> {
        self.send_request_with_timeout("/repl/raft/vote", rpc, option.soft_ttl())
            .await
    }

    #[tracing::instrument(skip_all)]
    async fn install_snapshot(
        &mut self,
        rpc: openraft_legacy::network_v1::InstallSnapshotRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<
        openraft_legacy::network_v1::InstallSnapshotResponse<TypeConfig>,
        RPCError<RaftError<TypeConfig, InstallSnapshotError>>,
    > {
        self.send_request_with_timeout("/repl/raft/stream-snapshot", rpc, option.soft_ttl())
            .await
    }

    #[tracing::instrument(skip_all)]
    async fn transfer_leader(
        &mut self,
        rpc: openraft::raft::TransferLeaderRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<openraft::raft::TransferLeaderResponse<TypeConfig>, RPCError> {
        self.send_request_with_timeout("/repl/raft/transfer_leader", rpc, option.soft_ttl())
            .await
    }
}

impl RaftNetworkFactory<TypeConfig> for NetworkFactory {
    type Network = openraft_legacy::network_v1::Adapter<TypeConfig, NetworkClient, StoredSnapshot>;

    async fn new_client(&mut self, target: NodeId, node: &Node) -> Self::Network {
        self.client_for(target, node).into_v2()
    }
}

impl NetworkFactory {
    /// Create a new client pointed at the given target
    pub(super) fn client_for(&self, target: NodeId, node: &Node) -> NetworkClient {
        NetworkClient {
            target,
            node: node.clone(),
            client: self.client.clone(),
            cfg: self.cfg.clone(),
            default_timeout: Duration::from_secs(60),
            metrics: self.metrics.clone(),
        }
    }
}

fn is_unspecified(s: &SocketAddr) -> bool {
    match s {
        SocketAddr::V4(s) => s.ip().is_unspecified(),
        SocketAddr::V6(s) => s.ip().is_unspecified(),
    }
}

async fn search_for_self_in_peers(
    seeds: &[PeerAddr],
    cfg: &Configuration,
    my_node_id: NodeId,
) -> anyhow::Result<Option<PeerAddr>> {
    let client = build_client(cfg, Some(Duration::from_secs(2)), false)?;
    for peer in seeds {
        let url = peer.as_base_url().join("/repl/node-id")?;
        let Ok(response) = client.get(url).send().await else {
            tracing::debug!(?peer, "skipping seed peer because it is not responding");
            continue;
        };
        let Ok(body) = response.msgpack::<proto::GetNodeIdResponse>().await else {
            tracing::debug!(
                ?peer,
                "skipping seed peer because it returned an invalid body"
            );
            continue;
        };
        if body.node_id == my_node_id {
            return Ok(Some(peer.clone()));
        }
    }
    Ok(None)
}

pub(crate) async fn detect_address(
    cfg: &Configuration,
    my_node_id: NodeId,
) -> anyhow::Result<PeerAddr> {
    if let Some(addr) = &cfg.cluster.advertised_address {
        tracing::debug!(?addr, "using configured advertised_address");
        return Ok(addr.clone());
    }

    let cluster_addr = cfg.cluster.listen_address;
    if !is_unspecified(&cluster_addr) {
        tracing::debug!(addr = ?cluster_addr, "using configured cluster listen_address");
        return Ok(PeerAddr::SocketAddr(cluster_addr));
    }

    if !cfg.cluster.seed_nodes.is_empty()
        && let Some(addr) =
            search_for_self_in_peers(&cfg.cluster.seed_nodes, cfg, my_node_id).await?
    {
        tracing::debug!(?addr, "using address from seed_nodes");
        return Ok(addr);
    }

    tracing::debug!("falling back to looking on all local interfaces");

    // TODO: this should handle dual-homed (ipv4 + ipv6) systems
    let port = cluster_addr.port();
    for interface in pnet::datalink::interfaces() {
        if !interface.is_up() || interface.is_loopback() || interface.ips.is_empty() {
            continue;
        }
        if let Some(ip) = interface.ips.iter().find(|i| i.is_ipv4()) {
            return Ok(PeerAddr::SocketAddr(SocketAddr::new(ip.ip(), port)));
        }
    }
    anyhow::bail!("unable to find any valid interfaces");
}

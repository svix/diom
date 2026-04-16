use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use crate::cfg::{Configuration, PeerAddr};

use super::{LogId, Node, NodeId, proto, raft::TypeConfig};
use anyhow::Context;
use diom_proto::prelude::*;
use http::{HeaderMap, HeaderValue, header};
use openraft::{
    RaftNetworkFactory, RaftNetworkV2,
    error::{NetworkError, Unreachable},
    network::RPCOption,
};
use serde::{Serialize, de::DeserializeOwned};
use tap::Pipe;

type RPCError<E = openraft::errors::Infallible> = openraft::error::RPCError<TypeConfig, E>;
type RPCResult<T, E = openraft::errors::Infallible> = Result<T, RPCError<E>>;

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
}

impl NetworkFactory {
    pub(super) fn new(cfg: &Configuration) -> anyhow::Result<Self> {
        Ok(Self {
            client: build_client(cfg, None, true)?,
            cfg: cfg.clone(),
        })
    }
}

pub(super) struct NetworkClient {
    target: NodeId,
    node: Node,
    client: reqwest::Client,
    cfg: Configuration,
    default_timeout: Duration,
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
                if err.is_connect() {
                    RPCError::Unreachable(Unreachable::new(&err))
                } else {
                    RPCError::Network(NetworkError::new(&err))
                }
            })?
            .error_for_status()
            .map_err(|e| {
                tracing::warn!(status = ?e.status(), "error from responding server");
                RPCError::Network(NetworkError::new(&e))
            })?;

        tracing::trace!(
            status = ?response.status(),
            duration = ?start.elapsed(),
            "response from peer server");

        response
            .msgpack()
            .await
            .map_err(|e| RPCError::Network(NetworkError::new(&e)))
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

    #[tracing::instrument(skip_all)]
    pub(super) async fn install_snapshot(
        &mut self,
        rpc: openraft::raft::InstallSnapshotRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<
        openraft::raft::InstallSnapshotResponse<TypeConfig>,
        RPCError<openraft::errors::RaftError<TypeConfig, openraft::errors::InstallSnapshotError>>,
    > {
        self.send_request_with_timeout("/repl/raft/stream-snapshot", rpc, option.soft_ttl())
            .await
    }
}

impl RaftNetworkV2<TypeConfig> for NetworkClient {
    #[tracing::instrument(skip_all)]
    async fn append_entries(
        &mut self,
        rpc: openraft::raft::AppendEntriesRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<openraft::raft::AppendEntriesResponse<TypeConfig>, RPCError> {
        self.send_request_with_timeout("/repl/raft/append_entries", rpc, option.soft_ttl())
            .await
    }

    #[tracing::instrument(skip_all)]
    async fn vote(
        &mut self,
        rpc: openraft::raft::VoteRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<openraft::raft::VoteResponse<TypeConfig>, RPCError> {
        self.send_request_with_timeout("/repl/raft/vote", rpc, option.soft_ttl())
            .await
    }

    #[tracing::instrument(skip_all, fields(?vote))]
    async fn full_snapshot(
        &mut self,
        vote: openraft::type_config::alias::VoteOf<TypeConfig>,
        snapshot: openraft::type_config::alias::SnapshotOf<TypeConfig>,
        cancel: impl Future<Output = openraft::error::ReplicationClosed>
        + openraft::OptionalSend
        + 'static,
        option: RPCOption,
    ) -> Result<
        openraft::raft::SnapshotResponse<TypeConfig>,
        openraft::error::StreamingError<TypeConfig>,
    > {
        super::streaming_snapshot::Sender::send_snapshot(self, vote, snapshot, cancel, option).await
    }
}

impl RaftNetworkFactory<TypeConfig> for NetworkFactory {
    type Network = NetworkClient;

    async fn new_client(&mut self, target: NodeId, node: &Node) -> Self::Network {
        self.client_for(target, node)
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

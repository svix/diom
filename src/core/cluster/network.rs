use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use crate::cfg::Configuration;

use super::{Node, NodeId, raft::TypeConfig};
use anyhow::Context;
use coyote_proto::prelude::*;
use http::{HeaderMap, HeaderValue, header};
use openraft::{
    RaftNetwork, RaftNetworkFactory, RaftTypeConfig,
    error::{NetworkError, RPCError, Unreachable},
    network::RPCOption,
};
use serde::{Serialize, de::DeserializeOwned};

pub(super) fn build_client(
    cfg: &Configuration,
    _request_timeout: Duration,
) -> anyhow::Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    if let Some(secret) = &cfg.cluster.secret {
        let header_value = format!("Bearer {secret}");
        let header_value =
            HeaderValue::from_str(&header_value).context("invalid interserver secret")?;
        headers.insert(header::AUTHORIZATION, header_value);
    }
    let client = reqwest::Client::builder()
        .connect_timeout(cfg.cluster.connection_timeout)
        .default_headers(headers)
        .build()
        .context("building raft network client")?;
    Ok(client)
}

pub(super) struct NetworkFactory {
    client: reqwest::Client,
}

impl NetworkFactory {
    pub(super) fn new(cfg: &Configuration) -> Self {
        Self {
            client: build_client(cfg, cfg.cluster.replication_request_timeout)
                .expect("failed to build Raft network"),
        }
    }
}

pub(super) struct NetworkClient {
    target: NodeId,
    node: Node,
    client: reqwest::Client,
}

impl NetworkClient {
    async fn send_request<Req, Resp, Err>(
        &self,
        path: &str,
        req: Req,
    ) -> Result<Resp, openraft::error::RPCError<NodeId, Node, Err>>
    where
        Req: Serialize,
        Err: std::error::Error + DeserializeOwned,
        Resp: DeserializeOwned,
    {
        let start = Instant::now();
        let addr = self.node.addr.as_str();
        let url = format!("http://{addr}/{path}");
        tracing::trace!(url, target=?self.target, "sending internal RPC");

        let response = self
            .client
            .post(url)
            .msgpack(&req)
            .map_err(|err| {
                tracing::warn!(
                    ?err,
                    "serialization error on RPC! this should be impossible!"
                );
                RPCError::Network(NetworkError::new(&err))
            })?
            .header(
                header::ACCEPT,
                "application/msgpack;q=0.9, application/json;q=0.5",
            )
            .send()
            .await
            .map_err(|e| {
                tracing::warn!(err=?e, "error sending message to peer");
                if e.is_connect() {
                    RPCError::Unreachable(Unreachable::new(&e))
                } else {
                    RPCError::Network(NetworkError::new(&e))
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
}

impl RaftNetwork<TypeConfig> for NetworkClient {
    async fn append_entries(
        &mut self,
        rpc: openraft::raft::AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<
        openraft::raft::AppendEntriesResponse<NodeId>,
        RPCError<NodeId, Node, openraft::error::RaftError<NodeId>>,
    > {
        self.send_request("/repl/raft/append_entries", rpc).await
    }

    async fn vote(
        &mut self,
        rpc: openraft::raft::VoteRequest<<TypeConfig as RaftTypeConfig>::NodeId>,
        _option: RPCOption,
    ) -> Result<
        openraft::raft::VoteResponse<NodeId>,
        RPCError<NodeId, Node, openraft::error::RaftError<NodeId>>,
    > {
        self.send_request("/repl/raft/vote", rpc).await
    }

    async fn install_snapshot(
        &mut self,
        rpc: openraft::raft::InstallSnapshotRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<
        openraft::raft::InstallSnapshotResponse<NodeId>,
        RPCError<
            NodeId,
            Node,
            openraft::error::RaftError<NodeId, openraft::error::InstallSnapshotError>,
        >,
    > {
        self.send_request("/repl/raft/stream-snapshot", rpc).await
    }
}

impl RaftNetworkFactory<TypeConfig> for NetworkFactory {
    type Network = NetworkClient;

    async fn new_client(
        &mut self,
        target: <TypeConfig as RaftTypeConfig>::NodeId,
        node: &<TypeConfig as RaftTypeConfig>::Node,
    ) -> Self::Network {
        NetworkClient {
            target,
            node: node.clone(),
            client: self.client.clone(),
        }
    }
}

pub(super) fn detect_address(cfg: &Configuration) -> anyhow::Result<SocketAddr> {
    // TODO: this should handle the address changing, which it currently can't
    // TODO: this should handle dual-homed (ipv4 + ipv6) systems
    let port = cfg.cluster.listen_address.port();
    for interface in pnet::datalink::interfaces() {
        if !interface.is_up() || interface.is_loopback() || interface.ips.is_empty() {
            continue;
        }
        if let Some(ip) = interface.ips.iter().find(|i| i.is_ipv4()) {
            return Ok(SocketAddr::new(ip.ip(), port));
        }
    }
    anyhow::bail!("unable to find any valid interfaces");
}

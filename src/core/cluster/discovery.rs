use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use coyote_proto::prelude::*;
use futures_util::{StreamExt, stream};
use itertools::Itertools;
use openraft::ServerState;
use tap::{Pipe, TapFallible};

use super::{Node, NodeId, network::build_client, raft::Raft, state_machine::ClusterId};
use crate::{
    Configuration,
    cfg::PeerAddr,
    core::cluster::{
        network::{NetworkFactory, detect_address},
        proto::{
            AddLearnerRequest, DiscoverClusterResponse, DiscoverResponse, UpgradeLearnerRequest,
        },
    },
    shutting_down_token,
};

const CONCURRENT_FETCHES: usize = 8;

pub(super) struct Discovery {
    client: reqwest::Client,
    my_node_id: NodeId,
    raft: Raft,
    cfg: Configuration,
    my_addr: PeerAddr,
    network: NetworkFactory,
}

impl Discovery {
    pub(super) async fn new(
        cfg: Configuration,
        raft: Raft,
        my_node_id: NodeId,
        network: NetworkFactory,
    ) -> anyhow::Result<Self> {
        let client = build_client(&cfg, cfg.cluster.discovery_request_timeout, true)?;
        let my_addr = detect_address(&cfg, my_node_id).await?;
        Ok(Self {
            my_addr,
            client,
            cfg,
            raft,
            my_node_id,
            network,
        })
    }

    async fn poll_node(&self, s: &PeerAddr) -> Option<DiscoverResponse> {
        let url = s
            .as_base_url()
            .join("/repl/discover")
            .expect("discovery URL should be valid");
        self.client
            .get(url)
            .send()
            .await
            .tap_err(|err| tracing::warn!(peer=?s, ?err, "unable to poll seed node"))
            .ok()?
            .error_for_status()
            .tap_err(|err| tracing::warn!(peer=?s, ?err, "got invalid HTTP response from seed"))
            .ok()?
            .msgpack()
            .await
            .tap_err(|err| tracing::warn!(peer=?s, ?err, "unable to read response body from seed"))
            .ok()
    }

    async fn poll_seeds(&self) -> (Option<PeerAddr>, BTreeMap<PeerAddr, DiscoverResponse>) {
        let mut responses = self
            .cfg
            .cluster
            .seed_nodes
            .clone()
            .into_iter()
            .map(|s| async move { self.poll_node(&s).await.map(|resp| (s, resp)) })
            .pipe(stream::iter)
            .buffer_unordered(CONCURRENT_FETCHES)
            .filter_map(futures_util::future::ready)
            .collect::<BTreeMap<_, _>>()
            .await;
        let my_addr = responses
            .iter()
            .find(|(_k, v)| v.node_id == self.my_node_id)
            .map(|(k, _)| k.to_owned());
        if let Some(addr) = &my_addr {
            responses.remove(addr);
        }
        (my_addr.clone(), responses)
    }

    async fn join_cluster(
        &self,
        cluster_id: ClusterId,
        peers: Vec<(PeerAddr, NodeId, DiscoverClusterResponse)>,
    ) -> anyhow::Result<()> {
        tracing::info!(?cluster_id, "joining running cluster");
        let Some((leader_addr, leader_node_id, leader_cluster)) = peers
            .into_iter()
            .find_or_first(|p| p.2.state == ServerState::Leader)
        else {
            anyhow::bail!("failed to find any peers to join!");
        };
        let Some(log_id) = leader_cluster.last_committed_log_id else {
            anyhow::bail!("existing cluster has no logs");
        };
        // TODO: if any of these steps fail, how do we recover?
        let client = self.network.client_for(leader_node_id, &leader_addr.into());
        client
            .add_learner(AddLearnerRequest {
                node_id: self.my_node_id,
                address: self.my_addr.clone(),
            })
            .await?;
        tracing::debug!("waiting to catch up in replication");
        self.raft
            .wait(None)
            .log_index_at_least(Some(log_id.index), "waiting to catch up to leader")
            .await?;
        tracing::debug!("adding self as a full member");
        client
            .upgrade_learner(UpgradeLearnerRequest {
                node_id: self.my_node_id,
            })
            .await?;
        Ok(())
    }

    async fn initialize_cluster(
        &self,
        discovered_seeds: BTreeMap<PeerAddr, DiscoverResponse>,
    ) -> anyhow::Result<()> {
        if !self.cfg.cluster.auto_initialize {
            tracing::warn!(
                "auto-initialization disabled, but we found no peers. You must manually initialize!"
            );
            return Ok(());
        }
        let my_node = Node::from(self.my_addr.clone());
        let mut nodes = [(self.my_node_id, my_node)]
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        for (peer_address, response) in discovered_seeds {
            tracing::trace!(?peer_address, ?response, "adding node to new cluster");
            nodes.insert(response.node_id, Node::from(peer_address));
        }
        tracing::debug!(?nodes, "initializing cluster with nodes");
        super::raft::initialize_cluster(&self.raft, nodes).await?;
        tracing::info!("initialized new cluster");
        Ok(())
    }

    pub(super) async fn discover_cluster(mut self) -> anyhow::Result<()> {
        if self.cfg.cluster.seed_nodes.is_empty() {
            tracing::debug!("no seed nodes provided, initializing a one-node cluster");
            self.initialize_cluster(BTreeMap::new()).await?;
            return Ok(());
        }

        tracing::debug!("starting new cluster discovery");

        let deadline = Instant::now() + self.cfg.cluster.discovery_timeout;
        // delay a little bit to allow simultaneous startups to finish
        let token = shutting_down_token();
        if token
            .run_until_cancelled(tokio::time::sleep(self.cfg.cluster.startup_discovery_delay))
            .await
            .is_none()
        {
            return Ok(());
        }
        let mut rounds_with_no_peers = 0;
        while Instant::now() < deadline {
            let (my_addr, discovered_seeds) = self.poll_seeds().await;
            if let Some(addr) = &my_addr {
                tracing::trace!(?addr, "discovered my seed address");
                self.my_addr = addr.clone();
            }
            tracing::trace!(?discovered_seeds, "discovered peers");

            let num_nodes_in_live_clusters = discovered_seeds
                .values()
                .filter(|v| {
                    v.cluster
                        .as_ref()
                        .map(|c| c.last_committed_log_id.is_some())
                        .unwrap_or(false)
                })
                .count();

            if num_nodes_in_live_clusters == 0 {
                rounds_with_no_peers += 1;
                // TODO: magic number
                if rounds_with_no_peers > 3 {
                    if let Err(err) = self.initialize_cluster(discovered_seeds).await {
                        tracing::error!(?err, "failed to initialize cluster");
                    }
                    return Ok(());
                }
            } else {
                let clusters_to_peers = discovered_seeds
                    .into_iter()
                    .filter_map(|(k, v)| {
                        if let Some(cluster) = v.cluster {
                            if cluster.last_committed_log_id.is_none() {
                                None
                            } else if let Some(cluster_id) = cluster.cluster_id {
                                Some((cluster_id, (k, v.node_id, cluster)))
                            } else {
                                tracing::warn!("found a last_committed_log_id, but no cluster_id! refusing to join");
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .into_group_map();

                if clusters_to_peers.len() == 1 {
                    let (cluster_id, config) = clusters_to_peers
                        .into_iter()
                        .next()
                        .expect("length already validated as non-zero");
                    if let Err(err) = self.join_cluster(cluster_id, config).await {
                        tracing::error!(?err, "failed to join cluster!");
                    } else {
                        tracing::info!("finished joining cluster");
                        return Ok(());
                    }
                } else {
                    tracing::warn!("found multiple live clusters");
                }
            }
            let sleep_time = Duration::from_millis(rand::random_range(100..=1000));
            if token
                .run_until_cancelled(tokio::time::sleep(sleep_time))
                .await
                .is_none()
            {
                return Ok(());
            }
        }
        Ok(())
    }
}

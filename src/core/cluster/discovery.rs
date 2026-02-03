use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures_util::{StreamExt, stream};
use itertools::Itertools;
use openraft::ServerState;
use tap::Pipe;
use url::Url;

use super::network::build_client;
use super::raft::{Node, NodeId, Raft};
use crate::core::cluster::network::detect_address;
use crate::core::cluster::proto::{
    AddLearnerRequest, DiscoverClusterResponse, DiscoverResponse, UpgradeLearnerRequest,
};
use crate::{Configuration, shutting_down_token};

const CONCURRENT_FETCHES: usize = 8;

pub(super) struct Discovery {
    client: reqwest::Client,
    my_node_id: NodeId,
    raft: Raft,
    cfg: Configuration,
    my_addr: SocketAddr,
}

impl Discovery {
    pub(super) fn new(cfg: Configuration, raft: Raft, my_node_id: NodeId) -> anyhow::Result<Self> {
        let client = build_client(&cfg, cfg.cluster.discovery_request_timeout)?;
        Ok(Self {
            my_addr: detect_address(&cfg)?,
            client,
            cfg,
            raft,
            my_node_id,
        })
    }

    async fn poll_seeds(&self) -> (Option<SocketAddr>, BTreeMap<SocketAddr, DiscoverResponse>) {
        let mut responses = self.cfg.cluster.seed_nodes.iter().copied().map(|s| async move {
                    let url_string = format!("http://{s}/repl/discover");
                    let url = match Url::parse(&url_string) {
                        Ok(url) => url,
                        Err(err) => {
                            tracing::warn!(peer=?s, ?err, "invalid seed node");
                            return None;
                        }
                    };
                    let response = match self.client.get(url).send().await {
                        Ok(resp) => resp,
                        Err(err) => {
                            tracing::warn!(peer=?s, ?err, "unable to poll seed node");
                            return None;
                        }
                    };
                    let response: DiscoverResponse = match response.json().await {
                        Ok(resp) => resp,
                        Err(err) => {
                            tracing::warn!(peer=?s, ?err, "unable to read response body from seed node");
                            return None
                        }
                    };
                    Some((s, response))
                }).pipe(stream::iter)
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
        (my_addr, responses)
    }

    async fn join_cluster(
        &self,
        cluster_name: String,
        peers: Vec<(SocketAddr, NodeId, DiscoverClusterResponse)>,
    ) -> anyhow::Result<()> {
        tracing::info!(?cluster_name, "joining running cluster");
        let Some((leader_addr, _leader_node_id, leader_cluster)) = peers
            .iter()
            .find_or_first(|p| p.2.state == ServerState::Leader)
        else {
            anyhow::bail!("failed to find any peers to join!");
        };
        let Some(log_id) = leader_cluster.last_committed_log_id else {
            anyhow::bail!("existing cluster has no logs");
        };
        // TODO: if any of these steps fail, how do we recover?
        let url = format!("http://{leader_addr}/repl/raft/admin/add-learner");
        let request = AddLearnerRequest {
            node_id: self.my_node_id,
            address: self.my_addr.to_string(),
        };
        self.client.post(url).json(&request).send().await?;
        tracing::debug!("waiting to catch up in replication");
        self.raft
            .wait(None)
            .log_index_at_least(Some(log_id.index), "waiting to catch up to leader")
            .await?;
        tracing::debug!("adding self as a full member");
        let url = format!("http://{leader_addr}/repl/raft/admin/upgrade-learner");
        let request = UpgradeLearnerRequest {
            node_id: self.my_node_id,
        };
        self.client.post(url).json(&request).send().await?;
        Ok(())
    }

    async fn initialize_cluster(
        &self,
        discovered_seeds: BTreeMap<SocketAddr, DiscoverResponse>,
    ) -> anyhow::Result<()> {
        if !self.cfg.cluster.auto_initialize {
            tracing::warn!(
                "auto-initialization disabled, but we found no peers. You must manually initialize!"
            );
            return Ok(());
        }
        let my_node = Node::new(self.my_addr);
        let mut nodes = [(self.my_node_id, my_node)]
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        for (peer_address, response) in discovered_seeds {
            tracing::trace!(?peer_address, ?response, "adding node to new cluster");
            nodes.insert(response.node_id, Node::new(peer_address));
        }
        self.raft.initialize(nodes).await?;
        tracing::info!("initialized new cluster");
        Ok(())
    }

    pub(super) async fn discover_cluster(mut self) -> anyhow::Result<()> {
        if self.cfg.cluster.seed_nodes.is_empty() {
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
            if let Some(addr) = my_addr {
                self.my_addr = addr;
            }
            tracing::debug!(?discovered_seeds, "discovered peers");
            if let Some(addr) = my_addr {
                tracing::debug!(?addr, "discovered my seed address")
            };

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
                            } else {
                                Some((cluster.cluster_name.clone(), (k, v.node_id, cluster)))
                            }
                        } else {
                            None
                        }
                    })
                    .into_group_map();

                if clusters_to_peers.len() == 1 {
                    let (cluster_name, config) = clusters_to_peers.into_iter().next().unwrap();
                    if let Err(err) = self.join_cluster(cluster_name, config).await {
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

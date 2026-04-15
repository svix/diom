// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ClusterAdminArgs {
    #[command(subcommand)]
    pub command: ClusterAdminCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum ClusterAdminCommands {
    /// Get information about the current cluster
    #[command(after_long_help = "\x1b[1;4mExample response:\x1b[0m
{
  \"cluster_id\": \"...\",
  \"cluster_name\": \"...\",
  \"this_node_id\": \"...\",
  \"this_node_state\": {\"...\": \"...\"},
  \"this_node_last_committed_timestamp\": 1234567890123,
  \"this_node_last_snapshot_id\": \"...\",
  \"nodes\": [{\"node_id\": \"...\", \"address\": \"...\", \"state\": {\"...\": \"...\"}, \"last_committed_log_index\": 123, \"last_committed_term\": 123}]
}")]
    Status {},
    /// Initialize this node as the leader of a new cluster
    ///
    /// This operation may only be performed against a node which has not been
    /// initialized and is not currently a member of a cluster.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"cluster_id\": \"...\"
}")]
    Initialize {
        cluster_initialize_in: Option<crate::json::JsonOf<diom::models::ClusterInitializeIn>>,
    },
    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"node_id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"node_id\": \"...\"
}")]
    RemoveNode {
        cluster_remove_node_in: crate::json::JsonOf<diom::models::ClusterRemoveNodeIn>,
    },
    /// Force the cluster to take a snapshot immediately
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"snapshot_time\": 1234567890123,
  \"snapshot_log_index\": 123,
  \"snapshot_id\": \"...\"
}")]
    ForceSnapshot {
        cluster_force_snapshot_in:
            Option<crate::json::JsonOf<diom::models::ClusterForceSnapshotIn>>,
    },
}

impl ClusterAdminCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Status {} => {
                let resp = client.cluster_admin().status().await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Initialize {
                cluster_initialize_in,
            } => {
                let resp = client
                    .cluster_admin()
                    .initialize(cluster_initialize_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::RemoveNode {
                cluster_remove_node_in,
            } => {
                let resp = client
                    .cluster_admin()
                    .remove_node(cluster_remove_node_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::ForceSnapshot {
                cluster_force_snapshot_in,
            } => {
                let resp = client
                    .cluster_admin()
                    .force_snapshot(cluster_force_snapshot_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

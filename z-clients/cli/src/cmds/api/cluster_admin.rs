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
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom cluster-admin status\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"cluster_id\": \"...\", // The unique ID of this cluster.
  \"cluster_name\": \"...\", // The name of this cluster (as defined in the config)
  \"this_node_id\": \"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8\", // The unique ID of the node servicing this request
  \"this_node_state\": \"leader\", // The cluster state of the node servicing this request
  \"this_node_last_committed_timestamp\": 1234567890123, // The timestamp of the last transaction committed on this node
  \"this_node_last_snapshot_id\": \"...\", // The last snapshot taken on this node
  \"this_node_last_purged_log_index\": 123, // The last-purged log on this node
  \"nodes\": [{\"node_id\": \"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8\", \"address\": \"...\", \"state\": \"leader\", \"last_committed_log_index\": 123, \"last_committed_term\": 123}] // A list of all nodes known to be in the cluster
}\n")]
    Status {},
    /// Initialize this node as the leader of a new cluster
    ///
    /// This operation may only be performed against a node which has not been
    /// initialized and is not currently a member of a cluster.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom cluster-admin initialize {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
}\n\nExample response:
{
  \"cluster_id\": \"...\"
}\n")]
    Initialize {
        cluster_initialize_in: Option<crate::json::JsonOf<diom::models::ClusterInitializeIn>>,
    },
    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom cluster-admin remove-node {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"node_id\": \"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8\"
}\n\nExample response:
{
  \"node_id\": \"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8\"
}\n")]
    RemoveNode {
        cluster_remove_node_in: crate::json::JsonOf<diom::models::ClusterRemoveNodeIn>,
    },
    /// Force the cluster to take a snapshot immediately
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom cluster-admin force-snapshot {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
}\n\nExample response:
{
  \"snapshot_time\": 1234567890123, // The wall-clock time at which the snapshot was initiated
  \"snapshot_log_index\": 123, // The log index at which the snapshot was initiated
  \"snapshot_id\": \"...\" // If this is `null`, the snapshot is still building in the background
}\n")]
    ForceSnapshot {
        cluster_force_snapshot_in:
            Option<crate::json::JsonOf<diom::models::ClusterForceSnapshotIn>>,
    },
    /// Force the cluster to conduct an election immediately
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom cluster-admin force-election {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
}\n\nExample response:
{
  \"previous_leader_id\": \"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8\",
  \"new_leader_id\": \"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8\"
}\n")]
    ForceElection {
        cluster_force_election_in:
            Option<crate::json::JsonOf<diom::models::ClusterForceElectionIn>>,
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
            Self::ForceElection {
                cluster_force_election_in,
            } => {
                let resp = client
                    .cluster_admin()
                    .force_election(cluster_force_election_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

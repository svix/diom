// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminClusterArgs {
    #[command(subcommand)]
    pub command: AdminClusterCommands,
}

#[derive(Subcommand)]
pub enum AdminClusterCommands {
    /// Get information about the current cluster
    #[command(after_long_help = "\x1b[1;4mExample response:\x1b[0m
{
  \"cluster_id\": \"...\",
  \"cluster_name\": \"...\",
  \"this_node_id\": \"...\",
  \"this_node_state\": \"...\",
  \"this_node_last_committed_timestamp\": \"...\",
  \"this_node_last_snapshot_id\": \"...\",
  \"nodes\": \"...\"
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
        cluster_initialize_in:
            Option<crate::json::JsonOf<diom_client::models::ClusterInitializeIn>>,
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
        cluster_remove_node_in: crate::json::JsonOf<diom_client::models::ClusterRemoveNodeIn>,
    },
    /// Force the cluster to take a snapshot immediately
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"snapshot_time\": \"...\",
  \"snapshot_log_index\": \"...\",
  \"snapshot_id\": \"...\"
}")]
    ForceSnapshot {
        cluster_force_snapshot_in:
            Option<crate::json::JsonOf<diom_client::models::ClusterForceSnapshotIn>>,
    },
}

impl AdminClusterCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Status {} => {
                let resp = client.admin().cluster().status().await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Initialize {
                cluster_initialize_in,
            } => {
                let resp = client
                    .admin()
                    .cluster()
                    .initialize(cluster_initialize_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::RemoveNode {
                cluster_remove_node_in,
            } => {
                let resp = client
                    .admin()
                    .cluster()
                    .remove_node(cluster_remove_node_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::ForceSnapshot {
                cluster_force_snapshot_in,
            } => {
                let resp = client
                    .admin()
                    .cluster()
                    .force_snapshot(cluster_force_snapshot_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

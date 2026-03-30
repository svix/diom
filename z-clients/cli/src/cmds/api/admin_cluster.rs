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
    Status {},
    /// Initialize this node as the leader of a new cluster
    ///
    /// This operation may only be performed against a node which has not been
    /// initialized and is not currently a member of a cluster.
    Initialize {
        cluster_initialize_in:
            Option<crate::json::JsonOf<diom_client::models::ClusterInitializeIn>>,
    },
    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    RemoveNode {
        cluster_remove_node_in: crate::json::JsonOf<diom_client::models::ClusterRemoveNodeIn>,
    },
    /// Force the cluster to take a snapshot immediately
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

// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

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
    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    RemoveNode {
        cluster_remove_node_in: crate::json::JsonOf<coyote_client::models::ClusterRemoveNodeIn>,
    },
}

impl AdminClusterCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Status {} => {
                let resp = client.admin().cluster().status().await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RemoveNode {
                cluster_remove_node_in,
            } => {
                let resp = client
                    .admin()
                    .cluster()
                    .remove_node(cluster_remove_node_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

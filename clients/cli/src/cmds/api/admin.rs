// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommands,
}

#[derive(Subcommand)]
pub enum AdminCommands {
    /// Get information about the current cluster
    ClusterStatus {},
    /// Remove a node from the cluster.
    ///
    /// This operation executes immediately and the node must be wiped and reset
    /// before it can safely be added to the cluster.
    ClusterRemoveNode {
        cluster_remove_node_in: crate::json::JsonOf<diom_client::models::ClusterRemoveNodeIn>,
    },
}

impl AdminCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::ClusterStatus {} => {
                let resp = client.admin().cluster_status().await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ClusterRemoveNode {
                cluster_remove_node_in,
            } => {
                let resp = client
                    .admin()
                    .cluster_remove_node(cluster_remove_node_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

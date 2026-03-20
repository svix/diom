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
}

impl AdminClusterCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Status {} => {
                let resp = client.admin().cluster().status().await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

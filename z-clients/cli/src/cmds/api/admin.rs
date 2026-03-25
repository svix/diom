// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

use super::AdminClusterArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommands,
}

#[derive(Subcommand)]
pub enum AdminCommands {
    Cluster(AdminClusterArgs),
}

impl AdminCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Cluster(args) => {
                args.command.exec(client, color_mode).await?;
            }
        }

        Ok(())
    }
}

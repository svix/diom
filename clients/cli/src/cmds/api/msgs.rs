// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

use super::MsgsNamespaceArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsArgs {
    #[command(subcommand)]
    pub command: MsgsCommands,
}

#[derive(Subcommand)]
pub enum MsgsCommands {
    Namespace(MsgsNamespaceArgs),
}

impl MsgsCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client, color_mode).await?;
            }
        }

        Ok(())
    }
}

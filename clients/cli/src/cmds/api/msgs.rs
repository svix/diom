// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

use super::MsgsTopicArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsArgs {
    #[command(subcommand)]
    pub command: MsgsCommands,
}

#[derive(Subcommand)]
pub enum MsgsCommands {
    Topic(MsgsTopicArgs),
}

impl MsgsCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Topic(args) => {
                args.command.exec(client, color_mode).await?;
            }
        }

        Ok(())
    }
}

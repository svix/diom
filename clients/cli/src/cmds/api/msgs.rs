// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

use super::{MsgsNamespaceArgs, MsgsStreamArgs};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsArgs {
    #[command(subcommand)]
    pub command: MsgsCommands,
}

#[derive(Subcommand)]
pub enum MsgsCommands {
    Namespace(MsgsNamespaceArgs),
    Stream(MsgsStreamArgs),
    /// Publishes messages to a topic within a namespace.
    Publish {
        publish_in: crate::json::JsonOf<coyote_client::models::PublishIn>,
    },
}

impl MsgsCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Stream(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Publish { publish_in } => {
                let resp = client.msgs().publish(publish_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsArgs {
    #[command(subcommand)]
    pub command: MsgsCommands,
}

#[derive(Subcommand)]
pub enum MsgsCommands {}

impl MsgsCommands {
    pub async fn exec(
        self,
        _client: &CoyoteClient,
        _color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

use super::IdempotencyNamespaceArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IdempotencyArgs {
    #[command(subcommand)]
    pub command: IdempotencyCommands,
}

#[derive(Subcommand)]
pub enum IdempotencyCommands {
    Namespace(IdempotencyNamespaceArgs),
    /// Abandon an idempotent request (remove lock without saving response)
    Abort {
        idempotency_abort_in: crate::json::JsonOf<diom_client::models::IdempotencyAbortIn>,
    },
}

impl IdempotencyCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Abort {
                idempotency_abort_in,
            } => {
                let resp = client
                    .idempotency()
                    .abort(idempotency_abort_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

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
    /// Start an idempotent request
    Start {
        key: String,
        idempotency_start_in: crate::json::JsonOf<diom_client::models::IdempotencyStartIn>,
    },
    /// Complete an idempotent request with a response
    Complete {
        key: String,
        idempotency_complete_in: crate::json::JsonOf<diom_client::models::IdempotencyCompleteIn>,
    },
    /// Abandon an idempotent request (remove lock without saving response)
    Abort {
        key: String,
        idempotency_abort_in: crate::json::JsonOf<diom_client::models::IdempotencyAbortIn>,
    },
}

impl IdempotencyCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Start {
                key,
                idempotency_start_in,
            } => {
                let resp = client
                    .idempotency()
                    .start(key, idempotency_start_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Complete {
                key,
                idempotency_complete_in,
            } => {
                let resp = client
                    .idempotency()
                    .complete(key, idempotency_complete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Abort {
                key,
                idempotency_abort_in,
            } => {
                let resp = client
                    .idempotency()
                    .abort(key, idempotency_abort_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

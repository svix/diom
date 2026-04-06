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
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"ttl_ms\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Start {
        key: String,
        idempotency_start_in: crate::json::JsonOf<diom_client::models::IdempotencyStartIn>,
    },
    /// Complete an idempotent request with a response
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"response\": \"...\",
  \"ttl_ms\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Complete {
        key: String,
        idempotency_complete_in: crate::json::JsonOf<diom_client::models::IdempotencyCompleteIn>,
    },
    /// Abandon an idempotent request (remove lock without saving response)
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Abort {
        key: String,
        idempotency_abort_in:
            Option<crate::json::JsonOf<diom_client::models::IdempotencyAbortIn>>,
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
                    .abort(key, idempotency_abort_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

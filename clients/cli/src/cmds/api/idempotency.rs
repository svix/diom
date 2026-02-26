// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IdempotencyArgs {
    #[command(subcommand)]
    pub command: IdempotencyCommands,
}

#[derive(Subcommand)]
pub enum IdempotencyCommands {
    /// Abandon an idempotent request (remove lock without saving response)
    Abort {
        idempotency_abort_in: crate::json::JsonOf<coyote_client::models::IdempotencyAbortIn>,
    },
    /// Get idempotency namespace
    GetNamespace {
        idempotency_get_namespace_in:
            crate::json::JsonOf<coyote_client::models::IdempotencyGetNamespaceIn>,
    },
}

impl IdempotencyCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Abort {
                idempotency_abort_in,
            } => {
                let resp = client
                    .idempotency()
                    .abort(idempotency_abort_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetNamespace {
                idempotency_get_namespace_in,
            } => {
                let resp = client
                    .idempotency()
                    .get_namespace(idempotency_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

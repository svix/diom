// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IdempotencyNamespaceArgs {
    #[command(subcommand)]
    pub command: IdempotencyNamespaceCommands,
}

#[derive(Subcommand)]
pub enum IdempotencyNamespaceCommands {
    /// Create idempotency namespace
    Create {
        idempotency_create_namespace_in:
            crate::json::JsonOf<diom_client::models::IdempotencyCreateNamespaceIn>,
    },
    /// Get idempotency namespace
    Get {
        idempotency_get_namespace_in:
            crate::json::JsonOf<diom_client::models::IdempotencyGetNamespaceIn>,
    },
}

impl IdempotencyNamespaceCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                idempotency_create_namespace_in,
            } => {
                let resp = client
                    .idempotency()
                    .namespace()
                    .create(idempotency_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                idempotency_get_namespace_in,
            } => {
                let resp = client
                    .idempotency()
                    .namespace()
                    .get(idempotency_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

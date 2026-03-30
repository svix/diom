// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimitNamespaceArgs {
    #[command(subcommand)]
    pub command: RateLimitNamespaceCommands,
}

#[derive(Subcommand)]
pub enum RateLimitNamespaceCommands {
    /// Create rate limiter namespace
    Create {
        rate_limit_create_namespace_in:
            crate::json::JsonOf<diom_client::models::RateLimitCreateNamespaceIn>,
    },
    /// Get rate limiter namespace
    Get {
        rate_limit_get_namespace_in:
            crate::json::JsonOf<diom_client::models::RateLimitGetNamespaceIn>,
    },
}

impl RateLimitNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Create {
                rate_limit_create_namespace_in,
            } => {
                let resp = client
                    .rate_limit()
                    .namespace()
                    .create(rate_limit_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                rate_limit_get_namespace_in,
            } => {
                let resp = client
                    .rate_limit()
                    .namespace()
                    .get(rate_limit_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

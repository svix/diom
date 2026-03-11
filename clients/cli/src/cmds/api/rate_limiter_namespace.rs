// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimiterNamespaceArgs {
    #[command(subcommand)]
    pub command: RateLimiterNamespaceCommands,
}

#[derive(Subcommand)]
pub enum RateLimiterNamespaceCommands {
    /// Create rate limiter namespace
    Create {
        rate_limiter_create_namespace_in:
            crate::json::JsonOf<coyote_client::models::RateLimiterCreateNamespaceIn>,
    },
    /// Get rate limiter namespace
    Get {
        rate_limiter_get_namespace_in:
            crate::json::JsonOf<coyote_client::models::RateLimiterGetNamespaceIn>,
    },
}

impl RateLimiterNamespaceCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                rate_limiter_create_namespace_in,
            } => {
                let resp = client
                    .rate_limiter()
                    .namespace()
                    .create(rate_limiter_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                rate_limiter_get_namespace_in,
            } => {
                let resp = client
                    .rate_limiter()
                    .namespace()
                    .get(rate_limiter_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

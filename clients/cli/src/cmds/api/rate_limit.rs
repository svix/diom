// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

use super::RateLimitNamespaceArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimitArgs {
    #[command(subcommand)]
    pub command: RateLimitCommands,
}

#[derive(Subcommand)]
pub enum RateLimitCommands {
    Namespace(RateLimitNamespaceArgs),
    /// Rate Limiter Check and Consume
    Limit {
        rate_limit_check_in: crate::json::JsonOf<coyote_client::models::RateLimitCheckIn>,
    },
    /// Rate Limiter Get Remaining
    GetRemaining {
        rate_limit_get_remaining_in:
            crate::json::JsonOf<coyote_client::models::RateLimitGetRemainingIn>,
    },
}

impl RateLimitCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Limit {
                rate_limit_check_in,
            } => {
                let resp = client
                    .rate_limit()
                    .limit(rate_limit_check_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetRemaining {
                rate_limit_get_remaining_in,
            } => {
                let resp = client
                    .rate_limit()
                    .get_remaining(rate_limit_get_remaining_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

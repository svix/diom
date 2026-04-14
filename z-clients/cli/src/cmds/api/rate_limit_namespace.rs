// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimitNamespaceArgs {
    #[command(subcommand)]
    pub command: RateLimitNamespaceCommands,
}

#[derive(Subcommand)]
pub enum RateLimitNamespaceCommands {
    /// Create rate limiter namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Create {
        rate_limit_create_namespace_in:
            crate::json::JsonOf<diom::models::RateLimitCreateNamespaceIn>,
    },
    /// Get rate limiter namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Get {
        rate_limit_get_namespace_in: crate::json::JsonOf<diom::models::RateLimitGetNamespaceIn>,
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

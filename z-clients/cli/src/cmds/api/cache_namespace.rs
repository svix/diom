// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct CacheNamespaceArgs {
    #[command(subcommand)]
    pub command: CacheNamespaceCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum CacheNamespaceCommands {
    /// Configure cache namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"eviction_policy\": {\"...\": \"...\"}
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"eviction_policy\": {\"...\": \"...\"},
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}")]
    Configure {
        cache_configure_namespace_in: crate::json::JsonOf<diom::models::CacheConfigureNamespaceIn>,
    },
    /// Get cache namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"eviction_policy\": {\"...\": \"...\"},
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}")]
    Get {
        cache_get_namespace_in: crate::json::JsonOf<diom::models::CacheGetNamespaceIn>,
    },
}

impl CacheNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                cache_configure_namespace_in,
            } => {
                let resp = client
                    .cache()
                    .namespace()
                    .configure(cache_configure_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                cache_get_namespace_in,
            } => {
                let resp = client
                    .cache()
                    .namespace()
                    .get(cache_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

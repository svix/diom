// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

use super::CacheNamespaceArgs;
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum CacheCommands {
    Namespace(CacheNamespaceArgs),
    /// Cache Set
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"ttl_ms\": 60000
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Set {
        key: String,
        value: ByteString,
        cache_set_in: crate::json::JsonOf<diom::models::CacheSetIn>,
    },
    /// Cache Get
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"consistency\": \"strong\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"expiry\": 1234567890123,
  \"value\": \"dGVzdA==\"
}")]
    Get {
        key: String,
        cache_get_in: Option<crate::json::JsonOf<diom::models::CacheGetIn>>,
    },
    /// Cache Delete
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": true
}")]
    Delete {
        key: String,
        cache_delete_in: Option<crate::json::JsonOf<diom::models::CacheDeleteIn>>,
    },
}

impl CacheCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Set {
                key,
                value,
                cache_set_in,
            } => {
                let resp = client
                    .cache()
                    .set(key, value.into(), cache_set_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get { key, cache_get_in } => {
                let resp = client
                    .cache()
                    .get(key, cache_get_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete {
                key,
                cache_delete_in,
            } => {
                let resp = client
                    .cache()
                    .delete(key, cache_delete_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

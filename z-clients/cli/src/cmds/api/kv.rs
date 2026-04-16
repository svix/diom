// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

use super::KvNamespaceArgs;
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvArgs {
    #[command(subcommand)]
    pub command: KvCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum KvCommands {
    Namespace(KvNamespaceArgs),
    /// KV Set
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"ttl_ms\": 60000,
  \"behavior\": \"upsert\",
  \"version\": 123
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"version\": 123
}")]
    Set {
        key: String,
        value: ByteString,
        kv_set_in: Option<crate::json::JsonOf<diom::models::KvSetIn>>,
    },
    /// KV Get
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"consistency\": \"strong\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"expiry\": 1234567890123,
  \"value\": \"...\",
  \"version\": 123
}")]
    Get {
        key: String,
        kv_get_in: Option<crate::json::JsonOf<diom::models::KvGetIn>>,
    },
    /// KV Delete
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"version\": 123
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": true
}")]
    Delete {
        key: String,
        kv_delete_in: Option<crate::json::JsonOf<diom::models::KvDeleteIn>>,
    },
}

impl KvCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Set {
                key,
                value,
                kv_set_in,
            } => {
                let resp = client
                    .kv()
                    .set(
                        key,
                        value.into(),
                        kv_set_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get { key, kv_get_in } => {
                let resp = client
                    .kv()
                    .get(key, kv_get_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete { key, kv_delete_in } => {
                let resp = client
                    .kv()
                    .delete(key, kv_delete_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

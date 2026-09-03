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
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom kv set KEY VALUE {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"ttl_ms\": 60000, // Time to live in milliseconds
  \"behavior\": \"upsert\",
  \"version\": 123 // If set, the write only succeeds when the stored version matches this value. Use the `version` field from a prior `get` response.
}\n\nExample response:
{
  \"version\": 123
}\n")]
    Set {
        key: String,
        value: ByteString,
        kv_set_in: Option<crate::json::JsonOf<diom::models::KvSetIn>>,
    },
    /// KV Get
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom kv get KEY {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"consistency\": \"strong\"
}\n\nExample response:
{
  \"expiry\": 1234567890123, // Time of expiry
  \"value\": \"...\",
  \"version\": 123 // Opaque version token for optimistic concurrency control. Pass as `version` in a subsequent `set` to perform a conditional write.
}\n")]
    Get {
        key: String,
        kv_get_in: Option<crate::json::JsonOf<diom::models::KvGetIn>>,
    },
    /// KV Delete
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom kv delete KEY {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"version\": 123 // If set, the delete only succeeds when the stored version matches this value. Use the `version` field from a prior `get` response.
}\n\nExample response:
{
  \"success\": true // Whether the operation succeeded or was a noop due to pre-conditions.
}\n")]
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

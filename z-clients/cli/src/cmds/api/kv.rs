// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

use super::KvNamespaceArgs;
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvArgs {
    #[command(subcommand)]
    pub command: KvCommands,
}

#[derive(Subcommand)]
pub enum KvCommands {
    Namespace(KvNamespaceArgs),
    /// KV Set
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"value\": \"...\",
  \"ttl_ms\": \"...\",
  \"behavior\": \"...\",
  \"version\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": \"...\",
  \"version\": \"...\"
}")]
    Set {
        key: String,
        kv_set_in: crate::json::JsonOf<coyote_client::models::KvSetIn>,
    },
    /// KV Get
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"consistency\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"expiry\": \"...\",
  \"value\": \"...\",
  \"version\": \"...\"
}")]
    Get {
        key: String,
        kv_get_in: Option<crate::json::JsonOf<coyote_client::models::KvGetIn>>,
    },
    /// KV Delete
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"version\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": \"...\"
}")]
    Delete {
        key: String,
        kv_delete_in: Option<crate::json::JsonOf<coyote_client::models::KvDeleteIn>>,
    },
}

impl KvCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Set { key, kv_set_in } => {
                let resp = client.kv().set(key, kv_set_in.into_inner()).await?;
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

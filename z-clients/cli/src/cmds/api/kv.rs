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
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  value
  namespace (optional)
  ttl_ms (optional) — Time to live in milliseconds
  behavior (optional)
  version (optional) — If set, the write only succeeds when the stored version matches this value. Use the `version` field from a prior `get` response.")]
    Set {
        key: String,
        kv_set_in: crate::json::JsonOf<coyote_client::models::KvSetIn>,
    },
    /// KV Get
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  namespace (optional)
  consistency (optional)")]
    Get {
        key: String,
        kv_get_in: Option<crate::json::JsonOf<coyote_client::models::KvGetIn>>,
    },
    /// KV Delete
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  namespace (optional)")]
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

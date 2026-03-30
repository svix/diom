// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

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
    Set {
        key: String,
        kv_set_in: crate::json::JsonOf<diom_client::models::KvSetIn>,
    },
    /// KV Get
    Get {
        key: String,
        kv_get_in: crate::json::JsonOf<diom_client::models::KvGetIn>,
    },
    /// KV Delete
    Delete {
        key: String,
        kv_delete_in: crate::json::JsonOf<diom_client::models::KvDeleteIn>,
    },
}

impl KvCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Set { key, kv_set_in } => {
                let resp = client.kv().set(key, kv_set_in.into_inner()).await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get { key, kv_get_in } => {
                let resp = client.kv().get(key, kv_get_in.into_inner()).await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete { key, kv_delete_in } => {
                let resp = client.kv().delete(key, kv_delete_in.into_inner()).await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

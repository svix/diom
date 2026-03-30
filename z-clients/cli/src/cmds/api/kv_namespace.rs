// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvNamespaceArgs {
    #[command(subcommand)]
    pub command: KvNamespaceCommands,
}

#[derive(Subcommand)]
pub enum KvNamespaceCommands {
    /// Create KV namespace
    Create {
        kv_create_namespace_in: crate::json::JsonOf<diom_client::models::KvCreateNamespaceIn>,
    },
    /// Get KV namespace
    Get {
        kv_get_namespace_in: crate::json::JsonOf<diom_client::models::KvGetNamespaceIn>,
    },
}

impl KvNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Create {
                kv_create_namespace_in,
            } => {
                let resp = client
                    .kv()
                    .namespace()
                    .create(kv_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                kv_get_namespace_in,
            } => {
                let resp = client
                    .kv()
                    .namespace()
                    .get(kv_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

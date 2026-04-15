// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvNamespaceArgs {
    #[command(subcommand)]
    pub command: KvNamespaceCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum KvNamespaceCommands {
    /// Configure KV namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Configure {
        kv_configure_namespace_in: crate::json::JsonOf<diom::models::KvConfigureNamespaceIn>,
    },
    /// Get KV namespace
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
        kv_get_namespace_in: crate::json::JsonOf<diom::models::KvGetNamespaceIn>,
    },
}

impl KvNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                kv_configure_namespace_in,
            } => {
                let resp = client
                    .kv()
                    .namespace()
                    .configure(kv_configure_namespace_in.into_inner())
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

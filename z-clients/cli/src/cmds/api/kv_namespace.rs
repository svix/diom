// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvNamespaceArgs {
    #[command(subcommand)]
    pub command: KvNamespaceCommands,
}

#[derive(Subcommand)]
pub enum KvNamespaceCommands {
    /// Create KV namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Create {
        kv_create_namespace_in: crate::json::JsonOf<diom::models::KvCreateNamespaceIn>,
    },
    /// Get KV namespace
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"...\",
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

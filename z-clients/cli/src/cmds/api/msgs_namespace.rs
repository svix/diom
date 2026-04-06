// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsNamespaceArgs {
    #[command(subcommand)]
    pub command: MsgsNamespaceCommands,
}

#[derive(Subcommand)]
pub enum MsgsNamespaceCommands {
    /// Creates or updates a msgs namespace with the given name.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"retention\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"...\",
  \"retention\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Create {
        name: String,
        msg_namespace_create_in:
            Option<crate::json::JsonOf<coyote_client::models::MsgNamespaceCreateIn>>,
    },
    /// Gets a msgs namespace by name.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"...\",
  \"retention\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Get {
        name: String,
        msg_namespace_get_in: Option<crate::json::JsonOf<coyote_client::models::MsgNamespaceGetIn>>,
    },
}

impl MsgsNamespaceCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
        match self {
            Self::Create {
                name,
                msg_namespace_create_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .create(
                        name,
                        msg_namespace_create_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                name,
                msg_namespace_get_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .get(name, msg_namespace_get_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthPolicyArgs {
    #[command(subcommand)]
    pub command: AdminAuthPolicyCommands,
}

#[derive(Subcommand)]
pub enum AdminAuthPolicyCommands {
    /// Create or update an access policy
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"description\": \"...\",
  \"rules\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Configure {
        admin_access_policy_configure_in:
            crate::json::JsonOf<diom::models::AdminAccessPolicyConfigureIn>,
    },
    /// Delete an access policy
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": \"...\"
}")]
    Delete {
        admin_access_policy_delete_in: crate::json::JsonOf<diom::models::AdminAccessPolicyDeleteIn>,
    },
    /// Get an access policy by ID
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"description\": \"...\",
  \"rules\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Get {
        admin_access_policy_get_in: crate::json::JsonOf<diom::models::AdminAccessPolicyGetIn>,
    },
    /// List all access policies
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"limit\": \"...\",
  \"iterator\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"data\": \"...\",
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": \"...\"
}")]
    List {
        admin_access_policy_list_in:
            Option<crate::json::JsonOf<diom::models::AdminAccessPolicyListIn>>,
    },
}

impl AdminAuthPolicyCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                admin_access_policy_configure_in,
            } => {
                let resp = client
                    .admin()
                    .auth_policy()
                    .configure(admin_access_policy_configure_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete {
                admin_access_policy_delete_in,
            } => {
                let resp = client
                    .admin()
                    .auth_policy()
                    .delete(admin_access_policy_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                admin_access_policy_get_in,
            } => {
                let resp = client
                    .admin()
                    .auth_policy()
                    .get(admin_access_policy_get_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::List {
                admin_access_policy_list_in,
            } => {
                let resp = client
                    .admin()
                    .auth_policy()
                    .list(admin_access_policy_list_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

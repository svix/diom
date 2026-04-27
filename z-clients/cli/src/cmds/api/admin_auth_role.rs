// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthRoleArgs {
    #[command(subcommand)]
    pub command: AdminAuthRoleCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum AdminAuthRoleCommands {
    /// Create or update a role
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-role configure {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"description\": \"...\",
  \"rules\": [{\"effect\": \"allow\", \"resource\": \"...\", \"actions\": [\"...\"]}],
  \"policies\": [\"...\"],
  \"context\": {\"key\": \"...\"}
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Configure {
        admin_role_configure_in: crate::json::JsonOf<diom::models::AdminRoleConfigureIn>,
    },
    /// Delete a role
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-role delete {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": true
}\n")]
    Delete {
        admin_role_delete_in: crate::json::JsonOf<diom::models::AdminRoleDeleteIn>,
    },
    /// Get a role by ID
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-role get {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"description\": \"...\",
  \"rules\": [{\"effect\": \"allow\", \"resource\": \"...\", \"actions\": [\"...\"]}],
  \"policies\": [\"...\"],
  \"context\": {\"key\": \"...\"},
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Get {
        admin_role_get_in: crate::json::JsonOf<diom::models::AdminRoleGetIn>,
    },
    /// List all roles
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-role list {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"limit\": 123,
  \"iterator\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"data\": [{\"id\": \"...\", \"description\": \"...\", \"rules\": [{\"effect\": \"allow\", \"resource\": \"...\", \"actions\": [\"...\"]}], \"policies\": [\"...\"], \"context\": {\"key\": \"...\"}, \"created\": 1234567890123, \"updated\": 1234567890123}],
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": true
}\n")]
    List {
        admin_role_list_in: Option<crate::json::JsonOf<diom::models::AdminRoleListIn>>,
    },
}

impl AdminAuthRoleCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                admin_role_configure_in,
            } => {
                let resp = client
                    .admin()
                    .auth_role()
                    .configure(admin_role_configure_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete {
                admin_role_delete_in,
            } => {
                let resp = client
                    .admin()
                    .auth_role()
                    .delete(admin_role_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get { admin_role_get_in } => {
                let resp = client
                    .admin()
                    .auth_role()
                    .get(admin_role_get_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::List { admin_role_list_in } => {
                let resp = client
                    .admin()
                    .auth_role()
                    .list(admin_role_list_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

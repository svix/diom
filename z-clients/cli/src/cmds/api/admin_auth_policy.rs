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

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum AdminAuthPolicyCommands {
    /// Create or update an access policy
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-policy configure {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"id\": \"some_access_policy\",
  \"description\": \"policy description…\",
  \"rules\": [{\"effect\": \"allow\", \"resource\": \"...\", \"actions\": [\"...\"]}]
}\n\nExample response:
{
  \"id\": \"some_access_policy\",
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Configure {
        admin_access_policy_configure_in:
            crate::json::JsonOf<diom::models::AdminAccessPolicyConfigureIn>,
    },
    /// Delete an access policy
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-policy delete {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"id\": \"some_access_policy\"
}\n\nExample response:
{
  \"success\": true
}\n")]
    Delete {
        admin_access_policy_delete_in: crate::json::JsonOf<diom::models::AdminAccessPolicyDeleteIn>,
    },
    /// Get an access policy by ID
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-policy get {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"id\": \"some_access_policy\"
}\n\nExample response:
{
  \"id\": \"some_access_policy\",
  \"description\": \"...\",
  \"rules\": [{\"effect\": \"allow\", \"resource\": \"...\", \"actions\": [\"...\"]}],
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Get {
        admin_access_policy_get_in: crate::json::JsonOf<diom::models::AdminAccessPolicyGetIn>,
    },
    /// List all access policies
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-policy list {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"limit\": 123,
  \"iterator\": \"some_access_policy\"
}\n\nExample response:
{
  \"data\": [{\"id\": \"some_access_policy\", \"description\": \"...\", \"rules\": [{\"effect\": \"allow\", \"resource\": \"...\", \"actions\": [\"...\"]}], \"created\": 1234567890123, \"updated\": 1234567890123}],
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": true
}\n")]
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

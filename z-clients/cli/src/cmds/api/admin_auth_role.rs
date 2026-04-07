// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthRoleArgs {
    #[command(subcommand)]
    pub command: AdminAuthRoleCommands,
}

#[derive(Subcommand)]
pub enum AdminAuthRoleCommands {
    /// Create or update a role
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"description\": \"...\",
  \"rules\": \"...\",
  \"policies\": \"...\",
  \"context\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Upsert {
        admin_role_upsert_in: crate::json::JsonOf<diom::models::AdminRoleUpsertIn>,
    },
    /// Delete a role
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": \"...\"
}")]
    Delete {
        admin_role_delete_in: crate::json::JsonOf<diom::models::AdminRoleDeleteIn>,
    },
    /// Get a role by ID
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"description\": \"...\",
  \"rules\": \"...\",
  \"policies\": \"...\",
  \"context\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Get {
        admin_role_get_in: crate::json::JsonOf<diom::models::AdminRoleGetIn>,
    },
    /// List all roles
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
        admin_role_list_in: Option<crate::json::JsonOf<diom::models::AdminRoleListIn>>,
    },
}

impl AdminAuthRoleCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Upsert {
                admin_role_upsert_in,
            } => {
                let resp = client
                    .admin()
                    .auth_role()
                    .upsert(admin_role_upsert_in.into_inner())
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

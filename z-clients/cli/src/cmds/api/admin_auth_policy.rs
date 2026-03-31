// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthPolicyArgs {
    #[command(subcommand)]
    pub command: AdminAuthPolicyCommands,
}

#[derive(Subcommand)]
pub enum AdminAuthPolicyCommands {
    /// Create or update an access policy
    Upsert {
        admin_access_policy_upsert_in:
            crate::json::JsonOf<coyote_client::models::AdminAccessPolicyUpsertIn>,
    },
    /// Delete an access policy
    Delete {
        admin_access_policy_delete_in:
            crate::json::JsonOf<coyote_client::models::AdminAccessPolicyDeleteIn>,
    },
    /// Get an access policy by ID
    Get {
        admin_access_policy_get_in:
            crate::json::JsonOf<coyote_client::models::AdminAccessPolicyGetIn>,
    },
    /// List all access policies
    List {
        admin_access_policy_list_in:
            Option<crate::json::JsonOf<coyote_client::models::AdminAccessPolicyListIn>>,
    },
}

impl AdminAuthPolicyCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
        match self {
            Self::Upsert {
                admin_access_policy_upsert_in,
            } => {
                let resp = client
                    .admin()
                    .auth_policy()
                    .upsert(admin_access_policy_upsert_in.into_inner())
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

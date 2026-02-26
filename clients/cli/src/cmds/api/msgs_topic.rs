// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsTopicArgs {
    #[command(subcommand)]
    pub command: MsgsTopicCommands,
}

#[derive(Subcommand)]
pub enum MsgsTopicCommands {
    /// Upserts a new message topic with the given name.
    Create {
        create_msg_topic_in: crate::json::JsonOf<coyote_client::models::CreateMsgTopicIn>,
    },
    /// Get message topic with given name.
    Get {
        get_msg_topic_in: crate::json::JsonOf<coyote_client::models::GetMsgTopicIn>,
    },
}

impl MsgsTopicCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                create_msg_topic_in,
            } => {
                let resp = client
                    .msgs()
                    .topic()
                    .create(create_msg_topic_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { get_msg_topic_in } => {
                let resp = client
                    .msgs()
                    .topic()
                    .get(get_msg_topic_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

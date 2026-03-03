// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsTopicArgs {
    #[command(subcommand)]
    pub command: MsgsTopicCommands,
}

#[derive(Subcommand)]
pub enum MsgsTopicCommands {
    /// Configures the number of partitions for a topic.
    ///
    /// Partition count can only be increased, never decreased. The default for a new topic is 1.
    Configure {
        topic: String,
        msg_topic_configure_in: crate::json::JsonOf<diom_client::models::MsgTopicConfigureIn>,
    },
}

impl MsgsTopicCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                topic,
                msg_topic_configure_in,
            } => {
                let resp = client
                    .msgs()
                    .topic()
                    .configure(topic, msg_topic_configure_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}

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
    /// Configures the number of partitions for a topic.
    ///
    /// Partition count can only be increased, never decreased. The default for a new topic is 1.
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  partitions
  namespace (optional)")]
    Configure {
        topic: String,
        msg_topic_configure_in: crate::json::JsonOf<coyote_client::models::MsgTopicConfigureIn>,
    },
}

impl MsgsTopicCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
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
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

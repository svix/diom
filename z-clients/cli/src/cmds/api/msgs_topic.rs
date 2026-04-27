// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsTopicArgs {
    #[command(subcommand)]
    pub command: MsgsTopicCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsTopicCommands {
    /// Configures the number of partitions for a topic.
    ///
    /// Partition count can only be increased, never decreased. The default for a new topic is 1.
    #[command(help_template = concat!(
                "{about-with-newline}\n",
                "{usage-heading} {usage}\n",
                "{after-help}",
                "\n",
                "{all-args}",
            ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"partitions\": 123
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"partitions\": 123
}\n")]
    Configure {
        topic: String,
        msg_topic_configure_in: crate::json::JsonOf<diom::models::MsgTopicConfigureIn>,
    },
}

impl MsgsTopicCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
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

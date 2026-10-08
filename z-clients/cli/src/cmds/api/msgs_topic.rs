// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub(crate) struct MsgsTopicArgs {
    #[command(subcommand)]
    pub command: MsgsTopicCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub(crate) enum MsgsTopicCommands {
    /// Configures the number of partitions for a topic.
    ///
    /// Partition count can only be increased, never decreased. The default for a new topic is 1.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs topic configure TOPIC {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"partitions\": 123
}\n\nExample response:
{
  \"partitions\": 123
}\n")]
    Configure {
        topic: String,
        msg_topic_configure_in: crate::json::JsonOf<diom::models::MsgTopicConfigureIn>,
    },
    /// List available topics in the given namespace
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs topic list {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"consistency\": \"strong\",
  \"limit\": 123, // Limit the number of returned items
  \"iterator\": \"topiciter_c29tZV90b3BpY19uYW1l\" // The iterator returned from a prior invocation
}\n\nExample response:
{
  \"data\": [{\"id\": \"topic_06etngr201xwv7qj08mt4cs03w\", \"name\": \"some_topic_name\", \"partitions\": 123}],
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": true
}\n")]
    List {
        msg_topic_list_in: Option<crate::json::JsonOf<diom::models::MsgTopicListIn>>,
    },
    /// Show information about the given topic
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs topic describe TOPIC {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"consistency\": \"strong\"
}\n\nExample response:
{
  \"id\": \"topic_06etngr201xwv7qj08mt4cs03w\", // The unique internal ID of this topic
  \"name\": \"some_topic_name\",
  \"partitions\": [{\"partition_id\": 123, \"high_water_mark\": 123}]
}\n")]
    Describe {
        topic: String,
        msg_topic_describe_in: Option<crate::json::JsonOf<diom::models::MsgTopicDescribeIn>>,
    },
}

impl MsgsTopicCommands {
    pub(crate) async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
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
            Self::List { msg_topic_list_in } => {
                let resp = client
                    .msgs()
                    .topic()
                    .list(msg_topic_list_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Describe {
                topic,
                msg_topic_describe_in,
            } => {
                let resp = client
                    .msgs()
                    .topic()
                    .describe(
                        topic,
                        msg_topic_describe_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

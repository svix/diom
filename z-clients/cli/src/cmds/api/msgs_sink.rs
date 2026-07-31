// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsSinkArgs {
    #[command(subcommand)]
    pub command: MsgsSinkCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsSinkCommands {
    /// Create or update a sink for a topic. Overwrites any existing sink with the same id.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs sink configure {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"topic\": \"some_topic_name\",
  \"consumer_group\": \"some_consumer_group\",
  \"default_starting_position\": \"earliest\",
  \"max_in_flight\": 123,
  \"config\": {\"...\": \"...\"}
}\n\nExample response:
{
  \"topic\": \"some_topic_name\",
  \"consumer_group\": \"some_consumer_group\"
}\n")]
    Configure {
        sink_configure_in: crate::json::JsonOf<diom::models::SinkConfigureIn>,
    },
    /// Delete a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs sink delete {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"topic\": \"some_topic_name\",
  \"consumer_group\": \"some_consumer_group\"
}\n\nExample response:
{
  \"topic\": \"some_topic_name\",
  \"consumer_group\": \"some_consumer_group\",
  \"success\": true
}\n")]
    Delete {
        sink_delete_in: crate::json::JsonOf<diom::models::SinkDeleteIn>,
    },
    /// List sink configurations for a topic.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs sink list TOPIC {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"limit\": 123,
  \"iterator\": \"...\"
}\n\nExample response:
{
  \"data\": [{\"topic\": \"some_topic_name\", \"consumer_group\": \"some_consumer_group\", \"default_starting_position\": \"earliest\", \"max_in_flight\": 123, \"config\": {\"...\": \"...\"}}],
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": true
}\n")]
    List {
        topic: String,
        sink_list_in: Option<crate::json::JsonOf<diom::models::SinkListIn>>,
    },
}

impl MsgsSinkCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure { sink_configure_in } => {
                let resp = client
                    .msgs()
                    .sink()
                    .configure(sink_configure_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete { sink_delete_in } => {
                let resp = client
                    .msgs()
                    .sink()
                    .delete(sink_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::List {
                topic,
                sink_list_in,
            } => {
                let resp = client
                    .msgs()
                    .sink()
                    .list(topic, sink_list_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

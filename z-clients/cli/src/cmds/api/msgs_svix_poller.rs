// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsSvixPollerArgs {
    #[command(subcommand)]
    pub command: MsgsSvixPollerCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsSvixPollerCommands {
    /// Create a Svix poller configuration for a topic.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs svix-poller create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"topic\": \"some_topic_name\",
  \"poller_id\": \"...\",
  \"token\": \"...\"
}\n\nExample response:
{
  \"topic\": \"some_topic_name\",
  \"poller_id\": \"...\"
}\n")]
    Create {
        svix_poller_create_in: crate::json::JsonOf<diom::models::SvixPollerCreateIn>,
    },
    /// Delete a Svix poller configuration.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs svix-poller delete {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"topic\": \"some_topic_name\",
  \"poller_id\": \"...\"
}\n\nExample response:
{
  \"topic\": \"some_topic_name\",
  \"poller_id\": \"...\",
  \"success\": true
}\n")]
    Delete {
        svix_poller_delete_in: crate::json::JsonOf<diom::models::SvixPollerDeleteIn>,
    },
    /// List Svix poller configurations for a topic.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs svix-poller list TOPIC {...}\n",
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
  \"data\": [{\"topic\": \"some_topic_name\", \"poller_id\": \"...\", \"token\": \"...\"}],
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": true
}\n")]
    List {
        topic: String,
        svix_poller_list_in: Option<crate::json::JsonOf<diom::models::SvixPollerListIn>>,
    },
}

impl MsgsSvixPollerCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Create {
                svix_poller_create_in,
            } => {
                let resp = client
                    .msgs()
                    .svix_poller()
                    .create(svix_poller_create_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete {
                svix_poller_delete_in,
            } => {
                let resp = client
                    .msgs()
                    .svix_poller()
                    .delete(svix_poller_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::List {
                topic,
                svix_poller_list_in,
            } => {
                let resp = client
                    .msgs()
                    .svix_poller()
                    .list(topic, svix_poller_list_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

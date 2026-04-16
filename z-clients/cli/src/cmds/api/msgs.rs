// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

use super::{MsgsNamespaceArgs, MsgsQueueArgs, MsgsStreamArgs, MsgsTopicArgs};
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsArgs {
    #[command(subcommand)]
    pub command: MsgsCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsCommands {
    Namespace(MsgsNamespaceArgs),
    Queue(MsgsQueueArgs),
    Stream(MsgsStreamArgs),
    Topic(MsgsTopicArgs),
    /// Publishes messages to a topic within a namespace.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"msgs\": [{\"value\": \"...\", \"headers\": {\"key\": \"...\"}, \"key\": \"...\", \"delay_ms\": 60000}],
  \"idempotency_key\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"topics\": [{\"topic\": \"...\", \"start_offset\": 123, \"offset\": 123}]
}")]
    Publish {
        topic: String,
        msg_publish_in: crate::json::JsonOf<diom::models::MsgPublishIn>,
    },
}

impl MsgsCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Queue(args) => {
                args.command.exec(client).await?;
            }
            Self::Stream(args) => {
                args.command.exec(client).await?;
            }
            Self::Topic(args) => {
                args.command.exec(client).await?;
            }
            Self::Publish {
                topic,
                msg_publish_in,
            } => {
                let resp = client
                    .msgs()
                    .publish(topic, msg_publish_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

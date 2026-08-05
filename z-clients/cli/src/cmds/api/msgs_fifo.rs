// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsFifoArgs {
    #[command(subcommand)]
    pub command: MsgsFifoCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsFifoCommands {
    /// Receives messages from a topic with strict per-key ordering.
    ///
    /// Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
    /// message for a key, no other consumer receives that key's messages until it is acked (or its
    /// lease expires). A single call may return several messages of the same key, in order. Keyless
    /// messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
    /// split a key across partitions, breaking its order at that boundary.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs fifo receive TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"batch_size\": 123,
  \"lease_duration_ms\": 60000,
  \"batch_wait_ms\": 60000
}\n\nExample response:
{
  \"msgs\": [{\"msg_id\": \"...\", \"key\": \"...\", \"value\": \"...\", \"headers\": {\"key\": \"...\"}, \"timestamp\": 1234567890123, \"scheduled_at\": 1234567890123}]
}\n")]
    Receive {
        topic: String,
        consumer_group: String,
        msg_fifo_receive_in: Option<crate::json::JsonOf<diom::models::MsgFifoReceiveIn>>,
    },
    /// Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs fifo ack TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"msg_ids\": [\"...\"]
}\n\nExample response:
{
}\n")]
    Ack {
        topic: String,
        consumer_group: String,
        msg_fifo_ack_in: crate::json::JsonOf<diom::models::MsgFifoAckIn>,
    },
    /// Extends the lease on in-flight fifo messages.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs fifo extend-lease TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"msg_ids\": [\"...\"],
  \"lease_duration_ms\": 60000
}\n\nExample response:
{
}\n")]
    ExtendLease {
        topic: String,
        consumer_group: String,
        msg_fifo_extend_lease_in: crate::json::JsonOf<diom::models::MsgFifoExtendLeaseIn>,
    },
    /// Configures retry and DLQ behavior for a fifo consumer group on a topic.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs fifo configure TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"retry_schedule\": [123],
  \"dlq_topic\": \"some_topic_name\"
}\n\nExample response:
{
  \"retry_schedule\": [123],
  \"dlq_topic\": \"some_topic_name\"
}\n")]
    Configure {
        topic: String,
        consumer_group: String,
        msg_fifo_configure_in: Option<crate::json::JsonOf<diom::models::MsgFifoConfigureIn>>,
    },
    /// Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs fifo nack TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"msg_ids\": [\"...\"]
}\n\nExample response:
{
}\n")]
    Nack {
        topic: String,
        consumer_group: String,
        msg_fifo_nack_in: crate::json::JsonOf<diom::models::MsgFifoNackIn>,
    },
    /// Moves all dead-letter queue messages for a fifo consumer group back for reprocessing.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs fifo redrive-dlq TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\"
}\n\nExample response:
{
}\n")]
    RedriveDlq {
        topic: String,
        consumer_group: String,
        msg_fifo_redrive_dlq_in: Option<crate::json::JsonOf<diom::models::MsgFifoRedriveDlqIn>>,
    },
}

impl MsgsFifoCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Receive {
                topic,
                consumer_group,
                msg_fifo_receive_in,
            } => {
                let resp = client
                    .msgs()
                    .fifo()
                    .receive(
                        topic,
                        consumer_group,
                        msg_fifo_receive_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Ack {
                topic,
                consumer_group,
                msg_fifo_ack_in,
            } => {
                let resp = client
                    .msgs()
                    .fifo()
                    .ack(topic, consumer_group, msg_fifo_ack_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::ExtendLease {
                topic,
                consumer_group,
                msg_fifo_extend_lease_in,
            } => {
                let resp = client
                    .msgs()
                    .fifo()
                    .extend_lease(topic, consumer_group, msg_fifo_extend_lease_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Configure {
                topic,
                consumer_group,
                msg_fifo_configure_in,
            } => {
                let resp = client
                    .msgs()
                    .fifo()
                    .configure(
                        topic,
                        consumer_group,
                        msg_fifo_configure_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Nack {
                topic,
                consumer_group,
                msg_fifo_nack_in,
            } => {
                let resp = client
                    .msgs()
                    .fifo()
                    .nack(topic, consumer_group, msg_fifo_nack_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::RedriveDlq {
                topic,
                consumer_group,
                msg_fifo_redrive_dlq_in,
            } => {
                let resp = client
                    .msgs()
                    .fifo()
                    .redrive_dlq(
                        topic,
                        consumer_group,
                        msg_fifo_redrive_dlq_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

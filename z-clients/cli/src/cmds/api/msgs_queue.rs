// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsQueueArgs {
    #[command(subcommand)]
    pub command: MsgsQueueCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsQueueCommands {
    /// Receives messages from a topic as competing consumers.
    ///
    /// Messages are individually leased for the specified duration. Multiple consumers can receive
    /// different messages from the same topic concurrently. Leased messages are skipped until they
    /// are acked or their lease expires.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs queue receive TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"batch_size\": 123,
  \"lease_duration_ms\": 60000,
  \"batch_wait_ms\": 60000
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"msgs\": [{\"msg_id\": \"...\", \"value\": \"...\", \"headers\": {\"key\": \"...\"}, \"timestamp\": 1234567890123, \"scheduled_at\": 1234567890123}]
}\n")]
    Receive {
        topic: String,
        consumer_group: String,
        msg_queue_receive_in: Option<crate::json::JsonOf<diom::models::MsgQueueReceiveIn>>,
    },
    /// Acknowledges messages by their opaque msg_ids.
    ///
    /// Acked messages are permanently removed from the queue and will never be re-delivered.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs queue ack TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"msg_ids\": [\"...\"]
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}\n")]
    Ack {
        topic: String,
        consumer_group: String,
        msg_queue_ack_in: crate::json::JsonOf<diom::models::MsgQueueAckIn>,
    },
    /// Extends the lease on in-flight messages.
    ///
    /// Consumers that need more processing time can call this before the lease expires to prevent the
    /// message from being re-delivered to another consumer.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs queue extend-lease TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"msg_ids\": [\"...\"],
  \"lease_duration_ms\": 60000
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}\n")]
    ExtendLease {
        topic: String,
        consumer_group: String,
        msg_queue_extend_lease_in: crate::json::JsonOf<diom::models::MsgQueueExtendLeaseIn>,
    },
    /// Configures retry and DLQ behavior for a consumer group on a topic.
    ///
    /// `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
    /// the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs queue configure TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"retry_schedule\": [123],
  \"dlq_topic\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"retry_schedule\": [123],
  \"dlq_topic\": \"...\"
}\n")]
    Configure {
        topic: String,
        consumer_group: String,
        msg_queue_configure_in: Option<crate::json::JsonOf<diom::models::MsgQueueConfigureIn>>,
    },
    /// Rejects messages, sending them to the dead-letter queue.
    ///
    /// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
    /// move them back to the queue for reprocessing.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs queue nack TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"msg_ids\": [\"...\"]
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}\n")]
    Nack {
        topic: String,
        consumer_group: String,
        msg_queue_nack_in: crate::json::JsonOf<diom::models::MsgQueueNackIn>,
    },
    /// Moves all dead-letter queue messages back to the main queue for reprocessing.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs queue redrive-dlq TOPIC CONSUMER_GROUP {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}\n")]
    RedriveDlq {
        topic: String,
        consumer_group: String,
        msg_queue_redrive_dlq_in: Option<crate::json::JsonOf<diom::models::MsgQueueRedriveDlqIn>>,
    },
}

impl MsgsQueueCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Receive {
                topic,
                consumer_group,
                msg_queue_receive_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .receive(
                        topic,
                        consumer_group,
                        msg_queue_receive_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Ack {
                topic,
                consumer_group,
                msg_queue_ack_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .ack(topic, consumer_group, msg_queue_ack_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::ExtendLease {
                topic,
                consumer_group,
                msg_queue_extend_lease_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .extend_lease(
                        topic,
                        consumer_group,
                        msg_queue_extend_lease_in.into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Configure {
                topic,
                consumer_group,
                msg_queue_configure_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .configure(
                        topic,
                        consumer_group,
                        msg_queue_configure_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Nack {
                topic,
                consumer_group,
                msg_queue_nack_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .nack(topic, consumer_group, msg_queue_nack_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::RedriveDlq {
                topic,
                consumer_group,
                msg_queue_redrive_dlq_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .redrive_dlq(
                        topic,
                        consumer_group,
                        msg_queue_redrive_dlq_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}

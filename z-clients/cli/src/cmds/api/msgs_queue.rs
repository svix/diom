// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsQueueArgs {
    #[command(subcommand)]
    pub command: MsgsQueueCommands,
}

#[derive(Subcommand)]
pub enum MsgsQueueCommands {
    /// Receives messages from a topic as competing consumers.
    ///
    /// Messages are individually leased for the specified duration. Multiple consumers can receive
    /// different messages from the same topic concurrently. Leased messages are skipped until they
    /// are acked or their lease expires.
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  namespace (optional)
  batch_size (optional)
  lease_duration_ms (optional)
  batch_wait_ms (optional) — Maximum time (in milliseconds) to wait for messages before returning.")]
    Receive {
        topic: String,
        consumer_group: String,
        msg_queue_receive_in: Option<crate::json::JsonOf<coyote_client::models::MsgQueueReceiveIn>>,
    },
    /// Acknowledges messages by their opaque msg_ids.
    ///
    /// Acked messages are permanently removed from the queue and will never be re-delivered.
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  msg_ids
  namespace (optional)")]
    Ack {
        topic: String,
        consumer_group: String,
        msg_queue_ack_in: crate::json::JsonOf<coyote_client::models::MsgQueueAckIn>,
    },
    /// Configures retry and DLQ behavior for a consumer group on a topic.
    ///
    /// `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
    /// the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  namespace (optional)
  retry_schedule (optional)
  dlq_topic (optional)")]
    Configure {
        topic: String,
        consumer_group: String,
        msg_queue_configure_in:
            Option<crate::json::JsonOf<coyote_client::models::MsgQueueConfigureIn>>,
    },
    /// Rejects messages, sending them to the dead-letter queue.
    ///
    /// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
    /// move them back to the queue for reprocessing.
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  msg_ids
  namespace (optional)")]
    Nack {
        topic: String,
        consumer_group: String,
        msg_queue_nack_in: crate::json::JsonOf<coyote_client::models::MsgQueueNackIn>,
    },
    /// Moves all dead-letter queue messages back to the main queue for reprocessing.
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  namespace (optional)")]
    RedriveDlq {
        topic: String,
        consumer_group: String,
        msg_queue_redrive_dlq_in:
            Option<crate::json::JsonOf<coyote_client::models::MsgQueueRedriveDlqIn>>,
    },
}

impl MsgsQueueCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
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

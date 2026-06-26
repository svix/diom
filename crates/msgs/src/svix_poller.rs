use std::{
    collections::HashMap,
    marker::PhantomData,
    num::NonZeroUsize,
    time::{Duration, Instant},
};

use diom_core::{
    svix_client::{SvixAutoConfigClient, SvixClientError},
    tokio_nursery::TaskNursery,
    types::ByteString,
};
use diom_operations::{BackgroundResult, OperationWriter};
use fjall_utils::{FjallKey, TableRow};
use svix::api::PollerV2MessageOut;
use tracing::instrument;

use crate::{
    State,
    entities::{MsgIn, TopicIn, TopicName},
    operations::{MsgsOperation, PublishOperation},
    storage::{SvixPollerKey, SvixPollerRow},
};

const DEFAULT_MAX_MESSAGES_PER_POLL: usize = 1000;

/// We deliberately use the longest lease duration we can get away with when querying the Svix
/// Autoconfig poller. If the time between `poll_from_config(...)` and `client.commit(...)` exceeds
/// the lease duration, then `client.commit(...)` will fail, and we end up repolling the same
/// messages, which risks Publishing duplicates to the topic. No Bueno.
const SVIX_LEASE_DURATION: Duration = Duration::from_mins(5);

/// Runtime tuning for the Svix poller background worker.
#[derive(Clone, Copy)]
pub struct SvixPollerConfig {
    pub max_concurrent_pollers: NonZeroUsize,
    pub max_task_duration: Duration,
}

/// Polls a single Svix Ingest endpoint via the AutoConfigConsumer and returns
/// up to `max_messages` messages along with the last offset seen.
///
/// Does NOT commit — the caller must commit after successfully persisting the
/// messages to avoid data loss on publish failure.
#[instrument(skip_all, fields(namespace=?key.namespace_id, topic = ?key.topic_id, poller_id = %key.poller_id, count, success = false))]
pub(crate) async fn poll_from_config(
    client: &impl SvixAutoConfigClient,
    key: &SvixPollerKey,
    max_messages: usize,
    lease_duration: Duration,
) -> Result<(Vec<MsgIn>, Option<i32>), SvixClientError> {
    tracing::trace!("Polling from AutoConfig");

    let consumer_id = key.consumer_id();

    let limit = i32::try_from(max_messages).unwrap_or(i32::MAX);

    let lease_duration_ms: i32 = lease_duration.as_millis().try_into().unwrap_or(i32::MAX);

    let response = client
        .receive(&consumer_id, Some(limit), Some(lease_duration_ms))
        .await?;

    let span = tracing::Span::current();
    span.record("count", response.data.len());
    span.record("success", true);

    let last_offset = response.data.last().map(|msg| msg.offset);

    let msgs = response
        .data
        .iter()
        .map(pollerv2_msg_to_msg_in)
        .collect::<Result<Vec<_>, SvixClientError>>()?;

    Ok((msgs, last_offset))
}

fn pollerv2_msg_to_msg_in(msg: &PollerV2MessageOut) -> Result<MsgIn, SvixClientError> {
    let value =
        serde_json::to_vec(msg).map_err(|e| SvixClientError::new(format!("serialize: {e}")))?;
    Ok(MsgIn {
        value: ByteString::from(value),
        headers: HashMap::new(),
        key: None,
        delay: None,
    })
}

#[derive(Clone)]
pub struct LeaderWorker<C: SvixAutoConfigClient, F: OperationWriter<MsgsOperation>> {
    state: State,
    poll_interval: Duration,
    handle: F,
    config: SvixPollerConfig,
    _client: PhantomData<C>,
}

impl<C, F> LeaderWorker<C, F>
where
    C: SvixAutoConfigClient,
    F: OperationWriter<MsgsOperation> + Send + Sync + 'static,
{
    pub fn new(state: State, poll_interval: Duration, handle: F, config: SvixPollerConfig) -> Self {
        Self {
            state,
            poll_interval,
            handle,
            config,
            _client: PhantomData,
        }
    }

    /// Iterates over every poller config and drains each one concurrently, bounded by
    /// [`SvixPollerConfig::max_concurrent_pollers`]. `spawn`'s permit-based backpressure paces
    /// pagination so at most `max_concurrent_pollers` pollers (and their in-flight messages) are
    /// resident at once.
    async fn poll_cycle(&self) -> BackgroundResult<()> {
        let batch_size = self.config.max_concurrent_pollers.get();
        let mut nursery = TaskNursery::new(self.config.max_concurrent_pollers);

        let mut iterator: Option<Vec<u8>> = None;
        let prefix = &[<SvixPollerRow as TableRow>::ROW_TYPE];

        loop {
            let batch = SvixPollerRow::list_range(
                &self.state.metadata_tables,
                prefix,
                iterator,
                batch_size,
            )
            .map_err(diom_operations::BackgroundError::Other)?;

            if batch.is_empty() {
                break;
            }

            iterator = batch.last().map(|(k, _)| k.to_vec());
            let batch_len = batch.len();

            for (key_bytes, row) in batch {
                let Ok(key) = SvixPollerKey::from_fjall_key(key_bytes) else {
                    continue;
                };

                let handle = self.handle.clone();
                let deadline = self.config.max_task_duration;
                nursery
                    .spawn(drain_poller::<C, F>(
                        handle, key, row.topic, row.token, deadline,
                    ))
                    .await;
            }

            if batch_len < batch_size {
                break;
            }
        }

        nursery.join_all().await;

        Ok(())
    }
}

/// Subscribes once, then repeatedly polls, publishes, and commits until the endpoint is drained
/// (an empty poll) or `max_duration` elapses. Errors are logged and end the task — one poller's
/// failure never aborts the cycle or its siblings.
#[instrument(skip_all, fields(namespace=?key.namespace_id, topic = ?key.topic_id, poller_id = %key.poller_id))]
async fn drain_poller<C, F>(
    handle: F,
    key: SvixPollerKey,
    topic_name: TopicName,
    token: String,
    max_duration: Duration,
) where
    C: SvixAutoConfigClient,
    F: OperationWriter<MsgsOperation> + Send + Sync,
{
    let client = match C::new(token) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(error = %e, "failed to create svix client from token, skipping");
            return;
        }
    };

    if let Err(e) = client.subscribe().await {
        tracing::warn!(error = %e, "failed to subscribe svix poller, skipping");
        return;
    }

    let consumer_id = key.consumer_id();
    let deadline = Instant::now() + max_duration;

    while Instant::now() < deadline {
        let (msgs, offset) = match poll_from_config(
            &client,
            &key,
            DEFAULT_MAX_MESSAGES_PER_POLL,
            SVIX_LEASE_DURATION,
        )
        .await
        {
            // An empty poll means the poller is drained; stop looping.
            Ok((_, None)) => break,
            Ok((msgs, Some(offset))) => (msgs, offset),
            Err(e) => {
                tracing::warn!(error = %e, "failed to poll svix endpoint");
                break;
            }
        };

        let op = match PublishOperation::new(
            key.namespace_id,
            TopicIn::TopicName(topic_name.clone()),
            msgs,
            None,
        ) {
            Ok(op) => op,
            Err(e) => {
                tracing::warn!(error = %e, "failed to build publish operation, skipping");
                break;
            }
        };

        // Only commit after a successful publish, so a publish failure re-leases the messages for
        // the next cycle rather than dropping them.
        if let Err(e) = handle.write_request(op).await {
            tracing::warn!(error = %e, "failed to publish polled messages");
            break;
        }

        if let Err(e) = client.commit(&consumer_id, offset).await {
            tracing::warn!(error = %e, "failed to commit offset after publish");
            break;
        }
    }
}

impl<C: SvixAutoConfigClient, F: OperationWriter<MsgsOperation>>
    diom_operations::workers::BackgroundWorker for LeaderWorker<C, F>
{
    const NAME: &'static str = "leader-worker:svix-poller";

    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(self.poll_interval);
        let shutting_down = diom_core::shutdown::shutting_down_token();

        while shutting_down
            .run_until_cancelled(timer.tick())
            .await
            .is_some()
        {
            self.poll_cycle().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::disallowed_types)]

    use std::{
        num::{NonZeroU16, NonZeroUsize},
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
    };

    use diom_core::{
        svix_client::{SvixAutoConfigClient, SvixClientError},
        types::DurationMs,
    };
    use diom_id::NamespaceId;
    use diom_operations::OpContext;
    use svix::api::{PollerV2MessageOut, PollerV2PollOut};

    use crate::{
        entities::{ConsumerGroup, SeekPosition, TopicIn, TopicName},
        operations::{
            MsgsOperation, MsgsRaftState, Response, StreamReceiveOperation,
            SvixPollerCreateOperation,
        },
        test_fixture::{Fixture, ts},
    };

    use super::*;

    /// Mock endpoint that returns its messages on the first poll and then reports empty, so the
    /// draining loop terminates after one round (rather than looping until the time budget).
    #[derive(Clone)]
    struct MockSvixClient {
        messages: Vec<PollerV2MessageOut>,
        drained: Arc<AtomicBool>,
    }

    impl MockSvixClient {
        fn with_messages(messages: Vec<PollerV2MessageOut>) -> Self {
            Self {
                messages,
                drained: Arc::new(AtomicBool::new(false)),
            }
        }
    }

    impl SvixAutoConfigClient for MockSvixClient {
        fn new(_token: String) -> Result<Self, SvixClientError> {
            Ok(Self::with_messages(vec![
                make_pollerv2_msg("invoice.paid", serde_json::json!({"amount": 100}), 0),
                make_pollerv2_msg("invoice.created", serde_json::json!({"amount": 200}), 1),
            ]))
        }

        async fn subscribe(&self) -> Result<(), SvixClientError> {
            Ok(())
        }

        async fn receive(
            &self,
            _consumer_id: &str,
            limit: Option<i32>,
            _lease_duration_ms: Option<i32>,
        ) -> Result<PollerV2PollOut, SvixClientError> {
            // Once drained, subsequent polls are empty, ending the drain loop.
            if self.drained.swap(true, Ordering::SeqCst) {
                return Ok(PollerV2PollOut {
                    data: vec![],
                    done: true,
                });
            }
            let limit = limit.unwrap_or(i32::MAX) as usize;
            let data: Vec<_> = self.messages.iter().take(limit).cloned().collect();
            Ok(PollerV2PollOut { data, done: true })
        }

        async fn commit(&self, _consumer_id: &str, _offset: i32) -> Result<(), SvixClientError> {
            Ok(())
        }
    }

    #[derive(Clone)]
    struct DirectWriter {
        state: State,
        namespace_state: diom_namespace::State,
    }

    impl diom_operations::OperationWriterBase for DirectWriter {
        type Request = MsgsOperation;
        type Response = Response;

        async fn do_write_request(
            &self,
            request: Self::Request,
        ) -> BackgroundResult<Self::Response> {
            let raft_state = MsgsRaftState {
                msgs: &self.state,
                namespace: &self.namespace_state,
            };
            let ctx = OpContext {
                timestamp: ts(2000),
                log_index: 1,
                term: 1,
            };
            Ok(request.apply(raft_state, &ctx).await)
        }
    }

    fn make_pollerv2_msg(
        event_type: &str,
        payload: serde_json::Value,
        offset: i32,
    ) -> PollerV2MessageOut {
        PollerV2MessageOut::new(
            event_type.to_owned(),
            "msg_test123".to_owned(),
            offset,
            payload,
            "2024-01-01T00:00:00Z".to_owned(),
        )
    }

    #[tokio::test]
    async fn test_poll_from_config_converts_messages() {
        let expected_msgs = vec![
            make_pollerv2_msg("user.created", serde_json::json!({"user_id": "123"}), 0),
            make_pollerv2_msg("user.updated", serde_json::json!({"user_id": "456"}), 1),
        ];
        let client = MockSvixClient::with_messages(expected_msgs.clone());

        let key = SvixPollerKey {
            namespace_id: NamespaceId::nil(),
            topic_id: diom_id::TopicId::nil(),
            poller_id: "poller_test".into(),
        };

        let (result, last_offset) = poll_from_config(&client, &key, 100, Duration::from_mins(5))
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(last_offset, Some(1));

        let parsed: PollerV2MessageOut = serde_json::from_slice(&result[0].value).unwrap();
        assert_eq!(parsed, expected_msgs[0]);

        let parsed: PollerV2MessageOut = serde_json::from_slice(&result[1].value).unwrap();
        assert_eq!(parsed, expected_msgs[1]);
    }

    #[tokio::test]
    async fn test_poll_from_config_respects_max_messages() {
        let client = MockSvixClient::with_messages(vec![
            make_pollerv2_msg("a", serde_json::json!({}), 0),
            make_pollerv2_msg("b", serde_json::json!({}), 1),
            make_pollerv2_msg("c", serde_json::json!({}), 2),
        ]);

        let key = SvixPollerKey {
            namespace_id: NamespaceId::nil(),
            topic_id: diom_id::TopicId::nil(),
            poller_id: "poller_test".into(),
        };

        let (result, last_offset) = poll_from_config(&client, &key, 2, Duration::from_mins(5))
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(last_offset, Some(1));
    }

    #[tokio::test]
    async fn test_e2e_poll_cycle_publishes_messages() {
        let fixture = Fixture::new();
        let ns_id = fixture.create_namespace("test-ns", None, ts(1000)).await;
        let _topic_id = fixture.create_topic(ns_id, "webhooks", 1, ts(1000));

        let create_op = SvixPollerCreateOperation::new(
            ns_id,
            TopicName::new("webhooks".to_owned()).unwrap(),
            "poller_abc".into(),
            "tok_secret".into(),
        );
        let raft_state = MsgsRaftState {
            msgs: &fixture.state,
            namespace: &fixture.namespace_state,
        };
        let ctx = OpContext {
            timestamp: ts(1000),
            log_index: 1,
            term: 1,
        };
        let op: MsgsOperation = create_op.into();
        let _ = op.apply(raft_state, &ctx).await;

        let expected_msgs = [
            make_pollerv2_msg("invoice.paid", serde_json::json!({"amount": 100}), 0),
            make_pollerv2_msg("invoice.created", serde_json::json!({"amount": 200}), 1),
        ];

        let writer = DirectWriter {
            state: fixture.state.clone(),
            namespace_state: fixture.namespace_state.clone(),
        };
        let config = SvixPollerConfig {
            max_concurrent_pollers: NonZeroUsize::new(8).unwrap(),
            max_task_duration: Duration::from_secs(5),
        };
        let worker: LeaderWorker<MockSvixClient, _> = LeaderWorker::new(
            fixture.state.clone(),
            Duration::from_secs(5),
            writer,
            config,
        );
        worker.poll_cycle().await.unwrap();

        let receive_op = StreamReceiveOperation::new(
            ns_id,
            TopicIn::TopicName(TopicName::new("webhooks".to_owned()).unwrap()),
            ConsumerGroup::try_from("test-group").unwrap(),
            NonZeroU16::new(10).unwrap(),
            DurationMs::from_secs(30),
            SeekPosition::Earliest,
            None,
        )
        .unwrap();
        let raft_state = MsgsRaftState {
            msgs: &fixture.state,
            namespace: &fixture.namespace_state,
        };
        let ctx = OpContext {
            timestamp: ts(3000),
            log_index: 2,
            term: 1,
        };
        let op: MsgsOperation = receive_op.into();
        let response = op.apply(raft_state, &ctx).await;
        let msgs = match response {
            Response::StreamReceive(r) => r.0.unwrap().msgs,
            _ => panic!("unexpected response variant"),
        };

        assert_eq!(msgs.len(), 2);

        let parsed: PollerV2MessageOut = serde_json::from_slice(&msgs[0].value).unwrap();
        assert_eq!(parsed, expected_msgs[0]);

        let parsed: PollerV2MessageOut = serde_json::from_slice(&msgs[1].value).unwrap();
        assert_eq!(parsed, expected_msgs[1]);
    }
}

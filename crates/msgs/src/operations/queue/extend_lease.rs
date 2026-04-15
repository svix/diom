use diom_core::{
    PersistableValue,
    task::spawn_blocking_in_current_span,
    types::{DurationMs, UnixTimestampMs},
};
use diom_error::{Error, Result};
use diom_id::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, TopicName},
    tables::{QueueLeaseKey, QueueLeaseRow, TopicKey, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueExtendLeaseResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct QueueExtendLeaseOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
    msg_ids: Vec<MsgId>,
    lease_duration_ms: DurationMs,
}

impl QueueExtendLeaseOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        consumer_group: ConsumerGroup,
        msg_ids: Vec<MsgId>,
        lease_duration_ms: DurationMs,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
            msg_ids,
            lease_duration_ms,
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn apply_real(
        self,
        state: &State,
        now: UnixTimestampMs,
    ) -> Result<QueueExtendLeaseResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let extend_count = self.msg_ids.len() as u64;

            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic),
            )?
            .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

            let new_expiry = now
                .checked_add(self.lease_duration_ms)
                .expect("lease expiry overflow");

            let mut batch = state.db.batch();

            for msg_id in &self.msg_ids {
                let lease = QueueLeaseRow::fetch(
                    &state.metadata_tables,
                    QueueLeaseKey::build_key(
                        &topic_row.id,
                        &msg_id.partition,
                        &msg_id.offset,
                        &self.consumer_group,
                    ),
                )?;

                let lease = match lease {
                    Some(l) if l.is_acked() => {
                        return Err(Error::invalid_user_input("message is already acked"));
                    }
                    Some(l) if l.is_dlq() => {
                        return Err(Error::invalid_user_input(
                            "message is in the dead-letter queue",
                        ));
                    }
                    Some(l) if l.is_available(now) => {
                        return Err(Error::invalid_user_input("lease has expired"));
                    }
                    Some(l) => l,
                    None => {
                        return Err(Error::invalid_user_input("message has no active lease"));
                    }
                };

                batch.insert_row(
                    &state.metadata_tables,
                    QueueLeaseKey::build_key(
                        &topic_row.id,
                        &msg_id.partition,
                        &msg_id.offset,
                        &self.consumer_group,
                    ),
                    &QueueLeaseRow {
                        expiry: new_expiry,
                        dlq: false,
                        attempt_count: lease.attempt_count,
                    },
                )?;
            }

            batch.commit().map_err(Error::from)?;

            state.metrics.record_queue_lease_extended(
                &self.topic,
                &self.consumer_group,
                extend_count,
            );
            Ok(QueueExtendLeaseResponseData {})
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueExtendLeaseResponseData {}

impl MsgsRequest for QueueExtendLeaseOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> QueueExtendLeaseResponse {
        QueueExtendLeaseResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}

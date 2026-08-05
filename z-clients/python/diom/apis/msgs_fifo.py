# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgFifoAckIn,
    MsgFifoAckOut,
    MsgFifoConfigureIn,
    MsgFifoConfigureOut,
    MsgFifoExtendLeaseIn,
    MsgFifoExtendLeaseOut,
    MsgFifoNackIn,
    MsgFifoNackOut,
    MsgFifoReceiveIn,
    MsgFifoReceiveOut,
    MsgFifoRedriveDlqIn,
    MsgFifoRedriveDlqOut,
)

from ..models.msg_fifo_receive_in import _MsgFifoReceiveIn
from ..models.msg_fifo_ack_in import _MsgFifoAckIn
from ..models.msg_fifo_extend_lease_in import _MsgFifoExtendLeaseIn
from ..models.msg_fifo_configure_in import _MsgFifoConfigureIn
from ..models.msg_fifo_nack_in import _MsgFifoNackIn
from ..models.msg_fifo_redrive_dlq_in import _MsgFifoRedriveDlqIn


class MsgsFifoAsync(ApiBase):
    async def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_receive_in: MsgFifoReceiveIn = MsgFifoReceiveIn(),
    ) -> MsgFifoReceiveOut:
        """Receives messages from a topic with strict per-key ordering.

        Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
        message for a key, no other consumer receives that key's messages until it is acked (or its
        lease expires). A single call may return several messages of the same key, in order. Keyless
        messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
        split a key across partitions, breaking its order at that boundary."""
        body = _MsgFifoReceiveIn(
            namespace=msg_fifo_receive_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_fifo_receive_in.batch_size,
            lease_duration=msg_fifo_receive_in.lease_duration,
            batch_wait=msg_fifo_receive_in.batch_wait,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.fifo.receive",
            body=body,
            response_type=MsgFifoReceiveOut,
        )

    async def ack(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_ack_in: MsgFifoAckIn,
    ) -> MsgFifoAckOut:
        """Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message."""
        body = _MsgFifoAckIn(
            namespace=msg_fifo_ack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_fifo_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.fifo.ack",
            body=body,
            response_type=MsgFifoAckOut,
        )

    async def extend_lease(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_extend_lease_in: MsgFifoExtendLeaseIn,
    ) -> MsgFifoExtendLeaseOut:
        """Extends the lease on in-flight fifo messages."""
        body = _MsgFifoExtendLeaseIn(
            namespace=msg_fifo_extend_lease_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_fifo_extend_lease_in.msg_ids,
            lease_duration=msg_fifo_extend_lease_in.lease_duration,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.fifo.extend-lease",
            body=body,
            response_type=MsgFifoExtendLeaseOut,
        )

    async def configure(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_configure_in: MsgFifoConfigureIn = MsgFifoConfigureIn(),
    ) -> MsgFifoConfigureOut:
        """Configures retry and DLQ behavior for a fifo consumer group on a topic."""
        body = _MsgFifoConfigureIn(
            namespace=msg_fifo_configure_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            retry_schedule=msg_fifo_configure_in.retry_schedule,
            dlq_topic=msg_fifo_configure_in.dlq_topic,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.fifo.configure",
            body=body,
            response_type=MsgFifoConfigureOut,
        )

    async def nack(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_nack_in: MsgFifoNackIn,
    ) -> MsgFifoNackOut:
        """Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ."""
        body = _MsgFifoNackIn(
            namespace=msg_fifo_nack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_fifo_nack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.fifo.nack",
            body=body,
            response_type=MsgFifoNackOut,
        )

    async def redrive_dlq(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_redrive_dlq_in: MsgFifoRedriveDlqIn = MsgFifoRedriveDlqIn(),
    ) -> MsgFifoRedriveDlqOut:
        """Moves all dead-letter queue messages for a fifo consumer group back for reprocessing."""
        body = _MsgFifoRedriveDlqIn(
            namespace=msg_fifo_redrive_dlq_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.fifo.redrive-dlq",
            body=body,
            response_type=MsgFifoRedriveDlqOut,
        )


class MsgsFifo(ApiBase):
    def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_receive_in: MsgFifoReceiveIn = MsgFifoReceiveIn(),
    ) -> MsgFifoReceiveOut:
        """Receives messages from a topic with strict per-key ordering.

        Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
        message for a key, no other consumer receives that key's messages until it is acked (or its
        lease expires). A single call may return several messages of the same key, in order. Keyless
        messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
        split a key across partitions, breaking its order at that boundary."""
        body = _MsgFifoReceiveIn(
            namespace=msg_fifo_receive_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_fifo_receive_in.batch_size,
            lease_duration=msg_fifo_receive_in.lease_duration,
            batch_wait=msg_fifo_receive_in.batch_wait,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.fifo.receive",
            body=body,
            response_type=MsgFifoReceiveOut,
        )

    def ack(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_ack_in: MsgFifoAckIn,
    ) -> MsgFifoAckOut:
        """Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message."""
        body = _MsgFifoAckIn(
            namespace=msg_fifo_ack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_fifo_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.fifo.ack",
            body=body,
            response_type=MsgFifoAckOut,
        )

    def extend_lease(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_extend_lease_in: MsgFifoExtendLeaseIn,
    ) -> MsgFifoExtendLeaseOut:
        """Extends the lease on in-flight fifo messages."""
        body = _MsgFifoExtendLeaseIn(
            namespace=msg_fifo_extend_lease_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_fifo_extend_lease_in.msg_ids,
            lease_duration=msg_fifo_extend_lease_in.lease_duration,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.fifo.extend-lease",
            body=body,
            response_type=MsgFifoExtendLeaseOut,
        )

    def configure(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_configure_in: MsgFifoConfigureIn = MsgFifoConfigureIn(),
    ) -> MsgFifoConfigureOut:
        """Configures retry and DLQ behavior for a fifo consumer group on a topic."""
        body = _MsgFifoConfigureIn(
            namespace=msg_fifo_configure_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            retry_schedule=msg_fifo_configure_in.retry_schedule,
            dlq_topic=msg_fifo_configure_in.dlq_topic,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.fifo.configure",
            body=body,
            response_type=MsgFifoConfigureOut,
        )

    def nack(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_nack_in: MsgFifoNackIn,
    ) -> MsgFifoNackOut:
        """Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ."""
        body = _MsgFifoNackIn(
            namespace=msg_fifo_nack_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_fifo_nack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.fifo.nack",
            body=body,
            response_type=MsgFifoNackOut,
        )

    def redrive_dlq(
        self,
        topic: str,
        consumer_group: str,
        msg_fifo_redrive_dlq_in: MsgFifoRedriveDlqIn = MsgFifoRedriveDlqIn(),
    ) -> MsgFifoRedriveDlqOut:
        """Moves all dead-letter queue messages for a fifo consumer group back for reprocessing."""
        body = _MsgFifoRedriveDlqIn(
            namespace=msg_fifo_redrive_dlq_in.namespace,
            topic=topic,
            consumer_group=consumer_group,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.fifo.redrive-dlq",
            body=body,
            response_type=MsgFifoRedriveDlqOut,
        )

# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgQueueAckIn,
    MsgQueueAckOut,
    MsgQueueNackIn,
    MsgQueueNackOut,
    MsgQueueReceiveIn,
    MsgQueueReceiveOut,
    MsgQueueRedriveDlqIn,
    MsgQueueRedriveDlqOut,
)

from ..models.msg_queue_receive_in import _MsgQueueReceiveIn
from ..models.msg_queue_ack_in import _MsgQueueAckIn
from ..models.msg_queue_nack_in import _MsgQueueNackIn
from ..models.msg_queue_redrive_dlq_in import _MsgQueueRedriveDlqIn


class MsgsQueueAsync(ApiBase):
    async def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_receive_in: MsgQueueReceiveIn = MsgQueueReceiveIn(),
    ) -> MsgQueueReceiveOut:
        """Receives messages from a topic as competing consumers.

        Messages are individually leased for the specified duration. Multiple consumers can receive
        different messages from the same topic concurrently. Leased messages are skipped until they
        are acked or their lease expires."""
        body = _MsgQueueReceiveIn(
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_queue_receive_in.batch_size,
            lease_duration_millis=msg_queue_receive_in.lease_duration_millis,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/queue/receive",
            body=body,
            response_type=MsgQueueReceiveOut,
        )

    async def ack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> MsgQueueAckOut:
        """Acknowledges messages by their opaque msg_ids.

        Acked messages are permanently removed from the queue and will never be re-delivered."""
        body = _MsgQueueAckIn(
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/queue/ack",
            body=body,
            response_type=MsgQueueAckOut,
        )

    async def nack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_nack_in: MsgQueueNackIn,
    ) -> MsgQueueNackOut:
        """Rejects messages, sending them to the dead-letter queue.

        Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
        move them back to the queue for reprocessing."""
        body = _MsgQueueNackIn(
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_nack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/queue/nack",
            body=body,
            response_type=MsgQueueNackOut,
        )

    async def redrive_dlq(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_redrive_dlq_in: MsgQueueRedriveDlqIn = MsgQueueRedriveDlqIn(),
    ) -> MsgQueueRedriveDlqOut:
        """Moves all dead-letter queue messages back to the main queue for reprocessing."""
        body = _MsgQueueRedriveDlqIn(
            topic=topic,
            consumer_group=consumer_group,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/queue/redrive-dlq",
            body=body,
            response_type=MsgQueueRedriveDlqOut,
        )


class MsgsQueue(ApiBase):
    def receive(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_receive_in: MsgQueueReceiveIn = MsgQueueReceiveIn(),
    ) -> MsgQueueReceiveOut:
        """Receives messages from a topic as competing consumers.

        Messages are individually leased for the specified duration. Multiple consumers can receive
        different messages from the same topic concurrently. Leased messages are skipped until they
        are acked or their lease expires."""
        body = _MsgQueueReceiveIn(
            topic=topic,
            consumer_group=consumer_group,
            batch_size=msg_queue_receive_in.batch_size,
            lease_duration_millis=msg_queue_receive_in.lease_duration_millis,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/queue/receive",
            body=body,
            response_type=MsgQueueReceiveOut,
        )

    def ack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> MsgQueueAckOut:
        """Acknowledges messages by their opaque msg_ids.

        Acked messages are permanently removed from the queue and will never be re-delivered."""
        body = _MsgQueueAckIn(
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/queue/ack",
            body=body,
            response_type=MsgQueueAckOut,
        )

    def nack(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_nack_in: MsgQueueNackIn,
    ) -> MsgQueueNackOut:
        """Rejects messages, sending them to the dead-letter queue.

        Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
        move them back to the queue for reprocessing."""
        body = _MsgQueueNackIn(
            topic=topic,
            consumer_group=consumer_group,
            msg_ids=msg_queue_nack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/queue/nack",
            body=body,
            response_type=MsgQueueNackOut,
        )

    def redrive_dlq(
        self,
        topic: str,
        consumer_group: str,
        msg_queue_redrive_dlq_in: MsgQueueRedriveDlqIn = MsgQueueRedriveDlqIn(),
    ) -> MsgQueueRedriveDlqOut:
        """Moves all dead-letter queue messages back to the main queue for reprocessing."""
        body = _MsgQueueRedriveDlqIn(
            topic=topic,
            consumer_group=consumer_group,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/queue/redrive-dlq",
            body=body,
            response_type=MsgQueueRedriveDlqOut,
        )

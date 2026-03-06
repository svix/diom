# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgQueueAckIn,
    MsgQueueAckOut,
    MsgQueueReceiveIn,
    MsgQueueReceiveOut,
)

from ..models.msg_queue_receive_in import _MsgQueueReceiveIn
from ..models.msg_queue_ack_in import _MsgQueueAckIn


class MsgsQueueAsync(ApiBase):
    async def receive(
        self,
        topic: str,
        msg_queue_receive_in: MsgQueueReceiveIn = MsgQueueReceiveIn(),
    ) -> MsgQueueReceiveOut:
        """Receives messages from a topic as competing consumers.

        Messages are individually leased for the specified duration. Multiple consumers can receive
        different messages from the same topic concurrently. Leased messages are skipped until they
        are acked or their lease expires."""
        body = _MsgQueueReceiveIn(
            topic=topic,
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
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> MsgQueueAckOut:
        """Acknowledges messages by their opaque msg_ids.

        Acked messages are permanently removed from the queue and will never be re-delivered."""
        body = _MsgQueueAckIn(
            topic=topic,
            msg_ids=msg_queue_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/queue/ack",
            body=body,
            response_type=MsgQueueAckOut,
        )


class MsgsQueue(ApiBase):
    def receive(
        self,
        topic: str,
        msg_queue_receive_in: MsgQueueReceiveIn = MsgQueueReceiveIn(),
    ) -> MsgQueueReceiveOut:
        """Receives messages from a topic as competing consumers.

        Messages are individually leased for the specified duration. Multiple consumers can receive
        different messages from the same topic concurrently. Leased messages are skipped until they
        are acked or their lease expires."""
        body = _MsgQueueReceiveIn(
            topic=topic,
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
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> MsgQueueAckOut:
        """Acknowledges messages by their opaque msg_ids.

        Acked messages are permanently removed from the queue and will never be re-delivered."""
        body = _MsgQueueAckIn(
            topic=topic,
            msg_ids=msg_queue_ack_in.msg_ids,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/queue/ack",
            body=body,
            response_type=MsgQueueAckOut,
        )

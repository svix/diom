# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    StreamReceiveIn,
    StreamReceiveOut,
)


class MsgsStreamAsync(ApiBase):
    async def receive(
        self,
        stream_receive_in: StreamReceiveIn,
    ) -> StreamReceiveOut:
        """Receives messages from a topic using a consumer group.

        Each consumer in the group reads from all partitions. Messages are locked by leases for the
        specified duration to prevent duplicate delivery within the same consumer group."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/stream/receive",
            body=stream_receive_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=StreamReceiveOut,
        )


class MsgsStream(ApiBase):
    def receive(
        self,
        stream_receive_in: StreamReceiveIn,
    ) -> StreamReceiveOut:
        """Receives messages from a topic using a consumer group.

        Each consumer in the group reads from all partitions. Messages are locked by leases for the
        specified duration to prevent duplicate delivery within the same consumer group."""
        return self._request_sync(
            method="post",
            path="/api/v1/msgs/stream/receive",
            body=stream_receive_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=StreamReceiveOut,
        )

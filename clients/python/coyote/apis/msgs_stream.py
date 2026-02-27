# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    StreamCommitIn,
    StreamCommitOut,
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

    async def commit(
        self,
        stream_commit_in: StreamCommitIn,
    ) -> StreamCommitOut:
        """Commits an offset for a consumer group on a specific partition.

        The topic must be a partition-level topic (e.g. `my-topic~3`). The offset is the last
        successfully processed offset; future receives will start after it."""
        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/stream/commit",
            body=stream_commit_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=StreamCommitOut,
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

    def commit(
        self,
        stream_commit_in: StreamCommitIn,
    ) -> StreamCommitOut:
        """Commits an offset for a consumer group on a specific partition.

        The topic must be a partition-level topic (e.g. `my-topic~3`). The offset is the last
        successfully processed offset; future receives will start after it."""
        return self._request_sync(
            method="post",
            path="/api/v1/msgs/stream/commit",
            body=stream_commit_in.model_dump(exclude_unset=True, by_alias=True),
            response_type=StreamCommitOut,
        )

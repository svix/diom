# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgStreamCommitIn,
    MsgStreamCommitOut,
    MsgStreamReceiveIn,
    MsgStreamReceiveOut,
)


class MsgsStreamAsync(ApiBase):
    async def receive(
        self,
        msg_stream_receive_in: MsgStreamReceiveIn,
    ) -> MsgStreamReceiveOut:
        """Receives messages from a topic using a consumer group.

        Each consumer in the group reads from all partitions. Messages are locked by leases for the
        specified duration to prevent duplicate delivery within the same consumer group."""
        body = msg_stream_receive_in.model_dump(exclude_unset=True, by_alias=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/stream/receive",
            body=body,
            response_type=MsgStreamReceiveOut,
        )

    async def commit(
        self,
        msg_stream_commit_in: MsgStreamCommitIn,
    ) -> MsgStreamCommitOut:
        """Commits an offset for a consumer group on a specific partition.

        The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
        successfully processed offset; future receives will start after it."""
        body = msg_stream_commit_in.model_dump(exclude_unset=True, by_alias=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/stream/commit",
            body=body,
            response_type=MsgStreamCommitOut,
        )


class MsgsStream(ApiBase):
    def receive(
        self,
        msg_stream_receive_in: MsgStreamReceiveIn,
    ) -> MsgStreamReceiveOut:
        """Receives messages from a topic using a consumer group.

        Each consumer in the group reads from all partitions. Messages are locked by leases for the
        specified duration to prevent duplicate delivery within the same consumer group."""
        body = msg_stream_receive_in.model_dump(exclude_unset=True, by_alias=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/stream/receive",
            body=body,
            response_type=MsgStreamReceiveOut,
        )

    def commit(
        self,
        msg_stream_commit_in: MsgStreamCommitIn,
    ) -> MsgStreamCommitOut:
        """Commits an offset for a consumer group on a specific partition.

        The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
        successfully processed offset; future receives will start after it."""
        body = msg_stream_commit_in.model_dump(exclude_unset=True, by_alias=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/stream/commit",
            body=body,
            response_type=MsgStreamCommitOut,
        )

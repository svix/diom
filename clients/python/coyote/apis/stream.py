# This file is @generated

from .common import ApiBase
from ..models import (
    Ack,
    AckMsgRangeIn,
    AckMsgRangeOut,
    AckOut,
    AppendToStreamIn,
    AppendToStreamOut,
    DlqIn,
    DlqOut,
    FetchFromStreamIn,
    FetchFromStreamOut,
    RedriveIn,
    RedriveOut,
)


class StreamAsync(ApiBase):
    async def append(
        self,
        append_to_stream_in: AppendToStreamIn,
    ) -> AppendToStreamOut:
        """Appends messages to the stream."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/append",
            path_params={},
            json_body=append_to_stream_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AppendToStreamOut.model_validate(response.json())

    async def fetch(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.

        Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
        messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
        until the visibility timeout expires, or the messages are acked."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/fetch",
            path_params={},
            json_body=fetch_from_stream_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return FetchFromStreamOut.model_validate(response.json())

    async def fetch_locking(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, locking over the consumer group.

        This call prevents other consumers within the same consumer group from reading from the stream
        until either the visibility timeout expires, or the last message in the batch is acknowledged."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/fetch-locking",
            path_params={},
            json_body=fetch_from_stream_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return FetchFromStreamOut.model_validate(response.json())

    async def ack_range(
        self,
        ack_msg_range_in: AckMsgRangeIn,
    ) -> AckMsgRangeOut:
        """Acks the messages for the consumer group, allowing more messages to be consumed."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/ack-range",
            path_params={},
            json_body=ack_msg_range_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AckMsgRangeOut.model_validate(response.json())

    async def ack(
        self,
        ack: Ack,
    ) -> AckOut:
        """Acks a single message."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/ack",
            path_params={},
            json_body=ack.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return AckOut.model_validate(response.json())

    async def dlq(
        self,
        dlq_in: DlqIn,
    ) -> DlqOut:
        """Moves a message to the dead letter queue."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/dlq",
            path_params={},
            json_body=dlq_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DlqOut.model_validate(response.json())

    async def redrive(
        self,
        redrive_in: RedriveIn,
    ) -> RedriveOut:
        """Redrives messages from the dead letter queue back to the stream."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stream/redrive-dlq",
            path_params={},
            json_body=redrive_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return RedriveOut.model_validate(response.json())


class Stream(ApiBase):
    def append(
        self,
        append_to_stream_in: AppendToStreamIn,
    ) -> AppendToStreamOut:
        """Appends messages to the stream."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/append",
            path_params={},
            json_body=append_to_stream_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AppendToStreamOut.model_validate(response.json())

    def fetch(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.

        Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
        messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
        until the visibility timeout expires, or the messages are acked."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/fetch",
            path_params={},
            json_body=fetch_from_stream_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return FetchFromStreamOut.model_validate(response.json())

    def fetch_locking(
        self,
        fetch_from_stream_in: FetchFromStreamIn,
    ) -> FetchFromStreamOut:
        """Fetches messages from the stream, locking over the consumer group.

        This call prevents other consumers within the same consumer group from reading from the stream
        until either the visibility timeout expires, or the last message in the batch is acknowledged."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/fetch-locking",
            path_params={},
            json_body=fetch_from_stream_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return FetchFromStreamOut.model_validate(response.json())

    def ack_range(
        self,
        ack_msg_range_in: AckMsgRangeIn,
    ) -> AckMsgRangeOut:
        """Acks the messages for the consumer group, allowing more messages to be consumed."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/ack-range",
            path_params={},
            json_body=ack_msg_range_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AckMsgRangeOut.model_validate(response.json())

    def ack(
        self,
        ack: Ack,
    ) -> AckOut:
        """Acks a single message."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/ack",
            path_params={},
            json_body=ack.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return AckOut.model_validate(response.json())

    def dlq(
        self,
        dlq_in: DlqIn,
    ) -> DlqOut:
        """Moves a message to the dead letter queue."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/dlq",
            path_params={},
            json_body=dlq_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return DlqOut.model_validate(response.json())

    def redrive(
        self,
        redrive_in: RedriveIn,
    ) -> RedriveOut:
        """Redrives messages from the dead letter queue back to the stream."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stream/redrive-dlq",
            path_params={},
            json_body=redrive_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return RedriveOut.model_validate(response.json())

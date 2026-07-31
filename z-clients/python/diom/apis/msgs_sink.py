# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ListResponseSinkOut,
    SinkConfigureIn,
    SinkConfigureOut,
    SinkDeleteIn,
    SinkDeleteOut,
    SinkListIn,
)

from ..models.sink_list_in import _SinkListIn


class MsgsSinkAsync(ApiBase):
    async def configure(
        self,
        sink_configure_in: SinkConfigureIn,
    ) -> SinkConfigureOut:
        """Create or update a sink for a topic. Overwrites any existing sink with the same id."""
        body = sink_configure_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.sink.configure",
            body=body,
            response_type=SinkConfigureOut,
        )

    async def delete(
        self,
        sink_delete_in: SinkDeleteIn,
    ) -> SinkDeleteOut:
        """Delete a sink."""
        body = sink_delete_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.sink.delete",
            body=body,
            response_type=SinkDeleteOut,
        )

    async def list(
        self,
        topic: str,
        sink_list_in: SinkListIn = SinkListIn(),
    ) -> ListResponseSinkOut:
        """List sink configurations for a topic."""
        body = _SinkListIn(
            namespace=sink_list_in.namespace,
            topic=topic,
            limit=sink_list_in.limit,
            iterator=sink_list_in.iterator,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.sink.list",
            body=body,
            response_type=ListResponseSinkOut,
        )


class MsgsSink(ApiBase):
    def configure(
        self,
        sink_configure_in: SinkConfigureIn,
    ) -> SinkConfigureOut:
        """Create or update a sink for a topic. Overwrites any existing sink with the same id."""
        body = sink_configure_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.sink.configure",
            body=body,
            response_type=SinkConfigureOut,
        )

    def delete(
        self,
        sink_delete_in: SinkDeleteIn,
    ) -> SinkDeleteOut:
        """Delete a sink."""
        body = sink_delete_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.sink.delete",
            body=body,
            response_type=SinkDeleteOut,
        )

    def list(
        self,
        topic: str,
        sink_list_in: SinkListIn = SinkListIn(),
    ) -> ListResponseSinkOut:
        """List sink configurations for a topic."""
        body = _SinkListIn(
            namespace=sink_list_in.namespace,
            topic=topic,
            limit=sink_list_in.limit,
            iterator=sink_list_in.iterator,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.sink.list",
            body=body,
            response_type=ListResponseSinkOut,
        )

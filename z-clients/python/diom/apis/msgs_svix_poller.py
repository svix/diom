# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ListResponseSvixPollerOut,
    SvixPollerCreateIn,
    SvixPollerCreateOut,
    SvixPollerDeleteIn,
    SvixPollerDeleteOut,
    SvixPollerListIn,
)

from ..models.svix_poller_list_in import _SvixPollerListIn


class MsgsSvixPollerAsync(ApiBase):
    async def create(
        self,
        svix_poller_create_in: SvixPollerCreateIn,
    ) -> SvixPollerCreateOut:
        """Create a Svix poller configuration for a topic."""
        body = svix_poller_create_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.svix-poller.create",
            body=body,
            response_type=SvixPollerCreateOut,
        )

    async def delete(
        self,
        svix_poller_delete_in: SvixPollerDeleteIn,
    ) -> SvixPollerDeleteOut:
        """Delete a Svix poller configuration."""
        body = svix_poller_delete_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.svix-poller.delete",
            body=body,
            response_type=SvixPollerDeleteOut,
        )

    async def list(
        self,
        topic: str,
        svix_poller_list_in: SvixPollerListIn = SvixPollerListIn(),
    ) -> ListResponseSvixPollerOut:
        """List Svix poller configurations for a topic."""
        body = _SvixPollerListIn(
            namespace=svix_poller_list_in.namespace,
            topic=topic,
            limit=svix_poller_list_in.limit,
            iterator=svix_poller_list_in.iterator,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.svix-poller.list",
            body=body,
            response_type=ListResponseSvixPollerOut,
        )


class MsgsSvixPoller(ApiBase):
    def create(
        self,
        svix_poller_create_in: SvixPollerCreateIn,
    ) -> SvixPollerCreateOut:
        """Create a Svix poller configuration for a topic."""
        body = svix_poller_create_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.svix-poller.create",
            body=body,
            response_type=SvixPollerCreateOut,
        )

    def delete(
        self,
        svix_poller_delete_in: SvixPollerDeleteIn,
    ) -> SvixPollerDeleteOut:
        """Delete a Svix poller configuration."""
        body = svix_poller_delete_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.svix-poller.delete",
            body=body,
            response_type=SvixPollerDeleteOut,
        )

    def list(
        self,
        topic: str,
        svix_poller_list_in: SvixPollerListIn = SvixPollerListIn(),
    ) -> ListResponseSvixPollerOut:
        """List Svix poller configurations for a topic."""
        body = _SvixPollerListIn(
            namespace=svix_poller_list_in.namespace,
            topic=topic,
            limit=svix_poller_list_in.limit,
            iterator=svix_poller_list_in.iterator,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.svix-poller.list",
            body=body,
            response_type=ListResponseSvixPollerOut,
        )

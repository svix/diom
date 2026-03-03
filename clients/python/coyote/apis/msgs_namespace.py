# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgNamespaceCreateIn,
    MsgNamespaceCreateOut,
    MsgNamespaceGetIn,
    MsgNamespaceGetOut,
)


class MsgsNamespaceAsync(ApiBase):
    async def create(
        self,
        msg_namespace_create_in: MsgNamespaceCreateIn,
    ) -> MsgNamespaceCreateOut:
        """Creates or updates a msgs namespace with the given name."""
        body = msg_namespace_create_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/create",
            body=body,
            response_type=MsgNamespaceCreateOut,
        )

    async def get(
        self,
        msg_namespace_get_in: MsgNamespaceGetIn,
    ) -> MsgNamespaceGetOut:
        """Gets a msgs namespace by name."""
        body = msg_namespace_get_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/get",
            body=body,
            response_type=MsgNamespaceGetOut,
        )


class MsgsNamespace(ApiBase):
    def create(
        self,
        msg_namespace_create_in: MsgNamespaceCreateIn,
    ) -> MsgNamespaceCreateOut:
        """Creates or updates a msgs namespace with the given name."""
        body = msg_namespace_create_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/create",
            body=body,
            response_type=MsgNamespaceCreateOut,
        )

    def get(
        self,
        msg_namespace_get_in: MsgNamespaceGetIn,
    ) -> MsgNamespaceGetOut:
        """Gets a msgs namespace by name."""
        body = msg_namespace_get_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/get",
            body=body,
            response_type=MsgNamespaceGetOut,
        )

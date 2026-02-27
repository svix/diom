# This file is @generated

from .common import ApiBase
from ..models import (
    PublishIn,
    PublishOut,
)
from .msgs_namespace import (
    MsgsNamespace,
    MsgsNamespaceAsync,
)


class MsgsAsync(ApiBase):
    @property
    def namespace(self) -> MsgsNamespaceAsync:
        return MsgsNamespaceAsync(self._client)

    async def publish(
        self,
        publish_in: PublishIn,
    ) -> PublishOut:
        """Publishes messages to a topic within a namespace."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/publish",
            path_params={},
            json_body=publish_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return PublishOut.model_validate(response.json())


class Msgs(ApiBase):
    @property
    def namespace(self) -> MsgsNamespace:
        return MsgsNamespace(self._client)

    def publish(
        self,
        publish_in: PublishIn,
    ) -> PublishOut:
        """Publishes messages to a topic within a namespace."""
        response = self._request_sync(
            method="post",
            path="/api/v1/msgs/publish",
            path_params={},
            json_body=publish_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return PublishOut.model_validate(response.json())

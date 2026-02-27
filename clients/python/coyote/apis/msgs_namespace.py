# This file is @generated

from .common import ApiBase
from ..models import (
    CreateNamespaceIn,
    CreateNamespaceOut,
    GetNamespaceIn,
    GetNamespaceOut,
)


class MsgsNamespaceAsync(ApiBase):
    async def create(
        self,
        create_namespace_in: CreateNamespaceIn,
    ) -> CreateNamespaceOut:
        """Creates or updates a msgs namespace with the given name."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/create",
            path_params={},
            json_body=create_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CreateNamespaceOut.model_validate(response.json())

    async def get(
        self,
        get_namespace_in: GetNamespaceIn,
    ) -> GetNamespaceOut:
        """Gets a msgs namespace by name."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/msgs/namespace/get",
            path_params={},
            json_body=get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return GetNamespaceOut.model_validate(response.json())


class MsgsNamespace(ApiBase):
    def create(
        self,
        create_namespace_in: CreateNamespaceIn,
    ) -> CreateNamespaceOut:
        """Creates or updates a msgs namespace with the given name."""
        response = self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/create",
            path_params={},
            json_body=create_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CreateNamespaceOut.model_validate(response.json())

    def get(
        self,
        get_namespace_in: GetNamespaceIn,
    ) -> GetNamespaceOut:
        """Gets a msgs namespace by name."""
        response = self._request_sync(
            method="post",
            path="/api/v1/msgs/namespace/get",
            path_params={},
            json_body=get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return GetNamespaceOut.model_validate(response.json())

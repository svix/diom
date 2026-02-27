# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    KvDeleteIn,
    KvDeleteOut,
    KvGetIn,
    KvGetNamespaceIn,
    KvGetNamespaceOut,
    KvGetOut,
    KvSetIn,
    KvSetOut,
)


class KvAsync(ApiBase):
    async def set(
        self,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/kv/set",
            path_params={},
            json_body=kv_set_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return KvSetOut.model_validate(response.json())

    async def get(
        self,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get",
            path_params={},
            json_body=kv_get_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return KvGetOut.model_validate(response.json())

    async def get_namespace(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get-namespace",
            path_params={},
            json_body=kv_get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return KvGetNamespaceOut.model_validate(response.json())

    async def delete(
        self,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/kv/delete",
            path_params={},
            json_body=kv_delete_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return KvDeleteOut.model_validate(response.json())


class Kv(ApiBase):
    def set(
        self,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        response = self._request_sync(
            method="post",
            path="/api/v1/kv/set",
            path_params={},
            json_body=kv_set_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return KvSetOut.model_validate(response.json())

    def get(
        self,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        response = self._request_sync(
            method="post",
            path="/api/v1/kv/get",
            path_params={},
            json_body=kv_get_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return KvGetOut.model_validate(response.json())

    def get_namespace(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        response = self._request_sync(
            method="post",
            path="/api/v1/kv/get-namespace",
            path_params={},
            json_body=kv_get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return KvGetNamespaceOut.model_validate(response.json())

    def delete(
        self,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        response = self._request_sync(
            method="post",
            path="/api/v1/kv/delete",
            path_params={},
            json_body=kv_delete_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return KvDeleteOut.model_validate(response.json())

# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    KvDeleteIn,
    KvDeleteOut,
    KvGetIn,
    KvGetOut,
    KvSetIn,
    KvSetOut,
)
from .kv_namespace import (
    KvNamespace,
    KvNamespaceAsync,
)


class KvAsync(ApiBase):
    @property
    def namespace(self) -> KvNamespaceAsync:
        return KvNamespaceAsync(self._client)

    async def set(
        self,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        body = kv_set_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/set",
            body=body,
            response_type=KvSetOut,
        )

    async def get(
        self,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        body = kv_get_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/get",
            body=body,
            response_type=KvGetOut,
        )

    async def delete(
        self,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        body = kv_delete_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/kv/delete",
            body=body,
            response_type=KvDeleteOut,
        )


class Kv(ApiBase):
    @property
    def namespace(self) -> KvNamespace:
        return KvNamespace(self._client)

    def set(
        self,
        kv_set_in: KvSetIn,
    ) -> KvSetOut:
        """KV Set"""
        body = kv_set_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/set",
            body=body,
            response_type=KvSetOut,
        )

    def get(
        self,
        kv_get_in: KvGetIn,
    ) -> KvGetOut:
        """KV Get"""
        body = kv_get_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/get",
            body=body,
            response_type=KvGetOut,
        )

    def delete(
        self,
        kv_delete_in: KvDeleteIn,
    ) -> KvDeleteOut:
        """KV Delete"""
        body = kv_delete_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/kv/delete",
            body=body,
            response_type=KvDeleteOut,
        )

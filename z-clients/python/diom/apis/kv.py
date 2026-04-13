# This file is @generated

from ..internal.api_common import ApiBase, parse_response
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

from ..models.kv_set_in import _KvSetIn
from ..models.kv_get_in import _KvGetIn
from ..models.kv_delete_in import _KvDeleteIn


class KvAsync(ApiBase):
    @property
    def namespace(self) -> KvNamespaceAsync:
        return KvNamespaceAsync(self._client)

    async def set(
        self,
        key: str,
        value: bytes,
        kv_set_in: KvSetIn = KvSetIn(),
    ) -> KvSetOut:
        """KV Set"""
        body = _KvSetIn(
            namespace=kv_set_in.namespace,
            key=key,
            value=value,
            ttl=kv_set_in.ttl,
            behavior=kv_set_in.behavior,
            version=kv_set_in.version,
        ).model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.kv.set",
            body=body,
        )
        return parse_response(response, KvSetOut)

    async def get(
        self,
        key: str,
        kv_get_in: KvGetIn = KvGetIn(),
    ) -> KvGetOut:
        """KV Get"""
        body = _KvGetIn(
            namespace=kv_get_in.namespace,
            key=key,
            consistency=kv_get_in.consistency,
        ).model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.kv.get",
            body=body,
        )
        return parse_response(response, KvGetOut)

    async def delete(
        self,
        key: str,
        kv_delete_in: KvDeleteIn = KvDeleteIn(),
    ) -> KvDeleteOut:
        """KV Delete"""
        body = _KvDeleteIn(
            namespace=kv_delete_in.namespace,
            key=key,
            version=kv_delete_in.version,
        ).model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.kv.delete",
            body=body,
        )
        return parse_response(response, KvDeleteOut)


class Kv(ApiBase):
    @property
    def namespace(self) -> KvNamespace:
        return KvNamespace(self._client)

    def set(
        self,
        key: str,
        value: bytes,
        kv_set_in: KvSetIn = KvSetIn(),
    ) -> KvSetOut:
        """KV Set"""
        body = _KvSetIn(
            namespace=kv_set_in.namespace,
            key=key,
            value=value,
            ttl=kv_set_in.ttl,
            behavior=kv_set_in.behavior,
            version=kv_set_in.version,
        ).model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.kv.set",
            body=body,
        )
        return parse_response(response, KvSetOut)

    def get(
        self,
        key: str,
        kv_get_in: KvGetIn = KvGetIn(),
    ) -> KvGetOut:
        """KV Get"""
        body = _KvGetIn(
            namespace=kv_get_in.namespace,
            key=key,
            consistency=kv_get_in.consistency,
        ).model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.kv.get",
            body=body,
        )
        return parse_response(response, KvGetOut)

    def delete(
        self,
        key: str,
        kv_delete_in: KvDeleteIn = KvDeleteIn(),
    ) -> KvDeleteOut:
        """KV Delete"""
        body = _KvDeleteIn(
            namespace=kv_delete_in.namespace,
            key=key,
            version=kv_delete_in.version,
        ).model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.kv.delete",
            body=body,
        )
        return parse_response(response, KvDeleteOut)
